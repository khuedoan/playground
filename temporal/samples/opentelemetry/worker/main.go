package main

import (
	"context"
	"log"

	otelworkflow "github.com/temporalio/samples-go/opentelemetry"
	"go.temporal.io/sdk/client"
	"go.temporal.io/sdk/contrib/opentelemetry"
	"go.temporal.io/sdk/interceptor"
	"go.temporal.io/sdk/worker"
)

func main() {
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	tp, err := otelworkflow.InitializeGlobalTracerProvider()
	if err != nil {
		log.Fatalln("Unable to create a global trace provider", err)
	}
	
	defer func() {
		if err := tp.Shutdown(ctx); err != nil {
			log.Println("Error shutting down trace provider:", err)
		}
	}()

	tracingInterceptor, err := opentelemetry.NewTracingInterceptor(opentelemetry.TracerOptions{})
	if err != nil {
		log.Fatalln("Unable to create interceptor", err)
	}

	options := client.Options{
		Interceptors: []interceptor.ClientInterceptor{tracingInterceptor},
	}

	// The client and worker are heavyweight objects that should be created once per process.
	c, err := client.Dial(options)
	if err != nil {
		log.Fatalln("Unable to create client", err)
	}
	defer c.Close()

	w := worker.New(c, "otel", worker.Options{})

	w.RegisterWorkflow(otelworkflow.Workflow)
	w.RegisterActivity(otelworkflow.Activity)

	err = w.Run(worker.InterruptCh())
	if err != nil {
		log.Fatalln("Worker run failed", err)
	}
}
