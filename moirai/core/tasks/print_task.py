import logging
from .base_task import BaseTask, TaskStatus
from moirai.core.task_registry import task_registry


class PrintTask(BaseTask):
    def __init__(self, name: str = "PrintTask", timeout: int = None):
        super().__init__(name=name, timeout=timeout)
        self.inputs = {"message": ""}  # Expected input: a single string message

    def execute(self):
        """Print the input message to the console."""
        try:
            # Ensure the input is a string
            message = self.inputs.get("message")
            if not isinstance(message, str):
                self.status = TaskStatus.VALIDATION_ERROR
                self.error_code = 2  # Example error code for validation error
                raise ValueError("Input 'message' must be a string")

            # Print the message to the console
            print(message)
            logging.info(
                f"{self.name} task executed successfully. Printed message: {message}"
            )
            self.status = TaskStatus.SUCCESS
        except Exception as e:
            logging.error(f"Error executing {self.name} task: {e}")
            self.status = TaskStatus.EXECUTION_ERROR
            self.error_code = 3  # Example error code for execution error


task_registry.register("PrintTask", PrintTask)
