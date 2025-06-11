package main

import (
	"context"
	"fmt"
	"log"
	"os"

	"cloud.google.com/go/pubsub"
)

func main() {
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
