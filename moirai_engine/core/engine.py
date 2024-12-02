import asyncio
from typing import List
from datetime import datetime
from moirai_engine.core.job import Job


class Engine:
    def __init__(self, max_workers=4, listener: callable = None):
        self.job_queue = asyncio.Queue()
        self.is_running = False
        self.max_workers = max_workers
        self.tasks: List[asyncio.Task] = []
        self.notification_listeners: dict[str, List[callable]] = {"_moirai": []}

        if listener:
            self.notification_listeners["_moirai"].append(listener)

    async def start(self):
        if not self.is_running:
            self.is_running = True
            for _ in range(self.max_workers):
                t = asyncio.create_task(self.worker())
                self.tasks.append(t)
            self.notify("[Start] Engine")

    async def stop(self):
        if self.is_running:
            self.is_running = False
            await self.job_queue.join()
            for t in self.tasks:
                t.cancel()
            await asyncio.gather(*self.tasks, return_exceptions=True)
            self.notify("[Stop] Engine")

    async def worker(self):
        while self.is_running:
            if self.job_queue.empty():
                await asyncio.sleep(1)
                continue
            job = await self.job_queue.get()
            try:
                await job.run()
            except Exception as e:
                print(f"Error in job {job.label}: {str(e)}")
                self.notify(f"[Error] job_id:{job.label}.  err:{str(e)}")
            finally:
                self.job_queue.task_done()

    async def add_job(self, job: Job, listener: callable = None):
        job.engine = self
        self.add_listener(listener, job.id)
        await self.job_queue.put(job)
        self.notify(f"[Queued] {job.label}")

    def add_listener(self, listener: callable, job_id: str = "_moirai"):
        """Add a new listener to job_id. If job_id not defined, read engine notifications"""
        if job_id not in self.notification_listeners:
            self.notification_listeners[job_id] = []
        self.notification_listeners[job_id].append(listener)

    def notify(self, message: str, job_id: str = "_moirai"):
        system_message = (
            f"[{datetime.now().strftime('%Y-%m-%d %H:%M:%S')}] [{job_id}] {message}"
        )
        if job_id not in self.notification_listeners:
            self.notification_listeners[job_id] = []
        for listener in self.notification_listeners[job_id]:
            asyncio.ensure_future(listener(system_message))
