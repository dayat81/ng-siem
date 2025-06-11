import os
import time
from google.cloud import pubsub_v1

# Configuration (These should ideally be read from environment variables or a config file)
PROJECT_ID = "northern-center-462508-k6"  # Replace with your GCP project ID
TOPIC_ID = "gcp-siem-topic"  # Replace with your Pub/Sub topic ID
LOG_FILES = [ "/var/log/openvswitch/ovs-vswitchd.log"]  # Log files to monitor

def publish_to_pubsub(project_id, topic_id, message):
    """Publishes a message to a Pub/Sub topic."""
    publisher = pubsub_v1.PublisherClient()
    topic_path = publisher.topic_path(project_id, topic_id)
    future = publisher.publish(topic_path, message.encode("utf-8"))
    try:
        message_id = future.result()
        print(f"Published message {message_id} to {topic_path}")
        print(f"Message sent to Google Cloud Pub/Sub: {message_id}")
    except Exception as e:
        print(f"Error publishing message: {e}")

def read_logs_and_publish():
    """Reads log files and publishes their content to Pub/Sub."""
    print("Starting log collection and publishing...")
    while True:
        for log_file in LOG_FILES:
            print(f"Monitoring log file: {log_file}")
            try:
                if not os.path.exists(log_file):
                    print(f"File does not exist: {log_file}")
                    continue
                if not os.access(log_file, os.R_OK):
                    print(f"No read access to file: {log_file}")
                    continue
                with open(log_file, "r") as f:
                    # Move to the end of the file to start reading new entries
                    f.seek(0, os.SEEK_END)
                    while True:
                        line = f.readline()
                        if not line:
                            time.sleep(1)  # Wait for new log entries
                            continue
                        log_message = f"{log_file}: {line.strip()}"
                        print(f"Publishing: {log_message}")
                        publish_to_pubsub(PROJECT_ID, TOPIC_ID, log_message)
            except FileNotFoundError:
                print(f"File not found: {log_file}")
            except Exception as e:
                print(f"Error reading/publishing from {log_file}: {e}")
        time.sleep(60)  # Check for new logs every 60 seconds

if __name__ == "__main__":
    read_logs_and_publish()
