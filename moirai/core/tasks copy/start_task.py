import logging
from ..base_task import BaseTask, TaskStatus
from moirai.core.task_registry import task_registry


class StartTask(BaseTask):
    def __init__(
        self,
        task_id: str,
        name: str = "Start",
        timeout: int = 5,
        debug: bool = False,
        log_level: int = logging.INFO,
    ):
        super().__init__(name=name, task_id=task_id, timeout=timeout)
        self.debug = debug
        self.log_level = log_level
        self._configure_logging()

    def _configure_logging(self):
        """Set up logging configuration based on debug mode and log level."""
        logging.basicConfig(level=self.log_level)
        if self.debug:
            logging.getLogger().setLevel(logging.DEBUG)
        logging.info("Logging initialized for StartTask.")
        if self.debug:
            logging.debug("Debug mode is enabled.")

    def execute(self):
        """Execute the start task, initializing any required settings for the engine."""
        try:
            # Additional initialization logic can go here if needed
            logging.info(f"{self.name} task is initializing the engine.")
            self.status = TaskStatus.SUCCESS
        except Exception as e:
            logging.error(f"Failed to initialize {self.name} task: {e}")
            self.status = TaskStatus.EXECUTION_ERROR
            self.error_code = 3  # Example error code for execution error


task_registry.register("StartTask", StartTask)
