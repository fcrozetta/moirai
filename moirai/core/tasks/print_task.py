import logging

from moirai.core.task_parameter import TaskEdge, TaskParameterIO
from ..base_task import BaseTask, TaskStatus
from moirai.core.task_registry import task_registry


class PrintTask(BaseTask):
    inputs: list[TaskParameterIO] = [
        TaskParameterIO(
            variable_name="input_string",
            name="String to print",
            expected_type="string",
            value="",
            role="input",
        )
    ]
    outputs = []
    parameters = []
    edges: list[TaskEdge] = [
        TaskEdge(condition="target"),
        TaskEdge(condition="success"),
    ]

    def __init__(self, name: str = "PrintTask", **kwargs):
        super().__init__(name=name, **kwargs)

    def execute(self):
        """Print the input message to the console."""
        try:
            # Ensure the input is a string
            input_string = self.get_input("input_string")
            if not isinstance(input_string.value, str):
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
