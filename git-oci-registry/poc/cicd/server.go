package main

import (
	"context"
	"encoding/json"
	"fmt"
	"log"
	"net/http"

	"go.temporal.io/sdk/client"
)

// TriggerRequest is the payload for the /trigger endpoint.
type TriggerRequest struct {
	Registry   string `json:"registry"`
	Repository string `json:"repository"`
	Tag        string `json:"tag"`
}

func runServer() {
	temporalAddr := getEnv("TEMPORAL_ADDRESS", "temporal:7233")
	c, err := client.Dial(client.Options{HostPort: temporalAddr})
	if err != nil {
		log.Fatalf("Failed to connect to Temporal: %v", err)
	}
	defer c.Close()

	http.HandleFunc("/trigger", func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
			return
		}

		var req TriggerRequest
		if err := json.NewDecoder(r.Body).Decode(&req); err != nil || req.Repository == "" {
			req = TriggerRequest{
				Registry:   getEnv("OCI_REGISTRY", "zot:5000"),
				Repository: getEnv("OCI_REPOSITORY", "demo/myapp"),
				Tag:        "main",
			}
		}

		workflowOptions := client.StartWorkflowOptions{
			TaskQueue: TaskQueue,
		}
		we, err := c.ExecuteWorkflow(
			context.Background(),
			workflowOptions,
			RepackageWorkflow,
			RepackageInput{
				SourceRegistry:   req.Registry,
				SourceRepository: req.Repository,
				SourceTag:        req.Tag,
			},
		)
		if err != nil {
			http.Error(w, fmt.Sprintf("Failed to start workflow: %v", err), http.StatusInternalServerError)
			return
		}

		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(map[string]string{
			"workflowID": we.GetID(),
			"runID":      we.GetRunID(),
		})
		log.Printf("Started workflow %s (run %s)", we.GetID(), we.GetRunID())
	})

	http.HandleFunc("/health", func(w http.ResponseWriter, _ *http.Request) {
		w.WriteHeader(http.StatusOK)
		w.Write([]byte("ok"))
	})

	addr := getEnv("LISTEN_ADDR", ":8080")
	log.Printf("Webhook server listening on %s", addr)
	log.Fatal(http.ListenAndServe(addr, nil))
}
