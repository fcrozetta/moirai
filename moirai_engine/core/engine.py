import threading
from queue import Queue
from typing import List
from datetime import datetime
from moirai_engine.core.job import Job


class Engine:
    engine_id = "_moirai"

    def __init__(self, max_workers=4, listener: callable = None):
        self.job_queue = Queue()
        self.is_running = False
        self.max_workers = max_workers
        self.threads: List[threading.Thread] = []
        self.notification_listeners: dict[str, List[callable]] = {self.engine_id: []}
        self.notification_history: dict[str, List[str]] = {}

        self.add_listener(listener, self.engine_id)

    def start(self):
        if not self.is_running:
            self.is_running = True
            for _ in range(self.max_workers):
                t = threading.Thread(target=self.worker, daemon=True)
                t.start()
                self.threads.append(t)
            self.notify("[Start] Engine")

    def stop(self):
        if self.is_running:
            self.is_running = False
            self.job_queue.join()
            while all(t.is_alive() for t in self.threads):
                print("Waiting for threads to finish")
                for t in self.threads:
                    t.join(5)
            self.notify("[Stop] Engine")

    def worker(self):
        while self.is_running:
            job = self.job_queue.get()
            try:
                job.run()
            except Exception as e:
                print(f"Error in job {job.label}: {str(e)}")
                self.notify(f"[Error] job_id:{job.label}.  err:{str(e)}")
            finally:
                self.job_queue.task_done()

    def add_job(self, job: Job, listener: callable = None):
        job.engine = self
        self.add_listener(listener, job.id)
        self.job_queue.put(job)
        self.notify(f"[Queued] {job.label}")

    def add_listener(self, listener: callable, job_id: str | None = None):
        """Add a new listener to job_id. If job_id not defined, read engine notifications"""
        job_id = job_id or self.engine_id
        if listener is None:
            return
        if job_id not in self.notification_listeners:
            self.notification_listeners[job_id] = []
        self.notification_listeners[job_id].append(listener)

    def get_notification_history(self, job_id: str | None = None) -> List[str]:
        job_id = job_id or self.engine_id
        return self.notification_history.get(job_id, [])

    def notify(self, message: str, job_id: str | None = None):
        job_id = job_id or self.engine_id
        system_message = (
            f"[{datetime.now().strftime('%Y-%m-%d %H:%M:%S')}] [{job_id}] {message}"
        )
        if job_id not in self.notification_listeners:
            self.notification_listeners[job_id] = []
        for listener in self.notification_listeners[job_id]:
            threading.Thread(target=listener, args=(system_message,)).start()
        if job_id not in self.notification_history:
            self.notification_history[job_id] = []
        self.notification_history[job_id].append(system_message)
