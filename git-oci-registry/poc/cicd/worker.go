package main

import (
	"log"
	"time"

	"go.temporal.io/sdk/client"
	"go.temporal.io/sdk/worker"
)

func runWorker() {
	temporalAddr := getEnv("TEMPORAL_ADDRESS", "temporal:7233")

	var c client.Client
	var err error
	for i := 0; i < 30; i++ {
		c, err = client.Dial(client.Options{HostPort: temporalAddr})
		if err == nil {
			break
		}
		log.Printf("Waiting for Temporal (%d/30): %v", i+1, err)
		time.Sleep(2 * time.Second)
	}
	if err != nil {
		log.Fatalf("Failed to connect to Temporal after retries: %v", err)
	}
	defer c.Close()

	w := worker.New(c, TaskQueue, worker.Options{})
	w.RegisterWorkflow(RepackageWorkflow)
	w.RegisterActivity(&Activities{})

	log.Println("Starting Temporal worker on queue:", TaskQueue)
	if err := w.Run(worker.InterruptCh()); err != nil {
		log.Fatalf("Worker failed: %v", err)
	}
}
