import threading, json
from queue import Queue
from typing import List
from datetime import datetime
from moirai_engine.core.workflow import Workflow, WorkflowStatus
from moirai_engine.core.notification import Notification, InnerNotification


class Engine:
    engine_id = "_moirai"

    def __init__(self, max_workers=4, listener: callable = None):
        self.workflow_queue = Queue()
        self.workflow_done = {}
        self.is_running = False
        self.max_workers = max_workers
        self.threads: List[threading.Thread] = []
        self.notification_listeners: dict[str, List[callable]] = {self.engine_id: []}
        self.notification_history: dict[str, List[Notification]] = {}

        self.add_listener(listener, self.engine_id)

    def start(self):
        if not self.is_running:
            self.is_running = True
            for _ in range(self.max_workers):
                t = threading.Thread(target=self.worker, daemon=True)
                t.start()
                self.threads.append(t)

            self.engine_notify("[Start] Engine")

    def stop(self):
        if self.is_running:
            self.is_running = False
            self.workflow_queue.join()
            for t in self.threads:
                self.engine_notify("Waiting for threads to finish")
                if t.is_alive():
                    t.join(5)
            self.engine_notify("[Stop] Engine")

    def worker(self):
        while self.is_running:
            workflow = self.workflow_queue.get()
            try:
                self.engine_notify(f"[Running] {workflow.label}")
                workflow.run()
            except Exception as e:
                print(f"Error in workflow {workflow.label}: {str(e)}")
                self.engine_notify(
                    f"[Error] workflow_id:{workflow.label}.  err:{str(e)}"
                )
            finally:
                # !This will be an issue for running same workflow twice
                self.workflow_done[workflow.id] = {
                    "workflow": workflow,
                    "logs": self.notification_history[workflow.id],
                }
                self.workflow_queue.task_done()

    def add_workflow(self, workflow: Workflow, listener: callable = None):
        workflow.engine = self
        self.add_listener(listener, workflow.id)
        self.workflow_queue.put(workflow)
        self.engine_notify(f"[Queued] {workflow.label}")
        self.notify(
            workflow_id=workflow.id,
            level=0,
            message=InnerNotification(
                component_id=self.engine_id,
                level=0,
                message={"message": f"[Queued] {workflow.label}"},
            ),
        )

    def add_listener(self, listener: callable, workflow_id: str | None = None):
        """Add a new listener to workflow_id. If workflow_id not defined, read engine notifications"""
        workflow_id = workflow_id or self.engine_id
        if listener is None:
            return
        if workflow_id not in self.notification_listeners:
            self.notification_listeners[workflow_id] = []
        self.notification_listeners[workflow_id].append(listener)

    def get_notification_history(
        self, workflow_id: str | None = None
    ) -> List[Notification]:
        workflow_id = workflow_id or self.engine_id
        return self.notification_history.get(workflow_id, [])

    def notify(self, workflow_id: str, level: int, message: InnerNotification):
        # FIXME: Migrate config to a separate file
        current_ts = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
        notification = Notification(workflow_id, level, current_ts, message)
        if workflow_id not in self.notification_listeners:
            self.notification_listeners[workflow_id] = []
        for listener in self.notification_listeners[workflow_id]:
            threading.Thread(
                target=listener, args=(json.dumps(notification.to_dict()),)
            ).start()

        if workflow_id not in self.notification_history:
            self.notification_history[workflow_id] = []
        self.notification_history[workflow_id].append(notification)

    def engine_notify(self, message: str, level=0):
        notif = InnerNotification(
            self.engine_id, level=level, message={"message": message}
        )
        self.notify(self.engine_id, level=level, message=message)
