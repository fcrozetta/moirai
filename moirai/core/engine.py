import json, logging
from moirai.core.task_registry import task_registry


class Engine:
    def __init__(self, job_file: str):
        self.job_file = job_file
        self.tasks = {}
        self.entry_task_id = None
        self.current_task = None
        self.results = {}

        # Use the singleton task registry to discover tasks
        task_registry.discover_tasks()

    def load_job(self):
        """Load and parse the job configuration file to initialize tasks."""
        with open(self.job_file, "r") as f:
            job_data = json.load(f)

        self.entry_task_id = job_data["entry_task"]

        for task_data in job_data["tasks"].values():
            task_type = task_data["type"]
            task_id = task_data["task_id"]

            # Generic parameters for all tasks
            task_kwargs = {"name": task_id, "timeout": task_data.get("timeout")}

            # Dynamically create the task instance using the TaskRegistry
            task = task_registry.create_task(task_type, **task_kwargs)

            # Set task-specific properties like inputs, outputs, and edges directly
            task.inputs = task_data.get("inputs", {})
            task.outputs = task_data.get("outputs", {})
            task.edges = task_data.get("edges", [])

            # Store task in the tasks dictionary
            self.tasks[task_id] = task

    def _resolve_inputs(self, task):
        """Resolve input values by replacing sources like 'task_id.output' with actual values."""
        for input_name, input_value in task.inputs.items():
            if isinstance(input_value, str) and "." in input_value:
                task_id, output_name = input_value.split(".")
                resolved_value = self.results.get((task_id, output_name))
                task.inputs[input_name] = (
                    resolved_value if resolved_value else input_value
                )

    def _get_next_task(self, task):
        """Get the ID of the next task based on the task’s status and edges."""
        for edge in task.edges:
            if edge["condition"] == task.status.value:
                return edge["target_task"]
        return None  # No more tasks to execute

    def run(self):
        """Run the job from the entry task, following the task flow until completion."""
        self.current_task = self.entry_task_id

        while self.current_task:
            task = self.tasks[self.current_task]
            logging.info(f"Executing task: {task.name}")

            # Resolve input sources dynamically if any outputs are referenced
            self._resolve_inputs(task)

            # Run the task and capture its output
            task.run()

            # Store task outputs for other tasks to reference
            for key, value in task.outputs.items():
                self.results[(self.current_task, key)] = value

            # Determine the next task based on the task’s edges and status
            self.current_task = self._get_next_task(task)
            if not self.current_task:
                logging.info("Job completed.")
                break
