import time, asyncio
from moirai_engine.actions.action import Action, ActionStatus


class SleepAction(Action):
    def __init__(self, _id: str, label: str = "Sleep", description: str = ""):
        super().__init__(_id, label, description)

    async def execute(self):
        time.sleep(3)
        self.status = ActionStatus.COMPLETED
