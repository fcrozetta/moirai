from moirai_engine.actions.action import Action, ActionStatus
from moirai_engine.sockets.socket import SocketType


class StringAction(Action):
    def __init__(self, id: str, label: str = "String Action", description: str = ""):
        super().__init__(id, label, description)
        input_1 = self.create_input("input_string", "string", SocketType.String)
        input_1.allow_direct_input = True
        input_1.diplay_socket = False

        self.create_output("output_string", "Output", SocketType.String)

    async def execute(self):
        input_string = self.get_input("input_string")
        output_string = self.get_output("output_string")
        output_string.set_value(input_string.get_value())

        self.status = ActionStatus.COMPLETED
