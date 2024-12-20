"""
This module contains sample jobs that can be used for testing purposes.
"""

from uuid import uuid4
from moirai_engine.core.job import Job
from moirai_engine.actions.start_action import StartAction
from moirai_engine.actions.end_action import EndAction
from moirai_engine.actions.string_action import StringAction
from moirai_engine.actions.print_action import PrintAction
from moirai_engine.actions.sleep_action import SleepAction
from moirai_engine.actions.error_action import ErrorAction


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


def force_error():
    """Returns a job that raises an exception"""
    job_id = f"job_{uuid4()}"
    start = StartAction("start", "Start")
    good_end = EndAction("end1", "End")
    bad_end = EndAction("end2", "End with error")
    err = ErrorAction("error", "Error")

    job = Job(job_id, "Error Job")
    job.add_action(start)
    job.add_action(good_end)
    job.add_action(bad_end)
    job.add_action(err)

    start.on_success = err
    err.on_success = good_end
    err.on_failure = bad_end

    job.start_action_id = f"{job_id}.start"

    return job
