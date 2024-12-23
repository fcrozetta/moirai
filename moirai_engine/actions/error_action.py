import asyncio
from moirai_engine.actions.action import Action, ActionStatus
from moirai_engine.sockets.socket import SocketType


class ErrorAction(Action):
    def __init__(
        self,
        id: str,
        label: str = "Error Action",
        description: str = "Forces an error by dividing by zero",
    ):
        super().__init__(id, label, description)

    def execute(self):
        self.notify("Forcing an error by dividing by zero", 3)
        1 / 0
