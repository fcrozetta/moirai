from uuid import uuid4
from moirai_engine.core.job import Job
from moirai_engine.actions.start_action import StartAction
from moirai_engine.actions.end_action import EndAction
from moirai_engine.actions.string_action import StringAction
from moirai_engine.actions.print_action import PrintAction
from moirai_engine.actions.sleep_action import SleepAction


def slow_hello_world():
    job_id = f"job_{uuid4()}"
    start = StartAction("start", "Start")
    end = EndAction("end", "End")
    string = StringAction("string", "String")
    string.get_input("input_string").set_value("Hello, World!")
    sleep = SleepAction("sleep", "Sleep")
    print_ = PrintAction("print", "Print")

    job = Job(job_id, "Slow Hello World Job")
    job.add_action(start)
    job.add_action(end)
    job.add_action(string)
    job.add_action(sleep)
    job.add_action(print_)

    start.on_success = string
    string.on_success = sleep
    sleep.on_success = print_
    print_.on_success = end
    print_.get_input("input_string").connect(
        string.get_output("output_string").get_full_path()
    )

    job.start_action_id = f"{job_id}.start"

    return job


def hello_world():
    """Returns a job that prints 'Hello, World!'"""
    job_id = f"job_{uuid4()}"
    start = StartAction("start", "Start")
    end = EndAction("end", "End")
    string = StringAction("string", "String")
    string.get_input("input_string").set_value("Hello, World!")
    print_ = PrintAction("print", "Print")

    job = Job(job_id, "Example Job")
    job.add_action(start)
    job.add_action(end)
    job.add_action(string)
    job.add_action(print_)

    start.on_success = print_
    print_.on_success = end
    print_.get_input("input_string").connect(
        string.get_output("output_string").get_full_path()
    )

    job.start_action_id = f"{job_id}.start"

    return job
