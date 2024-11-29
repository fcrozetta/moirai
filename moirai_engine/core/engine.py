import asyncio
from datetime import datetime
from moirai_engine.core.job import Job


class Engine:
    def __init__(self, max_workers=4):
        self.job_queue = asyncio.Queue()
        self.job_notification_queues = {}
        self.job_histories = {}
        self.running = False
        self.notification_listeners = []

    async def start(self):
        if not self.running:
            self.running = True
            asyncio.create_task(self.run())
            await self.notify("Engine started")

    async def stop(self):
        if self.running:
            self.running = False
            await self.notify("Engine stopped")

    async def run(self):
        while self.running:
            try:
                job = await self.job_queue.get()
                if job:
                    self.job_notification_queues[job.id] = asyncio.Queue()
                    self.job_histories[job.id] = []
                    job.engine = self  # Set the engine reference
                    asyncio.create_task(self.process_job(job))
                    await self.notify(f"Job {job.label} started", job.id)
            except asyncio.QueueEmpty:
                await asyncio.sleep(1)

    async def process_job(self, job: Job):
        """
        Processes a single job.

        Args:
            job (Job): The job to be processed.
        """
        try:
            await job.run()
            await self.notify(f"Job {job.label} completed", job.id)
        except Exception as e:
            await self.notify(f"Job {job.label} failed: {str(e)}", job.id)

    async def add_job(self, job: Job):
        """
        Adds a job to the job queue.

        Args:
            job (Job): The job to be added.
        """
        await self.job_queue.put(job)
        await self.notify(f"Job added: {job.label}", job.id)

    async def notify(self, message: str, job_id: str = None):
        """
        Sends a notification message.

        Args:
            message (str): The notification message.
            job_id (str, optional): The ID of the job associated with the notification.
        """
        timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
        notification = {"job_id": job_id, "message": message, "timestamp": timestamp}
        if job_id and job_id in self.job_notification_queues:
            await self.job_notification_queues[job_id].put(notification)
            self.job_histories[job_id].append(notification)
            for listener in self.notification_listeners:
                if asyncio.iscoroutinefunction(listener):
                    await listener(notification)
                else:
                    listener(notification)
        else:
            print(notification)  # Fallback to console if no job_id is provided

    async def get_notifications(self, job_id: str):
        notifications = []
        if job_id in self.job_notification_queues:
            while not self.job_notification_queues[job_id].empty():
                notifications.append(await self.job_notification_queues[job_id].get())
        return notifications

    def add_notification_listener(self, listener):
        if asyncio.iscoroutinefunction(listener):
            self.notification_listeners.append(listener)
        else:
            raise ValueError("Listener must be an async function")

    async def start_notification_listener(self, job_id: str):
        while self.running:
            notifications = await self.get_notifications(job_id)
            for notification in notifications:
                print(notification)
            await asyncio.sleep(1)  # Adjust the sleep time as needed

    async def get_notification_history(self, job_id: str):
        if job_id in self.job_histories:
            return self.job_histories[job_id]
        return []
