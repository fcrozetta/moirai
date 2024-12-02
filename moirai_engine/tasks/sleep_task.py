import time, asyncio
from moirai_engine.tasks.task import Task, TaskStatus


class SleepTask(Task):
    def __init__(self, task_id: str, label: str = "Sleep", description: str = ""):
        super().__init__(task_id, label, description)

    async def execute(self):
        time.sleep(3)
        self.status = TaskStatus.COMPLETED
