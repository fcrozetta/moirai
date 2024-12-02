from moirai_engine.tasks.task import Task, TaskStatus


class EndTask(Task):
    def __init__(
        self, task_id: str = "end", label: str = "End Task", description: str = ""
    ):
        super().__init__(task_id, label, description)
        self.on_success = None

    async def execute(self):
        self.notify("END TASK")
        self.status = TaskStatus.COMPLETED
