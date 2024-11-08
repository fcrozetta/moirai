import logging, time

from moirai.core.task_parameter import (
    TaskEdge,
    TaskParameterIO,
    TaskParameterInternalGroup,
)
from ..base_task import BaseTask, TaskStatus
from moirai.core.task_registry import task_registry


class EndTask(BaseTask):
    inputs: list[TaskParameterIO] = [
        TaskParameterIO("input_status", "Status Code", 0, "int", role="input"),
        TaskParameterIO("input_time", "Execution time", 0.0, "float", role="input"),
    ]
    outputs: list[TaskParameterIO] = []
    parameters: list[TaskParameterInternalGroup] = []
    edges: list[TaskEdge] = [TaskEdge(condition="target")]

    def __init__(self, name: str = "EndTask", **kwargs):
        super().__init__(name=name, **kwargs)

    def execute(self):
        """Indicate the end of the task flow."""
        start_time = self.get_input("input_time")
        end_time = time.perf_counter()
        logging.info(f"Execution Time: {end_time - start_time}")


task_registry.register("EndTask", EndTask)
