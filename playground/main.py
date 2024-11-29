import asyncio
from moirai_engine.core.engine import Engine
from moirai_engine.utils.samples import hello_world, slow_hello_world


async def notification_listener(notification):
    print(f"Received notification: {notification}")


async def main():
    engine = Engine(max_workers=4)
    engine.add_notification_listener(notification_listener)
    await engine.start()

    # Create jobs
    job = slow_hello_world()
    job2 = hello_world()

    # Add jobs to the engine
    await engine.add_job(job)
    await engine.add_job(job2)
    await engine.add_job(hello_world())
    await engine.add_job(hello_world())
    await engine.add_job(hello_world())
    await engine.add_job(slow_hello_world())
    await engine.add_job(slow_hello_world())
    await engine.add_job(slow_hello_world())
    await engine.add_job(slow_hello_world())
    await engine.add_job(slow_hello_world())
    await engine.add_job(slow_hello_world())

    # Start notification listeners for the jobs
    asyncio.create_task(engine.start_notification_listener(job.id))
    asyncio.create_task(engine.start_notification_listener(job2.id))

    # Let the engine run for a while
    await asyncio.sleep(2)

    await engine.stop()

    print("AFTER STOPPING ENGINE")
    # Get notification history for the job
    history = await engine.get_notification_history(job.id)
    for entry in history:
        print(entry)
    # Get notification history for the job
    history = await engine.get_notification_history(job2.id)
    for entry in history:
        print(entry)


if __name__ == "__main__":
    asyncio.run(main())
