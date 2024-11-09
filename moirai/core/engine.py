import json, logging
from moirai.core.task_registry import task_registry
from moirai.core.job import Job
import threading
import queue


class Engine:
    def __init__(self):
        # self.job_file = job_file
        # self.tasks = {}
        # self.entry_task_id = None
        # self.current_task = None
        # self.results = {}

        # Use the singleton task registry to discover tasks
        task_registry.discover_tasks()
        self.job_queue = queue.Queue()

    # def load_job(self):
    #     """Load and parse the job configuration file to initialize tasks."""
    #     with open(self.job_file, "r") as f:
    #         job_data = json.load(f)

    #     self.entry_task_id = job_data["entry_task"]

    #     for task_data in job_data["tasks"]:
    #         task_type = task_data["type"]
    #         task_id = task_data["task_id"]

    #         # Filter out non-constructor fields
    #         task_kwargs = {
    #             k: v
    #             for k, v in task_data.items()
    #             if k
    #             not in ["type", "task_id", "edges", "inputs", "outputs", "parameters"]
    #         }

    #         # Dynamically create the task instance using the TaskRegistry
    #         task = task_registry.create_task(task_type, task_id=task_id, **task_kwargs)

    #         # Set task edges for transitions
    #         task.edges = task_data.get("edges", [])

    #         # Set inputs directly on the task instance
    #         task.inputs = task_data.get("inputs", [])

    #         # Set outputs directly on the task instance
    #         task.outputs = task_data.get("outputs", [])

    #         # Populate parameters from JSON if provided
    #         parameter_values = task_data.get("parameters", {})
    #         for group in task.parameters:
    #             for param in group.parameters:

    #                 if param.name in parameter_values:
    #                     param.default_value = parameter_values[param.name]

    #         # Store task in the tasks dictionary
    #         self.tasks[task_id] = task

    # def _resolve_inputs(self, task):
    #     """Resolve input values by replacing sources like 'task_id.output' with actual values."""
    #     for task_input in task.inputs:
    #         if isinstance(input_value, str) and "." in input_value:
    #             task_id, output_name = input_value.split(".")
    #             resolved_value = self.results.get((task_id, output_name))
    #             task.inputs[input_name] = (
    #                 resolved_value if resolved_value else input_value
    #             )

    # def _get_next_task(self, task):
    #     """Get the ID of the next task based on the task’s status and edges."""
    #     for edge in task.edges:
    #         if edge["condition"] == task.status.value:
    #             return edge["target_task"]
    #     return None  # No more tasks to execute

    def add_job_from_file(self, file: str):
        with open(file, "r") as f:
            job_data = json.load(f)
            self.add_job(job_data)

    def add_job(self, job_str: str):
        job = Job(job_str)
        self.job_queue.put(job)

    def worker(self, job_queue):
        while True:
            job = job_queue.get()
            if job is None:
                break
            job.run()
            job_queue.task_done()

    def start(self):

        self.threads = []

        # for job in self.job_queue:
        #     self.job_queue.put(job)

        for _ in range(4):  # Number of worker threads
            thread = threading.Thread(target=self.worker, args=(self.job_queue,))
            thread.start()
            self.threads.append(thread)

        # Return immediately after setting up the pool
        return

    # def run(self):
    #     """Run the job from the entry task, following the task flow until completion."""
    #     self.current_task = self.entry_task_id

    #     while self.current_task:
    #         task = self.tasks[self.current_task]
    #         logging.info(f"Executing task: {task.name}")

    #         # Resolve input sources dynamically if any outputs are referenced
    #         self._resolve_inputs(task)

    #         # Run the task and capture its output
    #         task.run()

    #         # Store task outputs for other tasks to reference
    #         for key, value in task.outputs.items():
    #             self.results[(self.current_task, key)] = value

    #         # Determine the next task based on the task’s edges and status
    #         self.current_task = self._get_next_task(task)
    #         if not self.current_task:
    #             logging.info("Job completed.")
    #             break
