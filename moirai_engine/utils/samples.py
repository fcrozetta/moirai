from uuid import uuid4
from moirai_engine.core.job import Job
from moirai_engine.tasks.start_task import StartTask
from moirai_engine.tasks.end_task import EndTask
from moirai_engine.tasks.string_task import StringTask
from moirai_engine.tasks.print_task import PrintTask
from moirai_engine.tasks.sleep_task import SleepTask


def slow_hello_world():
    job_id = f"job_{uuid4()}"
    start = StartTask("start", "Start")
    end = EndTask("end", "End")
    string = StringTask("string", "String")
    string.get_input("input_string").set_value("Hello, World!")
    sleep = SleepTask("sleep", "Sleep")
    print_ = PrintTask("print", "Print")

    job = Job(job_id, "Slow Hello World Job")
    job.add_task(start)
    job.add_task(end)
    job.add_task(string)
    job.add_task(sleep)
    job.add_task(print_)

    start.on_success = string
    string.on_success = sleep
    sleep.on_success = print_
    print_.on_success = end
    print_.get_input("input_string").connect(
        string.get_output("output_string").get_full_path()
    )

    job.start_task_id = f"{job_id}.start"

    return job


def hello_world():
    """Returns a job that prints 'Hello, World!'"""
    job_id = f"job_{uuid4()}"
    start = StartTask("start", "Start")
    end = EndTask("end", "End")
    string = StringTask("string", "String")
    string.get_input("input_string").set_value("Hello, World!")
    print_ = PrintTask("print", "Print")

    job = Job(job_id, "Example Job")
    job.add_task(start)
    job.add_task(end)
    job.add_task(string)
    job.add_task(print_)

    start.on_success = print_
    print_.on_success = end
    print_.get_input("input_string").connect(
        string.get_output("output_string").get_full_path()
    )

    job.start_task_id = f"{job_id}.start"

    return job
