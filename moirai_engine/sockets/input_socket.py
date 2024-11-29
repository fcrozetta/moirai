from moirai_engine.sockets.socket import Socket, SocketType, SocketStatus
from moirai_engine.sockets.output_socket import OutputSocket


class InputSocket(Socket):
    diplay_socket: bool = True
    allow_direct_input: bool = False

    def __init__(self, id: str, label: str, type: SocketType, parent=None):
        super().__init__(id, label, type)
        self._value = None
        self.parent = parent
        self.source: OutputSocket = None
        self.source_full_path: str

    def to_dict(self):
        result = super().to_dict()
        local = {
            "display_socket": self.diplay_socket,
            "allow_direct_input": self.allow_direct_input,
            "source_full_path": self.source.get_full_path() if self.source else None,
        }
        result.update(local)
        return result

    @classmethod
    def from_dict(cls, data):
        socket = super().from_dict(data)
        socket.diplay_socket = data["display_socket"]
        socket.allow_direct_input = data["allow_direct_input"]
        return socket

    def connect(self, source_path: str):
        self.source_full_path = source_path

    async def resolve(self):
        if self.status == SocketStatus.PENDING:
            self.status = SocketStatus.RESOLVING
            source = self.parent.find_in_job(self.source_full_path)
            if not self.is_compatible(source):
                self.status = SocketStatus.ERROR
                raise Exception("Incompatible socket types")
            self.source = source
            if self.source is not None:
                await self.source.resolve()
                self.set_value(self.source.get_value())
                self.status = SocketStatus.RESOLVED
            else:
                self.status = SocketStatus.ERROR
                raise Exception("Could not resolve input socket")

    def get_full_path(self):
        # FIXME: The socket should not have to specify if it is an input or output socket
        return super().get_full_path("inputs")
