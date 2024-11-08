import logging
from core.engine import Engine


def main():
    # Configure logging for detailed output
    logging.basicConfig(
        level=logging.DEBUG, format="%(asctime)s - %(levelname)s - %(message)s"
    )

    # Path to the job configuration file
    job_file = "sample_jobs/00.helloWorld.json"  # Replace with the actual path to your JSON file

    # Initialize the engine with the job file
    engine = Engine(job_file)

    # Load the job configuration
    logging.info("Loading job configuration.")
    engine.load_job()

    # Run the engine
    logging.info("Starting engine.")
    engine.run()
    logging.info("Engine execution completed.")


if __name__ == "__main__":
    main()
