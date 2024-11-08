import logging
from typing import List, Optional
from moirai.core.task_status import TaskStatus
from moirai.core.task_parameter import (
    TaskEdge,
    TaskParameterInternal,
    TaskParameterInternalGroup,
    TaskParameterIO,
)


class BaseTask:
    def __init__(
        self,
        task_id: str,
        name: str,
        parameters: list[dict],
        inputs: list[dict],
        outputs: list[dict],
        edges: list[dict],
    ):
        self.task_id = task_id  # Task identifier as a string
        self.name = name  # Task name
        self.status = TaskStatus.PENDING
        self.error_code = None
        self._initialize_inputs(inputs)
        self._initialize_outputs(outputs)
        self._initialize_parameters(parameters)
        self._initialize_edges(edges)

    def _initialize_parameters(self, parameters: list[dict]):
        pass

    def _initialize_inputs(self, inputs: list[dict]):
        for i in inputs:
            if task_input := self.get_input(i["variable_name"]):
                task_input.value = i["value"]

    def _initialize_outputs(self, outputs: list[dict]):
        for o in outputs:
            if task_output := self.get_output(o["variable_name"]):
                task_output.value = o["value"]

    def _initialize_edges(self, edges: list[dict]):
        for e in edges:
            if task_edge := self.get_edge(e["condition"]):
                task_edge.target = e["target"]

    def get_edge(self, condition: str):
        for obj in self.edges:
            if hasattr(obj, "condition") and getattr(obj, "condition") == condition:
                return obj
        return None

    def get_input(self, variable_name: str) -> TaskParameterIO:
        for obj in self.inputs:
            if (
                hasattr(obj, "variable_name")
                and getattr(obj, "variable_name") == variable_name
            ):
                return obj
        return None

    def get_output(self, variable_name: str) -> TaskParameterIO:
        for obj in self.outputs:
            if (
                hasattr(obj, "variable_name")
                and getattr(obj, "variable_name") == variable_name
            ):
                return obj
        return None

    def get_parameter_group(self, group_id: str) -> TaskParameterInternalGroup:
        for obj in self.outputs:
            if getattr(obj, "grouop_id") == group_id:
                return obj
        return None

    def getParameter(self, group_id: str, variable_name: str):
        if group := self.get_parameter_group(group_id):
            for p in group.parameters:
                if (
                    hasattr(p, "variable_name")
                    and getattr(p, "variable_name") == variable_name
                ):
                    return p
        return None

    def pre_execute(self):
        """Prepare the task for execution."""
        pass

    def execute(self):
        """Main execution logic for the task."""
        raise NotImplementedError(
            "The execute method must be implemented in subclasses."
        )

    def post_execute(self):
        """Clean up resources after execution."""
        pass

    def run(self):
        """Run the task with pre- and post-execution."""
        self.status = TaskStatus.RUNNING
        try:
            self.pre_execute()
            self.execute()
            for output in self.outputs:
                output._validate_type()  # Ensure output types are validated after execution
            self.status = TaskStatus.SUCCESS
        except Exception as e:
            logging.error(f"Error executing {self.name} task: {e}")
            self.status = TaskStatus.EXECUTION_ERROR
            self.error_code = 3
        finally:
            self.post_execute()
