import logging
from .base_task import BaseTask, TaskStatus
from moirai.core.task_registry import task_registry


class EndTask(BaseTask):
    def __init__(self, name: str = "EndTask", timeout: int = None):
        super().__init__(name=name, timeout=timeout)
        self.inputs = {}  # No specific inputs required

    def execute(self):
        """Indicate the end of the task flow."""
        try:
            # Mark the task as successful to indicate the end
            logging.info(f"{self.name} task executed successfully. End of job.")
            self.status = TaskStatus.SUCCESS
        except Exception as e:
            logging.error(f"Error executing {self.name} task: {e}")
            self.status = TaskStatus.EXECUTION_ERROR
            self.error_code = 3  # Example error code for execution error


task_registry.register("EndTask", EndTask)
