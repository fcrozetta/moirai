import re
from typing import Optional, Union, Any


class MoiraiType:
    def __init__(self, base_type: str, element_type: Optional["MoiraiType"] = None):
        """
        Initialize a MoiraiType.

        :param base_type: The base type (e.g., 'string', 'int', 'float', 'object', 'list').
        :param element_type: The element type if this is a list type (e.g., 'list<int>').
        """
        self.base_type = base_type
        self.element_type = element_type

    def __str__(self):
        if self.base_type == "list" and self.element_type:
            return f"list<{self.element_type}>"
        return self.base_type

    @classmethod
    def parse(cls, type_str: str) -> "MoiraiType":
        """Parse a type string into a MoiraiType object."""
        list_match = re.match(r"^list<(.+)>$", type_str)
        if list_match:
            element_type_str = list_match.group(1)
            return cls(base_type="list", element_type=cls.parse(element_type_str))
        if type_str in {"string", "int", "float", "object"}:
            return cls(base_type=type_str)
        raise ValueError(f"Invalid type string: {type_str}")

    def is_compatible_with(self, other: "MoiraiType") -> bool:
        """Check if this type is compatible with another type."""
        if self.base_type == "list" and other.base_type == "list":
            # Both are lists, check if their element types are compatible.
            return self.element_type.is_compatible_with(other.element_type)
        return self.base_type == other.base_type

    def __eq__(self, other: Union["MoiraiType", str]):
        if isinstance(other, str):
            other = MoiraiType.parse(other)
        if self.base_type != other.base_type:
            return False
        if self.base_type == "list":
            return self.element_type == other.element_type
        return True
