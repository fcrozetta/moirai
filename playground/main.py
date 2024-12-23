import json, threading
from moirai_engine.core.engine import Engine
from moirai_engine.core.notification import Notification
from moirai_engine.utils.samples import hello_world, slow_hello_world


def notification_listener(notification):
    print(notification)


def main():
    engine = Engine(max_workers=4)
    engine.start()

    # Create workflows
    workflow = slow_hello_world()

    # Add workflows to the engine
    engine.add_workflow(workflow=workflow, listener=notification_listener)

    # Adding a stress test, for fun
    # while True:
    #     engine.add_job(job=slow_hello_world(), listener=notification_listener)

    # # Let the engine run for a while
    threading.Event().wait(5)
    hist = engine.get_notification_history(workflow_id=workflow.id)
    print("History:")
    for h in hist:
        print(json.dumps(h.to_dict(), indent=2))
    # engine_hist = engine.get_notification_history(job_id="_moirai")
    # print("Engine History:")
    # for h in engine_hist:
    #     print(h)

    engine.stop()


if __name__ == "__main__":
    main()
    print("Done")
    exit(0)
