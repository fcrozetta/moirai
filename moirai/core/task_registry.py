import importlib
import os
import pkgutil
import logging


class TaskRegistry:
    """Singleton registry to hold task types for dynamic instantiation."""

    _instance = None

    def __new__(cls, *args, **kwargs):
        if not cls._instance:
            cls._instance = super(TaskRegistry, cls).__new__(cls, *args, **kwargs)
            cls._instance._registry = {}  # Ensure _registry is only initialized once
        return cls._instance

    def register(self, task_type: str, task_class):
        """Register a task class with a type name."""
        logging.info(f"Registering task: {task_type}")
        self._registry[task_type] = task_class
        logging.debug(f"Current registry state: {self._registry}")

    def create_task(self, task_type: str, **kwargs):
        """Instantiate a task of the specified type."""
        logging.info(f"Attempting to create task of type: {task_type}")
        logging.debug(f"Current registry state in create_task: {self._registry}")
        task_class = self._registry.get(task_type)
        if not task_class:
            raise ValueError(f"Task type '{task_type}' is not registered.")
        return task_class(**kwargs)

    def discover_tasks(self):
        """Dynamically imports all task modules to ensure they are registered."""
        task_package = "moirai.core.tasks"
        package_path = os.path.dirname(importlib.import_module(task_package).__file__)
        logging.info(f"Discovering tasks in {package_path}")

        for _, module_name, _ in pkgutil.iter_modules([package_path]):
            full_module_name = f"{task_package}.{module_name}"
            logging.debug(f"Importing module {full_module_name}")
            importlib.import_module(full_module_name)
        logging.debug(f"Registry state after discovery: {self._registry}")


# Singleton instance of TaskRegistry
task_registry = TaskRegistry()
