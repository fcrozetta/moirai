from moirai_engine.actions.action import Action, ActionStatus


class StartAction(Action):
    def __init__(self, id: str = "start", label: str = "Start", description: str = ""):
        super().__init__(id, label, description)
        self.is_targetable = False

    async def execute(self):
        self.status = ActionStatus.COMPLETED
