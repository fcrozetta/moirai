import logging
from moirai.core.task_registry import task_registry
from moirai.core.base_task import BaseTask
from moirai.core.task_status import TaskStatus
from moirai.core.task_parameter import (
    TaskParameter,
    TaskParameterInternal,
    TaskParameterInternalGroup,
)


class StringTask(BaseTask):
    inputs = [TaskParameter("strings", "Strings", None, "list<string>", "input")]
    outputs = [TaskParameter("output", "Output", "Sample Output", "string", "output")]
    parameters = [
        TaskParameterInternalGroup(
            title="Fixed string Config",
            description="used when no inputs",
            parameters=[
                TaskParameterInternal(
                    variable_name="fixed_string",
                    label="String",
                    param_type="string",
                    default_value=None,
                )
            ],
        ),
        TaskParameterInternalGroup(
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

    def __init__(
        self,
        task_id: str,
        name: str = "StringTask",
        timeout: int = None,
        parameters: list[dict] = [],
        inputs: list[dict] = [],
        outputs: list[dict] = [],
    ):
        super().__init__(
            name=name,
            task_id=task_id,
            timeout=timeout,
            parameters=parameters,
            inputs=inputs,
            outputs=outputs,
        )

        # for i in self.inputs:
        #     i.
        self.outputs = [
            TaskParameter(
                name=output_data["name"],
                value=output_data["value"],
                expected_type=output_data["type"],
                role="output",
            )
            for output_data in outputs
        ]

    def execute(self):
        """Concatenate input strings with the join string."""
        # Retrieve the join string from parameters
        join_str = next(
            (
                param.default_value
                for group in self.parameters
                for param in group.parameters
                if param.name == "join_str"
            ),
            ", ",  # Default to ", " if not found
        )

        # Retrieve the list of strings from inputs
        strings = [
            input_param.value
            for input_param in self.inputs
            if input_param.name == "strings"
        ][0]

        # Join strings and store the result in outputs
        self.outputs[0].value = join_str.join(strings)
        logging.info(
            f"{self.name} task executed successfully with output: {self.outputs[0].value}"
        )
        self.status = TaskStatus.SUCCESS


task_registry.register("StringTask", StringTask)
