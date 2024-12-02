from moirai_engine.tasks.task import Task, TaskStatus


class StartTask(Task):
    def __init__(self, id: str = "start", label: str = "Start", description: str = ""):
        super().__init__(id, label, description)
        self.is_targetable = False

    async def execute(self):
        self.status = TaskStatus.COMPLETED
