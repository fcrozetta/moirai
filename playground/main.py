import time
from moirai_engine.core.engine import Engine
from moirai_engine.utils.samples import hello_world, slow_hello_world


def notification_listener(notification):
    print(f"Received notification: {notification}")


engine = Engine(max_workers=4)
engine.add_notification_listener(notification_listener)
engine.start()
job = slow_hello_world()
job2 = hello_world()

print(f"job1:{job.id}")
print(f"job2:{job2.id}")

engine.add_job(job)

# Start a notification listener for the job
engine.start_notification_listener(job.id)
engine.start_notification_listener(job2.id)
engine.add_job(job2)

# Let the engine run for a while
time.sleep(2)

# Cancel the current job
# engine.cancel_current_job(job.id)

# Let the engine run for a while
time.sleep(2)

engine.stop()

# print("AFTER STOPPING ENGINE")
# # Get notification history for the job
# history = engine.get_notification_history(job.id)
# for entry in history:
#     print(entry)
# # Get notification history for the job
# history = engine.get_notification_history(job2.id)
# for entry in history:
#     print(entry)
