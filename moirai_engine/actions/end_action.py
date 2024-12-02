from moirai_engine.actions.action import Action, ActionStatus


class EndAction(Action):
    def __init__(
        self, id: str = "end", label: str = "End Action", description: str = ""
    ):
        super().__init__(id, label, description)
        self.on_success = None

    async def execute(self):
        self.notify("END Action")
        self.status = ActionStatus.COMPLETED
