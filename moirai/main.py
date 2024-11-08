import logging
from core.engine import Engine


def main():
    # Configure logging for detailed output
    logging.basicConfig(
        level=logging.DEBUG, format="%(asctime)s - %(levelname)s - %(message)s"
    )

    # Path to the job configuration file
    # job_file = "sample_jobs/00.helloWorld.json"  # Replace with the actual path to your JSON file
    job_file = (
        "sample_jobs/sample.json"  # Replace with the actual path to your JSON file
    )

    # Initialize the engine with the job file
    logging.info("Initializing Engine.")
    engine = Engine()

    # Load the job configuration
    # engine.load_job()

    # Run the engine
    # logging.info("Starting engine.")
    engine.add_job_from_file(job_file)
    engine.run()


if __name__ == "__main__":
    main()
