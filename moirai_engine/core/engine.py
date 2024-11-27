import threading, queue
from moirai_engine.core.job import Job


class Engine:
    def __init__(self):
        self.job_queue = queue.Queue()
        self.running = False
        self.thread = None

    def start(self):
        if not self.running:
            self.running = True
            self.thread = threading.Thread(target=self._run)
            self.thread.start()
            print("Engine started")

    def stop(self):
        if self.running:
            self.running = False
            self.thread.join()
            print("Engine stopped")

    def _run(self):
        while self.running:
            try:
                job = self.job_queue.get(timeout=1)  # Wait for a job for 1 second
                if job:
                    print(f"Processing job: {job.label}")
                    job.run()
                    self.job_queue.task_done()
            except queue.Empty:
                continue

    def add_job(self, job: Job):
        self.job_queue.put(job)
        print(f"Job added: {job.label}")
