if __name__ == "__main__":
    import time  # Example usage
    from moirai_engine.core.engine import Engine
    from moirai_engine.core.job import Job
    from moirai_engine.tasks.start_task import StartTask
    from moirai_engine.tasks.end_task import EndTask
    from moirai_engine.tasks.string_task import StringTask
    from moirai_engine.tasks.print_task import PrintTask

    start = StartTask("start", "Start")
    end = EndTask("end", "End")
    string = StringTask("string", "String")
    string.get_input("input_string").set_value("Hello, World!")
    print_ = PrintTask("print", "Print")

    job = Job("job1", "Example Job")
    job.add_task(start)
    job.add_task(end)
    job.add_task(string)
    job.add_task(print_)

    start.on_success = print_
    print_.get_input("input_string").connect(
        string.get_output("output_string").get_full_path()
    )
    print_.on_success = end

    job.start_task_id = "job1.start"

    engine = Engine()
    engine.start()
    engine.add_job(job)

    # Let the engine run for a while
    time.sleep(5)
    engine.stop()
