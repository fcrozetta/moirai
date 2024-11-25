#include "Job.hpp"
#include <iostream>
#include <sstream>

// ...existing code...

Job::Job(std::string id, std::string label, std::string description)
    : id(id), label(label), description(description), status(JobStatus::PENDING) {}

Job::~Job() {}

void Job::addTask(Task *task)
{
    tasks.push_back(task);
}

Task *Job::getTask(std::string id)
{
    for (auto task : tasks)
    {
        if (task->getId() == id)
        {
            return task;
        }
    }
    throw std::runtime_error("Task not found");
}

void Job::setStartTask(std::string start_id)
{
    this->start_id = start_id;
}

void Job::run()
{
    status = JobStatus::RUNNING;
    try
    {
        Task *currentTask = getTask(start_id);
        while (currentTask)
        {
            try
            {
                currentTask->run();
                currentTask = currentTask->getOnSuccessTask();
            }
            catch (const std::exception &e)
            {
                std::cerr << "Task " << currentTask->getId() << " failed: " << e.what() << std::endl;
                currentTask->setStatus(TaskStatus::FAILED);
                currentTask = currentTask->getOnFailureTask();
                if (!currentTask)
                {
                    status = JobStatus::FAILED;
                    return;
                }
            }
        }
        status = JobStatus::COMPLETED;
    }
    catch (const std::exception &e)
    {
        status = JobStatus::FAILED;
        std::cerr << "Job failed: " << e.what() << std::endl;
    }
}

void Job::complete()
{
    status = JobStatus::COMPLETED;
    endTime = std::chrono::system_clock::now();
}

void Job::fail()
{
    status = JobStatus::FAILED;
    endTime = std::chrono::system_clock::now();
}

JobStatus Job::getStatus() const
{
    return status;
}

std::string Job::getSummary(bool verbose) const
{
    std::ostringstream summary;
    summary << "Job ID: " << id << "\n";
    summary << "Job Name: " << label << "\n";
    summary << "Status: " << (status == JobStatus::COMPLETED ? "Completed" : "In Progress") << "\n";
    if (verbose)
    {
        summary << "Tasks:\n";
        for (const auto &task : tasks)
        {
            summary << "- " << task->getLabel() << " (" << task->getStatusString() << ")\n";
        }
    }
    return summary.str();
}
