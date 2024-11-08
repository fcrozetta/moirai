from moirai.core.base_task import BaseTask
from moirai.core.task_registry import task_registry


class Job:
    _source: str
    _job_id: str
    _job_name: str
    _entry_task_id: str
    _tasks: list[BaseTask] = []

    def __init__(self, job_str: str) -> None:
        self._source = job_str
        self._load_job()

    def __str__(self) -> str:
        return self._job_id

    def _load_job(self):
        self._job_id = self._source["job_id"]
        self._job_name = self._source["job_name"]
        self._entry_task_id = self._source["entry_task_id"]

        for task_data in self._source["tasks"]:
            task_type = task_data["type"]

            # Filter out non-constructor fields
            # I'm pretty shure this is a shitty way of doing this
            task_kwargs = {k: v for k, v in task_data.items() if k not in ["type"]}
            task = task_registry.create_task(task_type, **task_kwargs)
            self._tasks.append(task)
