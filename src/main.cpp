#include "tasks/StartTask.hpp"
#include "tasks/EndTask.hpp"
#include "tasks/StringTask.hpp"
#include "tasks/PrintTask.hpp"
#include "tasks/FailTask.hpp" // Include a task that will fail
#include "Job.hpp"
#include "TaskRegistry.hpp" // Include the TaskRegistry header
#include "Engine.hpp"       // Include the Engine header

int main()
{
    // Register tasks
    TaskRegistry::getInstance().registerTask<StartTask>("StartTask");
    TaskRegistry::getInstance().registerTask<EndTask>("EndTask");
    TaskRegistry::getInstance().registerTask<StringTask>("StringTask");
    TaskRegistry::getInstance().registerTask<PrintTask>("PrintTask");
    TaskRegistry::getInstance().registerTask<FailTask>("FailTask"); // Register the failing task

    TaskRegistry &registry = TaskRegistry::getInstance();
    std::vector<std::string> tasks = registry.listTasks();

    std::cout << "Available tasks:" << std::endl;
    for (const auto &task : tasks)
    {
        std::cout << "- " << task << std::endl;
    }

    Engine engine;

    // Job that fails
    Job failJob("job_001", "Failing Job");

    Task *startFail = TaskRegistry::getInstance().createTask("StartTask", "start_fail", "Start");
    Task *failTask = TaskRegistry::getInstance().createTask("FailTask", "fail_task", "Fail Task");
    Task *endFail = TaskRegistry::getInstance().createTask("EndTask", "end_fail", "End");

    startFail->setOnSuccess(failTask);
    failTask->setOnSuccess(endFail);

    failJob.addTask(startFail);
    failJob.addTask(failTask);
    failJob.addTask(endFail);

    failJob.setStartTask("start_fail"); // Set the start task for the job

    engine.addJob(failJob);

    // Job that prints "Hello World"
    Job helloWorldJob("job_002", "Hello World Job");

    Task *startHello = TaskRegistry::getInstance().createTask("StartTask", "start_hello", "Start");
    Task *stringTask = TaskRegistry::getInstance().createTask("StringTask", "string_task", "String Task");
    Task *printTask = TaskRegistry::getInstance().createTask("PrintTask", "print_task", "Print Task");
    Task *endHello = TaskRegistry::getInstance().createTask("EndTask", "end_hello", "End");

    stringTask->getInputSocket("input_string")->setValue("Hello world");
    stringTask->getInputSocket("input_string")->isResolved = true;

    printTask->getInputSocket("input_string")->setSource(stringTask->getOutputSocket("output_string"));

    startHello->setOnSuccess(stringTask);
    stringTask->setOnSuccess(printTask);
    printTask->setOnSuccess(endHello);

    helloWorldJob.addTask(startHello);
    helloWorldJob.addTask(stringTask);
    helloWorldJob.addTask(printTask);
    helloWorldJob.addTask(endHello);

    helloWorldJob.setStartTask("start_hello"); // Set the start task for the job

    engine.addJob(helloWorldJob);

    engine.start();

    // Simulate some delay to allow jobs to process
    std::this_thread::sleep_for(std::chrono::seconds(2));

    engine.stop();

    return 0;
}
