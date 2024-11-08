from enum import Enum


class TaskStatus(Enum):
    PENDING = "pending"  # Task is initialized but not yet run
    RUNNING = "running"  # Task is currently running
    SUCCESS = "success"  # Task completed successfully
    FAILED = "failed"  # Task failed due to a generic error
    TIMEOUT = "timeout"  # Task exceeded its timeout limit
    VALIDATION_ERROR = "validation_error"  # Error related to input/output validation
    EXECUTION_ERROR = "execution_error"  # Error during task execution
