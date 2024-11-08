from moirai.core.moirai_type import MoiraiType
from typing import Any, Optional, List, Union


class TaskParameterIO:
    def __init__(
        self,
        variable_name: str,
        name: str,
        value: Any,
        expected_type: Union[str, MoiraiType],
        role: str = "input",
    ):
        """
        :param name: The name of the parameter (input or output).
        :param value: The actual value.
        :param expected_type: The expected type of the value.
        :param role: 'input' or 'output' to identify its role.
        """
        self.variable_name = variable_name
        self.name = name
        self.value = value
        self.expected_type = (
            MoiraiType.parse(expected_type)
            if isinstance(expected_type, str)
            else expected_type
        )
        self.role = role  # Can be 'input' or 'output'
        self._validate_type()

    def _validate_type(self):
        """Validate that the value matches the expected type."""
        if not self.expected_type.is_compatible_with(
            MoiraiType.parse(self._infer_type())
        ):
            raise TypeError(
                f"{self.role.capitalize()} '{self.name}' expected type {self.expected_type}, but got {self._infer_type()}"
            )

    def _infer_type(self) -> str:
        """Infer the type of the value for validation."""
        if isinstance(self.value, str):
            return "string"
        elif isinstance(self.value, int):
            return "int"
        elif isinstance(self.value, float):
            return "float"
        elif isinstance(self.value, dict):
            return "object"
        elif isinstance(self.value, list):
            if not self.value:
                # Check if expected_type is a MoiraiType instance and is a list type
                if (
                    isinstance(self.expected_type, MoiraiType)
                    and self.expected_type.base_type == "list"
                ):
                    # Return the type as "list<element_type>" if element_type is defined
                    return f"list<{self.expected_type.element_type}>"
                else:
                    return "list<any>"
            element_type = MoiraiType.parse(type(self.value[0]).__name__)
            return f"list<{element_type}>"
        raise TypeError(f"Unsupported type for value: {self.value}")


class TaskParameterInternal:
    def __init__(
        self,
        variable_name: str,
        param_type: str,
        label: str,
        default_value: Any = None,
        required: bool = False,
        options: Optional[List[Union[str, int]]] = None,
        placeholder: Optional[str] = None,
        min_value: Optional[float] = None,
        max_value: Optional[float] = None,
    ):
        """
        :param variable_name: The unique variable_name for the parameter.
        :param param_type: The type of control (e.g., 'text', 'number', 'select', 'checkbox').
        :param label: A user-friendly label for display in the UI.
        :param default_value: The default value for the parameter.
        :param required: If true, this parameter must be filled in.
        :param options: For select/radio types, a list of allowed options.
        :param placeholder: Placeholder text for text input fields.
        :param min_value: Minimum value (for numeric parameters).
        :param max_value: Maximum value (for numeric parameters).
        """
        self.variable_name = variable_name
        self.param_type = param_type
        self.label = label
        self.value = default_value
        self.default_value = default_value
        self.required = required
        self.options = options
        self.placeholder = placeholder
        self.min_value = min_value
        self.max_value = max_value


class TaskParameterInternalGroup:
    def __init__(
        self,
        group_id: str,
        title: str,
        description: str = "",
        parameters: Optional[List[TaskParameterInternal]] = None,
    ):
        """
        :param title: The title of the group (for UI display).
        :param description: Optional description for the group.
        :param parameters: List of TaskParameterInternal instances in this group.
        """
        self.group_id = group_id
        self.title = title
        self.description = description
        self.parameters = parameters or []


class TaskEdge:
    condition: str
    target: str

    def __init__(self, condition: str, target: str = "") -> None:
        """Task flow. this defines where the task goes next

        Args:
            condition (str): success / failure
            target (str): id of the target task. Must be present in  the job
        """
        self.target = target
        self.condition = condition
