import asyncio
from datetime import datetime
from enum import Enum
from moirai_engine.actions.action import Action


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

        self.actions: list[Action] = []
        self.current_action = None
        self.start_action_id: str = None

        self.queued_at: datetime = datetime.now()
        self.started_at: datetime = None
        self.completed_at: datetime = None
        self.engine = None  # Reference to the engine

    def to_dict(self):
        result = {
            "id": self.id,
            "label": self.label,
            "description": self.description,
            "start_action_id": self.start_action_id,
            "status": self.status.name,
            "current_action": (
                self.current_action.get_full_path() if self.current_action else None
            ),
            "actions": [action.to_dict() for action in self.actions],
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
        job.start_action_id = data["start_action_id"]
        job.status = JobStatus[data["status"]]
        job.queued_at = datetime.strptime(data["queued_at"], "%Y-%m-%d %H:%M:%S")
        job.started_at = datetime.strptime(data["started_at"], "%Y-%m-%d %H:%M:%S")
        job.completed_at = datetime.strptime(data["completed_at"], "%Y-%m-%d %H:%M:%S")
        job.actions = [Action.from_dict(action_data) for action_data in data["action"]]
        return job

    def add_action(self, action: Action):
        action.parent = self
        self.actions.append(action)

    def get_full_path(self):
        return self.id

    def find(self, path: str):
        parts = path.split(".")
        if parts[0] != self.id:
            raise ValueError("Invalid path")

        if len(parts) == 2:
            action_id = parts[1]
            for action in self.actions:
                if action.id == action_id:
                    return action
            raise ValueError("Action not found")
        elif len(parts) == 4:
            action_id = parts[1]
            attribute = parts[2]
            socket = parts[3]

            for action in self.actions:
                if action.id == action_id:
                    if attribute == "inputs":
                        return action.get_input(socket)
                    elif attribute == "outputs":
                        return action.get_output(socket)
                    else:
                        raise ValueError("Invalid attribute")
            raise ValueError("Action not found")
        else:
            raise ValueError("Invalid path format")

    async def run(self):
        self.started_at = datetime.now()
        self.notify(f"[Start] {self.label}")
        if self.current_action is None:
            self.current_action = self.find(self.start_action_id)
        await self.current_action.run()
        self.completed_at = datetime.now()
        self.notify(f"[End] {self.label}")

    def notify(self, message: str):
        if self.engine:
            self.engine.notify(message=message, job_id=self.id)
