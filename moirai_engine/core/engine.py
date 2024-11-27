import threading
import queue
import time
from datetime import datetime
from concurrent.futures import ThreadPoolExecutor, as_completed
from moirai_engine.core.job import Job, JobStatus


class Engine:
    def __init__(self, max_workers=4):
        self.job_queue = queue.Queue()
        self.job_notification_queues = {}
        self.job_histories = {}
        self.running = False
        self.executor = ThreadPoolExecutor(max_workers=max_workers)
        self.futures = []
        self.cancel_event = threading.Event()
        self.notification_listeners = []

    def start(self):
        if not self.running:
            self.running = True
            self.thread = threading.Thread(target=self.run)
            self.thread.start()
            self.notify("Engine started")

    def stop(self):
        if self.running:
            self.running = False
            self.executor.shutdown(wait=True)
            self.thread.join()
            self.notify("Engine stopped")

    def run(self):
        while self.running:
            try:
                job = self.job_queue.get(timeout=1)  # Wait for a job for 1 second
                if job:
                    self.job_notification_queues[job.id] = queue.Queue()
                    self.job_histories[job.id] = []
                    job.engine = self  # Set the engine reference
                    future = self.executor.submit(self.process_job, job)
                    self.futures.append(future)
                    self.notify(f"Job {job.label} started", job.id)
            except queue.Empty:
                continue

    def process_job(self, job: Job):
        try:
            job.run()
            self.notify(f"Job {job.label} completed", job.id)
        except Exception as e:
            self.notify(f"Job {job.label} failed: {str(e)}", job.id)

    def add_job(self, job: Job):
        self.job_queue.put(job)
        self.notify(f"Job added: {job.label}", job.id)

    def cancel_current_job(self, job_id: str):
        self.cancel_event.set()
        self.notify(f"Job {job_id} cancelled", job_id)

    def notify(self, message: str, job_id: str = None):
        timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
        notification = {"job_id": job_id, "message": message, "timestamp": timestamp}
        if job_id and job_id in self.job_notification_queues:
            self.job_notification_queues[job_id].put(notification)
            self.job_histories[job_id].append(notification)
            for listener in self.notification_listeners:
                listener(notification)
        else:
            print(notification)  # Fallback to console if no job_id is provided

    def get_notifications(self, job_id: str):
        notifications = []
        if job_id in self.job_notification_queues:
            while not self.job_notification_queues[job_id].empty():
                notifications.append(self.job_notification_queues[job_id].get())
        return notifications

    def get_notification_history(self, job_id: str):
        if job_id in self.job_histories:
            return self.job_histories[job_id]
        return []

    def add_notification_listener(self, listener):
        self.notification_listeners.append(listener)

    def start_notification_listener(self, job_id: str):
        def listen():
            while self.running:
                notifications = self.get_notifications(job_id)
                for notification in notifications:
                    print(notification)
                time.sleep(1)  # Adjust the sleep time as needed

        listener_thread = threading.Thread(target=listen)
        listener_thread.start()
