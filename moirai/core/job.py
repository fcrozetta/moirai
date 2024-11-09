import logging
from moirai.core.base_task import BaseTask
from moirai.core.task_registry import task_registry
from moirai.core.task_status import TaskStatus


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

    def _get_task(self, task_id: str) -> BaseTask:
        for task in self._tasks:
            if task.task_id == task_id:
                return task

    def _resolve_inputs(self, task: BaseTask):
        for i in task.inputs:
            if i.value:
                input_task_id, output_name = i.value.split(".")
                input_task = self._get_task(input_task_id)
                while input_task.status == TaskStatus.PENDING:
                    input_task.run()
                    self._resolve_inputs(input_task)
                i.value = input_task.get_output(output_name)

    def _get_next_task(self, task: BaseTask) -> BaseTask:
        for edge in task.edges:
            if edge.condition == task.status.value:
                return self._get_task(edge.target)

    def run(self):
        next_task = self._get_task(self._entry_task_id)
        while next_task:
            self._resolve_inputs(next_task)
            next_task.run()
            next_task = self._get_next_task(next_task)
