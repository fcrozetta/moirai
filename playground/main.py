import json, threading
from moirai_engine.core.engine import Engine
from moirai_engine.utils.samples import hello_world, slow_hello_world


def notification_listener(notification):
    print(f"Received notification: {notification}")


def main():
    engine = Engine(max_workers=4)
    engine.start()

    # Create jobs
    job = slow_hello_world()

    # Add jobs to the engine
    engine.add_job(job=job)

    # Adding a stress test, for fun
    # while True:
    #     engine.add_job(job=slow_hello_world(), listener=notification_listener)

    # # Let the engine run for a while
    threading.Event().wait(2)
    hist = engine.get_notification_history(job_id=job.id)
    print("History:")
    for h in hist:
        print(h)
    # engine_hist = engine.get_notification_history(job_id="_moirai")
    # print("Engine History:")
    # for h in engine_hist:
    #     print(h)

    engine.stop()


if __name__ == "__main__":
    main()
    print("Done")
    exit(0)
