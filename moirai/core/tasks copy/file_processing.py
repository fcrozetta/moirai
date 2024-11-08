from moirai.core.base_task import BaseTask
from moirai.core.task_parameter import (
    TaskParameterConfig,
    ParameterGroup,
    TaskParameter,
)
from moirai.core.task_registry import task_registry


class FileProcessingTask(BaseTask):
    def __init__(
        self, task_id: str, name: str = "FileProcessingTask", timeout: int = None
    ):
        super().__init__(name=name, task_id=task_id, timeout=timeout)

        # Define grouped parameters
        self.parameters = [
            ParameterGroup(
                title="File Settings",
                description="Settings related to the file being processed",
                parameters=[
                    TaskParameterConfig(
                        name="file_path",
                        param_type="text",
                        label="File Path",
                        placeholder="Enter file path",
                        required=True,
                    ),
                    TaskParameterConfig(
                        name="overwrite",
                        param_type="checkbox",
                        label="Overwrite Existing File",
                        default_value=False,
                    ),
                ],
            ),
            ParameterGroup(
                title="Processing Options",
                description="Options for processing the file",
                parameters=[
                    TaskParameterConfig(
                        name="encoding",
                        param_type="select",
                        label="File Encoding",
                        options=["UTF-8", "ASCII", "ISO-8859-1"],
                        default_value="UTF-8",
                    ),
                    TaskParameterConfig(
                        name="buffer_size",
                        param_type="number",
                        label="Buffer Size (KB)",
                        default_value=1024,
                        min_value=256,
                        max_value=8192,
                    ),
                ],
            ),
        ]
        # Define inputs and outputs
        self.inputs = [TaskParameter("input_file", "", "string", role="input")]
        self.outputs = [TaskParameter("processed_file", "", "string", role="output")]


task_registry.register("FileProcessingTask", FileProcessingTask)
