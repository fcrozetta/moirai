from datetime import datetime
from enum import Enum
from moirai_engine.tasks.task import Task


class JobStatus(Enum):
    PENDING = "PENDING"
    RUNNING = "RUNNING"
    COMPLETED = "COMPLETED"
    ERROR = "ERROR"


class Job:
    def __init__(self, job_id: str, label: str, description: str = None):
        self.id: str = job_id
        self.label: str = label
        self.description: str = description
        self.status: JobStatus = JobStatus.PENDING

        self.tasks: list[Task] = []
        self.current_task = None
        self.start_task_id: str = None

        self.queued_at: datetime = datetime.now()
        self.started_at: datetime = None
        self.completed_at: datetime = None
        self.engine = None  # Reference to the engine

    def to_dict(self):
        result = {
            "id": self.id,
            "label": self.label,
            "description": self.description,
            "start_task_id": self.start_task_id,
            "status": self.status.name,
            "current_task": (
                self.current_task.get_full_path() if self.current_task else None
            ),
            "tasks": [task.to_dict() for task in self.tasks],
            "queued_at": (
                self.queued_at.strftime("%Y-%m-%d %H:%M:%S") if self.queued_at else None
            ),
            "started_at": (
                self.started_at.strftime("%Y-%m-%d %H:%M:%S")
                if self.started_at
                else None
            ),
            "completed_at": (
                self.completed_at.strftime("%Y-%m-%d %H:%M:%S")
                if self.completed_at
                else None
            ),
        }
        return result

    @classmethod
    def from_dict(cls, data):
        job = cls(data["id"], data["label"], data["description"])
        job.start_task_id = data["start_task_id"]
        job.status = JobStatus[data["status"]]
        job.queued_at = datetime.strptime(data["queued_at"], "%Y-%m-%d %H:%M:%S")
        job.started_at = datetime.strptime(data["started_at"], "%Y-%m-%d %H:%M:%S")
        job.completed_at = datetime.strptime(data["completed_at"], "%Y-%m-%d %H:%M:%S")
        job.tasks = [Task.from_dict(task_data) for task_data in data["tasks"]]
        return job

    def add_task(self, task: Task):
        task.parent = self
        self.tasks.append(task)

    def get_full_path(self):
        return self.id

    def find(self, path: str):
        parts = path.split(".")
        if parts[0] != self.id:
            raise ValueError("Invalid path")

        if len(parts) == 2:
            task_id = parts[1]
            for task in self.tasks:
                if task.id == task_id:
                    return task
            raise ValueError("Task not found")
        elif len(parts) == 4:
            task_id = parts[1]
            attribute = parts[2]
            socket = parts[3]

            for task in self.tasks:
                if task.id == task_id:
                    if attribute == "inputs":
                        return task.get_input(socket)
                    elif attribute == "outputs":
                        return task.get_output(socket)
                    else:
                        raise ValueError("Invalid attribute")
            raise ValueError("Task not found")
        else:
            raise ValueError("Invalid path format")

    def run(self):
        self.started_at = datetime.now()
        if self.current_task is None:
            self.current_task = self.find(self.start_task_id)
        self.current_task.run()
        self.completed_at = datetime.now()

    def notify(self, message: str, task_id: str = None):
        if self.engine:
            self.engine.notify(message, self.id)
