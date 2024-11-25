from __future__ import annotations
__all__ = ['Engine', 'InputSocket', 'Job', 'OutputSocket', 'Task', 'TaskRegistry']
class Engine:
    @staticmethod
    def _pybind11_conduit_v1_(*args, **kwargs):
        ...
    def __init__(self) -> None:
        ...
    def addJob(self, job: Job) -> None:
        ...
    def start(self) -> None:
        ...
    def stop(self) -> None:
        ...
class InputSocket:
    @staticmethod
    def _pybind11_conduit_v1_(*args, **kwargs):
        ...
class Job:
    @staticmethod
    def _pybind11_conduit_v1_(*args, **kwargs):
        ...
    def __init__(self, id: str, label: str) -> None:
        ...
    def addTask(self, task: Task) -> None:
        ...
    def setStartTask(self, task: str) -> None:
        ...
class OutputSocket:
    @staticmethod
    def _pybind11_conduit_v1_(*args, **kwargs):
        ...
class Task:
    @staticmethod
    def _pybind11_conduit_v1_(*args, **kwargs):
        ...
    def getInputSocket(self, arg0: str) -> InputSocket:
        ...
    def getOutputSocket(self, arg0: str) -> OutputSocket:
        ...
    def setOnSuccess(self, task: Task) -> None:
        ...
class TaskRegistry:
    @staticmethod
    def _pybind11_conduit_v1_(*args, **kwargs):
        ...
    @staticmethod
    def getInstance() -> TaskRegistry:
        ...
    def createTask(self, taskName: str, id: str, label: str) -> Task:
        ...
    def listTasks(self) -> list[str]:
        ...
    def registerTask(self, taskName: str) -> None:
        ...
