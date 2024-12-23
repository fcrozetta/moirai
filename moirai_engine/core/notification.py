class InnerNotification:
    """
    Inner Notification is used when a socket or action needs to send a notification or log.
    """

    def __init__(self, component_id: str, level: int, message: dict):
        self.component_id = component_id
        self.level = level
        self.message = message

    def __str__(self):
        return f"{self.component_id} - {self.level}: {self.message}"

    def to_dict(self):
        return {
            "component_id": self.component_id,
            "level": self.level,
            "message": self.message,
        }


class Notification:
    """
    Notification is used when a workflow needs to send a notification or log.
    The message has to be an InnerNotification object, even if the sender is the workflow itself.
    """

    def __init__(
        self,
        workflow_id: str,
        level: int,
        notification_timestamp: str,
        message: InnerNotification,
    ):
        self.workflow_id = workflow_id
        self.level = level
        self.notification_timestamp = notification_timestamp
        self.message = message

    def __str__(self):
        return f"{self.notification_timestamp} {self.level} - {self.workflow_id}: {self.message}"

    def to_dict(self):
        return {
            "workflow_id": self.workflow_id,
            "level": self.level,
            "timestamp": self.notification_timestamp,
            "message": self.message.to_dict(),
        }
