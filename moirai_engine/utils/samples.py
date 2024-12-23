"""
This module contains sample workflows that can be used for testing purposes.
"""

from uuid import uuid4
from moirai_engine.core.workflow import Workflow
from moirai_engine.actions.start_action import StartAction
from moirai_engine.actions.end_action import EndAction
from moirai_engine.actions.string_action import StringAction
from moirai_engine.actions.print_action import PrintAction
from moirai_engine.actions.sleep_action import SleepAction
from moirai_engine.actions.error_action import ErrorAction


def slow_hello_world():
    workflow_id = f"wf_{uuid4()}"
    start = StartAction("start", "Start")
    end = EndAction("end", "End")
    string = StringAction("string", "String")
    string.get_input("input_string").set_value("Hello, World!")
    sleep = SleepAction("sleep", "Sleep")
    print_ = PrintAction("print", "Print")

    workflow = Workflow(workflow_id, "Slow Hello World Workflow")
    workflow.add_action(start)
    workflow.add_action(end)
    workflow.add_action(string)
    workflow.add_action(sleep)
    workflow.add_action(print_)

    start.on_success = string
    string.on_success = sleep
    sleep.on_success = print_
    print_.on_success = end
    print_.get_input("input_string").connect(
        string.get_output("output_string").get_full_path()
    )

    workflow.start_action_id = f"{workflow_id}.start"

    return workflow


def hello_world():
    """Returns a Workflow that prints 'Hello, World!'"""
    workflow_id = f"wf_{uuid4()}"
    start = StartAction("start", "Start")
    end = EndAction("end", "End")
    string = StringAction("string", "String")
    string.get_input("input_string").set_value("Hello, World!")
    print_ = PrintAction("print", "Print")

    workflow = Workflow(workflow_id, "Example Workflow")
    workflow.add_action(start)
    workflow.add_action(end)
    workflow.add_action(string)
    workflow.add_action(print_)

    start.on_success = print_
    print_.on_success = end
    print_.get_input("input_string").connect(
        string.get_output("output_string").get_full_path()
    )

    workflow.start_action_id = f"{workflow_id}.start"

    return workflow


def force_error():
    """Returns a workflow that raises an exception"""
    workflow_id = f"wf_{uuid4()}"
    start = StartAction("start", "Start")
    good_end = EndAction("end1", "End")
    bad_end = EndAction("end2", "End with error")
    err = ErrorAction("error", "Error")

    workflow = Workflow(workflow_id, "Error Workflow")
    workflow.add_action(start)
    workflow.add_action(good_end)
    workflow.add_action(bad_end)
    workflow.add_action(err)

    start.on_success = err
    err.on_success = good_end
    err.on_failure = bad_end

    workflow.start_action_id = f"{workflow_id}.start"

    return workflow
