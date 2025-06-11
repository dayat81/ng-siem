# GCP SIEM Architecture

This project contains the design and implementation for a comprehensive, scalable, and cost-effective AI-powered Security Information and Event Management (SIEM) architecture on Google Cloud's serverless infrastructure.

## Directory Structure

*   `docs/`: Contains the architecture design and implementation plan.
*   `config/`: Contains configuration files for BigQuery schemas, Cloud Storage lifecycle policies, and bucket metadata.
*   `src/`: Contains the source code for the SIEM components.
    *   `dataflow/`: Contains the Dataflow pipeline for processing alerts.
    *   `crowdstrike_edr_mock.py`: A Flask application that simulates integration with CrowdStrike EDR, intended for deployment on Cloud Run.

## Data Residency

All data is stored and processed within the Indonesia region (`asia-southeast2`) to comply with data residency requirements.
