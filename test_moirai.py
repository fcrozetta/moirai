# import moirai_module

# # Test TaskRegistry
# registry = moirai_module.TaskRegistry.getInstance()
# registry.registerTask("example_task")
# task = registry.createTask("example_task", "task_id", "task_label")
# print(task)

# # Test Job
# job = moirai_module.Job("job_id", "job_label")
# job.addTask(task)
# job.setStartTask("task_id")
# print(job)

# # Test Engine
# engine = moirai_module.Engine()
# engine.addJob(job)
# engine.start()
# engine.stop()
# print(engine)
