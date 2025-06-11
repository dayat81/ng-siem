import logging
import json
import os

from flask import Flask, request

app = Flask(__name__)

port = int(os.environ.get('PORT', 8080))

# Configure logging
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')

@app.route('/', methods=['GET', 'POST'])
def crowdstrike_edr_mock():
    """This Cloud Run app simulates integration with CrowdStrike EDR.
    It receives alerts from the SIEM and logs them to Cloud Logging.
    It also simulates triggering responses based on the alerts.
    """

    if request.method == 'GET':
        return "OK", 200

    data = request.get_json()
    logging.info(f"Received alert: {data}")

    try:
        # Simulate CrowdStrike data structure
        event = data.get('event', {})
        event_type = event.get('eventType', 'unknown')
        detection_name = event.get('DetectionName', 'unknown')
        device_id = event.get('DeviceID', 'unknown')

        logging.info(f"Event Type: {event_type}, Detection Name: {detection_name}, Device ID: {device_id}")

        # Simulate triggering responses based on the alert type
        if event_type == 'malware':
            logging.info(f"Simulating response: Isolating host {device_id}")
        elif event_type == 'suspicious_activity':
            logging.info(f"Simulating response: Investigating user activity on {device_id}")
        else:
            logging.info("Simulating response: No action required")

        return "OK", 200

    except Exception as e:
        logging.error(f"Error processing alert: {e}")
        return "Error", 500

if __name__ == '__main__':
    app.run(debug=True, host='0.0.0.0', port=port)
