import logging
from moirai.core.task_registry import task_registry
from moirai.core.base_task import BaseTask
from moirai.core.task_status import TaskStatus
from moirai.core.task_parameter import (
    TaskEdge,
    TaskParameterIO,
    TaskParameterInternal,
    TaskParameterInternalGroup,
)


class StringTask(BaseTask):
    inputs = [TaskParameterIO("input_strings", "Strings", [], "list<string>", "input")]
    outputs = [TaskParameterIO("output_result", "Output", "", "string", "output")]
    parameters = [
        TaskParameterInternalGroup(
            group_id="fixed_options",
            title="Fixed string Config",
            description="used when no inputs",
            parameters=[
                TaskParameterInternal(
                    variable_name="fixed_string",
                    label="String",
                    param_type="string",
                    default_value="",
                )
            ],
        ),
        TaskParameterInternalGroup(
            group_id="dynamic_options",
            title="Input Options",
            description="Used to configure Inputs",
            parameters=[
                TaskParameterInternal(
                    variable_name="str_join",
                    label="Join String",
                    param_type="string",
                    default_value=", ",
                )
            ],
        ),
    ]
    edges: list[TaskEdge] = [
        # TaskEdge(condition="success"),
        # TaskEdge(condition="failure"),
    ]

    def __init__(self, name: str = "StringTask", **kwargs):
        super().__init__(name=name, **kwargs)

    def execute(self):
        """Concatenate input strings with the join string."""
        result = self.get_output("result")
        if strs := self.get_input("strings"):
            str_join: str = self.get_parameter("dynamic_parameters", "str_join")
            result = str_join.join(strs)
        else:
            fixed_string: str = self.get_parameter("fixed_parameter", "fixed_string")
            result = fixed_string
        self.status = TaskStatus.SUCCESS


task_registry.register("StringTask", StringTask)
