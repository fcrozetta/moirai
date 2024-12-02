import asyncio
from moirai_engine.actions.action import Action, ActionStatus
from moirai_engine.sockets.socket import SocketType


class PrintAction(Action):
    def __init__(self, id: str, label: str = "Print Action", description: str = ""):
        super().__init__(id, label, description)
        input_1 = self.create_input("input_string", "Input", SocketType.String)
        input_1.allow_direct_input = True

    async def execute(self):
        input_string = self.get_input("input_string")
        self.notify(input_string.get_value())
        self.status = ActionStatus.COMPLETED
