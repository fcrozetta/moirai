import asyncio
from moirai_engine.core.engine import Engine
from moirai_engine.utils.samples import hello_world, slow_hello_world


async def notification_listener(notification):
    print(f"Received notification: {notification}")


async def main():
    engine = Engine(max_workers=4, listener=notification_listener)
    await engine.start()

    # Create jobs
    job = slow_hello_world()
    job2 = hello_world()

    # Add jobs to the engine
    await engine.add_job(job, notification_listener)
    # await engine.add_job(hello_world(), notification_listener)
    # await engine.add_job(job2)
    # await engine.add_job(hello_world())
    # await engine.add_job(hello_world())
    # await engine.add_job(slow_hello_world())
    # await engine.add_job(slow_hello_world())
    # await engine.add_job(slow_hello_world())
    # await engine.add_job(slow_hello_world())
    # await engine.add_job(slow_hello_world())
    # await engine.add_job(slow_hello_world())

    # Start notification listeners for the jobs
    # asyncio.create_task(engine.start_notification_listener(job.id))
    # asyncio.create_task(engine.start_notification_listener(job2.id))

    # Let the engine run for a while
    await asyncio.sleep(2)
    # await engine.add_job(hello_world())

    await engine.stop()


if __name__ == "__main__":
    asyncio.run(main())
