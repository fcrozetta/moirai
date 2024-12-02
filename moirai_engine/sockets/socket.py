from enum import Enum
from abc import ABC, abstractmethod


class SocketType(Enum):
    Int = 0
    Float = 1
    String = 2
    Boolean = 3
    IntList = 4
    FloatList = 5
    StringList = 6
    BooleanList = 7


class SocketStatus(Enum):
    PENDING = 0
    RESOLVING = 1
    RESOLVED = 2
    ERROR = 3


class Socket(ABC):

    def __init__(self, socket_id: str, label: str, type: SocketType):
        self.id = socket_id
        self.label = label
        self.type = type
        self.status = SocketStatus.PENDING

        self._value = None
        self.parent = None  # this is an Action

    def to_dict(self):
        return {
            "id": self.id,
            "label": self.label,
            "type": self.type.name,
            "status": self.status.name,
            "value": self._value,
            "parent": self.parent.get_full_path() if self.parent else None,
        }

    @classmethod
    def from_dict(cls, data):
        socket = cls(data["id"], data["label"], SocketType[data["type"]])
        socket.status = SocketStatus[data["status"]]
        socket._value = data["value"]
        return socket

    @abstractmethod
    def resolve(self):
        pass

    def set_value(self, value):
        self._value = value
        self.status = SocketStatus.RESOLVED

    def get_value(self):
        return self._value

    def is_compatible(self, other: "Socket"):
        return self.type == other.type

    def get_full_path(self, intermediate_path: str = ""):
        return f"{self.parent.get_full_path()}.{intermediate_path}.{self.id}"
