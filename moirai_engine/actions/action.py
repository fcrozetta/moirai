import asyncio
from enum import Enum
from abc import ABC, abstractmethod
from moirai_engine.sockets.socket import SocketType
from moirai_engine.sockets.input_socket import InputSocket
from moirai_engine.sockets.output_socket import OutputSocket


class ActionStatus(Enum):
    PENDING = "PENDING"
    RUNNING = "RUNNING"
    COMPLETED = "COMPLETED"
    ERROR = "ERROR"


class Action(ABC):

    def __init__(self, action_id: str, label: str, description: str = ""):
        self.id = action_id
        self.label = label
        self.description = description
        self.status = ActionStatus.PENDING

        self.parent = None
        self.is_targetable: bool = True  # this means other actions can connect here
        self.inputs: list[InputSocket] = []  # ? This should DEFINITELY be a hashmap
        self.outputs: list[OutputSocket] = []  # ? This should DEFINITELY be a hashmap
        self.on_success: "Action" = None
        self.on_failure: "Action" = None

    def to_dict(self):
        return {
            "id": self.id,
            "label": self.label,
            "description": self.description,
            "status": self.status.name,
            "is_targetable": self.is_targetable,
            "inputs": [socket.to_dict() for socket in self.inputs],
            "outputs": [socket.to_dict() for socket in self.outputs],
            "on_success": self.on_success.get_full_path() if self.on_success else None,
            "on_failure": self.on_failure.get_full_path() if self.on_failure else None,
        }

    @classmethod
    def from_dict(cls, data):
        obj = cls(
            action_id=data["id"], label=data["label"], description=data["description"]
        )
        obj.status = ActionStatus[data["status"]]
        obj.is_targetable = data["is_targetable"]
        obj.inputs = [
            InputSocket.from_dict(input_data, obj) for input_data in data["inputs"]
        ]
        obj.outputs = [
            OutputSocket.from_dict(output_data, obj) for output_data in data["outputs"]
        ]
        return obj

    def create_input(
        self, name: str, label: str, socket_type: SocketType
    ) -> InputSocket:
        socket = InputSocket(name, label, socket_type, self)
        socket.parent = self
        self.inputs.append(socket)
        return self.inputs[-1]

    def get_input(self, socket_id: str) -> InputSocket:
        for socket in self.inputs:
            if socket.id == socket_id:
                return socket
        raise Exception(f"Input socket {socket_id} not found")

    def get_output(self, socket_id: str) -> OutputSocket:
        for socket in self.outputs:
            if socket.id == socket_id:
                return socket
        return None

    def create_output(
        self, name: str, label: str, socket_type: SocketType
    ) -> OutputSocket:
        socket = OutputSocket(name, label, socket_type, self)
        socket.parent = self
        self.outputs.append(socket)
        return self.outputs[-1]

    def notify(self, message: str):
        if self.parent:
            self.parent.notify(message)

    async def run(self):
        self.notify(f"[Start] Action: {self.get_full_path()}")
        if self.status == ActionStatus.PENDING:
            self.status = ActionStatus.RUNNING

            # ? will this cause a deadlock?
            await asyncio.gather(*[socket.resolve() for socket in self.inputs])
            self.preExecute()
            try:
                await self.execute()
                self.status = ActionStatus.COMPLETED
                self.notify(f"[End] Action: {self.label}")
            except Exception as e:
                self.status = ActionStatus.ERROR
                self.notify(f"Error in action {self.label}: {str(e)}")
                print(f"Error in action {self.label}: {str(e)}")
                raise e  # ? Should we raise the exception here?
            self.postExecute()
            # ? Maybe The job should be responsible for running the on_success and on_failure actions
            if self.status == ActionStatus.COMPLETED and self.on_success is not None:
                await self.on_success.run()
            elif self.status == ActionStatus.ERROR and self.on_failure is not None:
                await self.on_failure.run()

    def find_in_job(self, path: str):
        if path.startswith(self.get_full_path()):
            remaining_path = path[len(self.get_full_path()) :].lstrip(".")
            if not remaining_path:
                return self
            for socket in self.inputs + self.outputs:
                if remaining_path.startswith(socket.id):
                    return socket.find_in_job(remaining_path)
            raise Exception(f"Path {path} not found in action {self.id}")
        else:
            return self.parent.find(path)

    def get_full_path(self):
        return self.parent.get_full_path() + "." + self.id

    def preExecute(self):
        pass

    @abstractmethod
    async def execute(self, *args, **kwargs):
        raise NotImplementedError

    def postExecute(self):
        pass
