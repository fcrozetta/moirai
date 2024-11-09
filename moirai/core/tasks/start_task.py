import logging, time

from moirai.core.task_parameter import (
    TaskEdge,
    TaskParameterIO,
    TaskParameterInternalGroup,
)
from moirai.core.base_task import BaseTask, TaskStatus
from moirai.core.task_registry import task_registry


class StartTask(BaseTask):
    inputs: list[TaskParameterIO] = []
    outputs: list[TaskParameterIO] = [
        TaskParameterIO("output_time", "Start Time", 0.0, "float", "output")
    ]
    parameters: list[TaskParameterInternalGroup] = []
    edges: list[TaskEdge] = [TaskEdge(condition="success")]

    def __init__(self, name: str = "StartTask", **kwargs):
        super().__init__(name=name, **kwargs)

    def execute(self):
        """Indicate the end of the task flow."""
        start_time = self.get_output("output_time")
        start_time.value = time.perf_counter()


task_registry.register("StartTask", StartTask)
