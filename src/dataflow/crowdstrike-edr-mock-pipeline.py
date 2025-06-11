import apache_beam as beam
from apache_beam.options.pipeline_options import PipelineOptions
import logging
import json

def process_alert(alert):
    """
    This function simulates integration with CrowdStrike EDR.
    It receives alerts from the SIEM and logs them to Cloud Logging.
    It also simulates triggering responses based on the alerts.
    """

    logging.info(f"Received alert: {alert}")

    try:
        alert_data = json.loads(alert)
        alert_type = alert_data.get('alert_type', 'unknown')

        # Simulate triggering responses based on the alert type
        if alert_type == 'malware':
            logging.info("Simulating response: Isolating host")
        elif alert_type == 'suspicious_activity':
            logging.info("Simulating response: Investigating user activity")
        else:
            logging.info("Simulating response: No action required")

    except json.JSONDecodeError:
        logging.error("Failed to decode alert data as JSON")


def run(argv=None):
    """
    This function defines the Dataflow pipeline.
    """

    pipeline_options = PipelineOptions(
        argv,
        save_main_session=True,
        streaming=True
    )

    with beam.Pipeline(options=pipeline_options) as pipeline:
        # Read alerts from Pub/Sub topic
        alerts = pipeline | "ReadFromPubSub" >> beam.io.ReadFromPubSub(topic="projects/northern-center-462508-k6/topics/gcp-siem-topic").with_output_types(bytes)

        # Process each alert
        alerts | "ProcessAlert" >> beam.Map(lambda x: x.decode('utf-8')) | "LogAlert" >> beam.Map(process_alert)


if __name__ == "__main__":
    logging.getLogger().setLevel(logging.INFO)
    run()