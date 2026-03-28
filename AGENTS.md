# Playground

This repository contains small experiments. Each top-level subdirectory is a
separate experiment and should stay independent from the others.

## Creating a New Experiment

- Create a new top-level directory with a short, descriptive name.
- Keep all code, data, scripts, and documentation for that experiment inside
  its own directory.
- Start with a clear objective and one primary metric. Record the metric name,
  unit, and whether higher or lower is better.
- Create `notes.md` early and append what you tried, what happened, key
  metrics, failed attempts, and decisions as you work.
- Create `README.md` in the experiment directory as the final report. It should
  explain the question, setup, results, and how to reproduce the experiment.
- If the experiment is iterative or benchmark-driven, also create a small
  session file such as `program.md` or `experiment.md` that captures the
  objective, constraints, files in scope, and stop conditions.
- If repeated runs matter, add a single reproducible runner such as `make`,
  `just`.
- If the experiment has multiple runs, keep an append-only log such as
  `results.jsonl` or `results.tsv` so a fresh agent can resume from the folder
  alone.

## Working Rules

- Do not edit unrelated experiment directories unless the task explicitly
  requires it.
- Do not define a shared repository-level development shell.
- Always use Nix to manage development tools, define `flake.nix` and `.envrc`
  inside that experiment directory.
- Keep the writable scope small. If the experiment targets another repo, record
  the files in scope and keep only patches, diffs, or minimal extracted code in
  this repo.
- Establish a baseline before changing things. Try an idea, measure it, keep
  what works, and discard regressions.
- Prefer simpler approaches when results are comparable.
- Do not create repo-wide summary files such as `SUMMARY.md` or `_summary.md`.
- Do not vendor large external repositories into this repo.
- If you need external code for reference, keep only minimal excerpts,
  patches, diffs, or concise notes about what changed.
- When saving results here, prefer new files you created or diffs against
  external code, not full copies of fetched repositories.
- Keep generated artifacts small and necessary. Large binaries do not belong in
  this repo.
- Prefer small, self-contained artifacts that make the experiment easy to
  understand and reproduce.

## Resumability

- A fresh agent should be able to open the experiment folder and continue from
  `README.md`, `notes.md`, and any session or results files without additional
  context.
- Keep the experiment folder human-readable. Favor plain text logs and scripts
  over opaque tooling when both are sufficient.
