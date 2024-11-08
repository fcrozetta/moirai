import uuid
from enum import Enum
from typing import List, Any, Dict


class TaskStatus(Enum):
    PENDING = "pending"  # Task is initialized but not yet run
    RUNNING = "running"  # Task is currently running
    SUCCESS = "success"  # Task completed successfully
    FAILED = "failed"  # Task failed due to a generic error
    TIMEOUT = "timeout"  # Task exceeded its timeout limit
    VALIDATION_ERROR = "validation_error"  # Error related to input/output validation
    EXECUTION_ERROR = "execution_error"  # Error during task execution


class BaseTask:
    def __init__(self, name: str, timeout: int = None):
        self.id: str  # Unique identifier for the task
        self.name = name  # Task name
        self.timeout = timeout  # Optional timeout for task execution
        self.inputs = []  # List of input objects (to be defined as needed)
        self.outputs = []  # List of output objects (to be defined as needed)
        self.status = TaskStatus.PENDING  # Initial status is 'PENDING'
        self.error_code = None  # Optional error code for specific failure reasons

    def pre_execute(self):
        """Prepare the task for execution. Can be overridden if preparation steps are needed."""
        pass

    def execute(self):
        """Main execution logic for the task. Should be implemented in subclasses."""
        raise NotImplementedError(
            "The execute method must be implemented in subclasses."
        )

    def post_execute(self):
        """Clean up resources after execution. Can be overridden if needed."""
        pass

    def run(self):
        """Orchestrates the task execution with pre- and post-execution hooks and status tracking."""
        self.status = TaskStatus.RUNNING
        try:
            self.pre_execute()
            self.execute()  # Should set outputs within the subclass
            self.status = TaskStatus.SUCCESS
        except TimeoutError:
            self.status = TaskStatus.TIMEOUT
            self.error_code = 1  # Example error code for timeout
        except ValueError:
            self.status = TaskStatus.VALIDATION_ERROR
            self.error_code = 2  # Example error code for validation error
        except Exception as e:
            self.status = TaskStatus.EXECUTION_ERROR
            self.error_code = 3  # Example error code for general execution error
        finally:
            self.post_execute()
            if self.status == TaskStatus.RUNNING:
                self.status = (
                    TaskStatus.FAILED
                )  # Set to FAILED if no other status was assigned
