package main

import (
	"context"
	"fmt"
	"log"
	"net/http"
	"os"
	"strconv"

	"cloud.google.com/go/pubsub"
)

func main() {
	portStr := os.Getenv("PORT")
	if portStr == "" {
		portStr = "8080"
	}
	port, err := strconv.Atoi(portStr)
	if err != nil {
		log.Fatalf("Invalid port number: %v", err)
	}

	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprintln(w, "Hello, Cloud Run!")
	})

	log.Printf("Listening on port: %d", port)
	log.Fatal(http.ListenAndServe(fmt.Sprintf(":%d", port), nil))

	projectID := os.Getenv("GOOGLE_CLOUD_PROJECT")
	log.Printf("GOOGLE_CLOUD_PROJECT: %s", projectID)
	if projectID == "" {
		log.Fatalf("GOOGLE_CLOUD_PROJECT environment variable must be set.")
	}

	topicID := os.Getenv("PUBSUB_TOPIC_ID")
	log.Printf("PUBSUB_TOPIC_ID: %s", topicID)
	if topicID == "" {
		log.Fatalf("PUBSUB_TOPIC_ID environment variable must be set.")
	}

	subscriptionID := os.Getenv("PUBSUB_SUBSCRIPTION_ID")
	log.Printf("PUBSUB_SUBSCRIPTION_ID: %s", subscriptionID)
	if subscriptionID == "" {
		log.Fatalf("PUBSUB_SUBSCRIPTION_ID environment variable must be set.")
	}

	ctx := context.Background()

	client, err := pubsub.NewClient(ctx, projectID)
	if err != nil {
		log.Fatalf("Failed to create client: %v. Project ID: %s", err, projectID)
	}
	defer client.Close()

	sub := client.Subscription(subscriptionID)

	err = sub.Receive(ctx, func(ctx context.Context, msg *pubsub.Message) {
		fmt.Printf("Got message: %q\n", string(msg.Data))
		msg.Ack()
	})
	if err != nil {
		log.Fatalf("Failed to receive messages: %v", err)
	}
}
