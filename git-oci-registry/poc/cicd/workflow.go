package main

import (
	"context"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"time"

	"go.temporal.io/sdk/activity"
	"go.temporal.io/sdk/temporal"
	"go.temporal.io/sdk/workflow"
)

const TaskQueue = "repackage"

// RepackageInput describes what to pull from the OCI registry.
type RepackageInput struct {
	SourceRegistry   string
	SourceRepository string
	SourceTag        string
}

// RepackageOutput describes the resulting Flux OCI artifact.
type RepackageOutput struct {
	FluxArtifactURL string
	Digest          string
}

// RepackageWorkflow clones a git repo from an OCI registry (gnoci format)
// and re-publishes the Kubernetes manifests as a Flux OCI artifact.
func RepackageWorkflow(ctx workflow.Context, input RepackageInput) (*RepackageOutput, error) {
	logger := workflow.GetLogger(ctx)
	logger.Info("Starting repackage workflow", "input", input)

	activityOptions := workflow.ActivityOptions{
		StartToCloseTimeout: 5 * time.Minute,
		RetryPolicy: &temporal.RetryPolicy{
			MaximumAttempts: 3,
		},
	}
	ctx = workflow.WithActivityOptions(ctx, activityOptions)

	var a *Activities

	// Step 1: Clone the repo from the OCI registry via gnoci
	var cloneResult CloneResult
	err := workflow.ExecuteActivity(ctx, a.CloneFromOCI, input).Get(ctx, &cloneResult)
	if err != nil {
		return nil, fmt.Errorf("clone failed: %w", err)
	}

	// Step 2: Push the manifests as a Flux OCI artifact
	var output RepackageOutput
	err = workflow.ExecuteActivity(ctx, a.PushFluxArtifact, cloneResult, input).Get(ctx, &output)
	if err != nil {
		return nil, fmt.Errorf("push flux artifact failed: %w", err)
	}

	logger.Info("Repackage complete", "artifactURL", output.FluxArtifactURL)
	return &output, nil
}

// Activities holds the activity implementations.
type Activities struct{}

// CloneResult holds the path to the cloned repo and the HEAD revision.
type CloneResult struct {
	Dir      string
	Revision string
}

// CloneFromOCI clones a git repository from an OCI registry using gnoci.
func (a *Activities) CloneFromOCI(ctx context.Context, input RepackageInput) (*CloneResult, error) {
	logger := activity.GetLogger(ctx)

	cloneDir, err := os.MkdirTemp("", "gnoci-clone-*")
	if err != nil {
		return nil, fmt.Errorf("create temp dir: %w", err)
	}

	ociURL := fmt.Sprintf("oci://%s/%s:%s", input.SourceRegistry, input.SourceRepository, input.SourceTag)
	logger.Info("Cloning from OCI", "url", ociURL, "dir", cloneDir)

	cmd := exec.CommandContext(ctx, "git", "clone", ociURL, cloneDir)
	cmd.Env = append(os.Environ(), "GIT_TERMINAL_PROMPT=0")
	out, err := cmd.CombinedOutput()
	if err != nil {
		os.RemoveAll(cloneDir)
		return nil, fmt.Errorf("git clone failed: %s: %w", string(out), err)
	}
	logger.Info("Clone succeeded", "output", string(out))

	// Get HEAD revision
	revision := fmt.Sprintf("oci-%d", time.Now().Unix())
	revCmd := exec.CommandContext(ctx, "git", "-C", cloneDir, "rev-parse", "HEAD")
	if revOut, revErr := revCmd.Output(); revErr == nil {
		revision = strings.TrimSpace(string(revOut))
	}

	return &CloneResult{Dir: cloneDir, Revision: revision}, nil
}

// PushFluxArtifact takes the cloned repo and pushes its manifests as a
// Flux-compatible OCI artifact.
func (a *Activities) PushFluxArtifact(ctx context.Context, clone CloneResult, input RepackageInput) (*RepackageOutput, error) {
	logger := activity.GetLogger(ctx)
	defer os.RemoveAll(clone.Dir)

	// Find the manifests directory — try common locations, fall back to repo root
	manifestsDir := clone.Dir
	for _, candidate := range []string{"manifests", "deploy", "k8s", "kubernetes"} {
		p := filepath.Join(clone.Dir, candidate)
		if info, err := os.Stat(p); err == nil && info.IsDir() {
			manifestsDir = p
			break
		}
	}

	fluxRepo := fmt.Sprintf("%s/%s-manifests", input.SourceRegistry, input.SourceRepository)
	fluxURL := fmt.Sprintf("oci://%s:latest", fluxRepo)

	logger.Info("Pushing Flux artifact",
		"url", fluxURL,
		"path", manifestsDir,
		"revision", clone.Revision,
	)

	cmd := exec.CommandContext(ctx, "flux", "push", "artifact",
		fluxURL,
		"--path", manifestsDir,
		"--source", fmt.Sprintf("oci://%s/%s", input.SourceRegistry, input.SourceRepository),
		"--revision", fmt.Sprintf("%s@sha1:%s", input.SourceTag, clone.Revision),
		"--insecure-registry",
	)
	out, err := cmd.CombinedOutput()
	if err != nil {
		return nil, fmt.Errorf("flux push artifact failed: %s: %w", string(out), err)
	}
	logger.Info("Flux push succeeded", "output", string(out))

	return &RepackageOutput{FluxArtifactURL: fluxURL}, nil
}
