import logging
from .base_task import BaseTask, TaskStatus
from moirai.core.task_registry import task_registry


class StringTask(BaseTask):
    def __init__(self, name: str = "StringTask", timeout: int = 5, join_str: str = ""):
        super().__init__(name=name, timeout=timeout)
        self.join_str = join_str  # The string to use for joining inputs
        self.inputs = []  # Ordered list of strings to be joined
        self.outputs = {"result": ""}  # Output dictionary with a single string output

    def execute(self):
        """Concatenate input strings with the join string."""
        try:
            # Check that inputs are valid (should be a list of strings)
            if not all(isinstance(item, str) for item in self.inputs):
                self.status = TaskStatus.VALIDATION_ERROR
                self.error_code = 2  # Example error code for validation error
                raise ValueError("All inputs must be strings")

            # Join the input strings with the specified join string
            if self.inputs:
                self.outputs["result"] = self.join_str.join(self.inputs)
            else:
                self.outputs["result"] = ""

            logging.info(
                f"{self.name} task executed successfully with output: {self.outputs['result']}"
            )
            self.status = TaskStatus.SUCCESS
        except Exception as e:
            logging.error(f"Error executing {self.name} task: {e}")
            self.status = TaskStatus.EXECUTION_ERROR
            self.error_code = 3  # Example error code for execution error


task_registry.register("StringTask", StringTask)
