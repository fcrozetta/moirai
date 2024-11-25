#pragma once
#include <chrono>
#include "common.hpp"
#include "tasks/Task.hpp"

enum class JobStatus
{
    PENDING,
    RUNNING,
    COMPLETED,
    FAILED
};

class Job
{
private:
    std::list<Task *> tasks; // Store pointers to Task objects
    std::string start_id;
    std::string id;
    std::string label;
    std::string description;
    JobStatus status;

    std::chrono::system_clock::time_point queuedTime;
    std::chrono::system_clock::time_point startTime;
    std::chrono::system_clock::time_point endTime;

public:
    Job(std::string id, std::string label, std::string description = "");
    ~Job();

    void addTask(Task *task); // Accept pointers to Task objects
    Task *getTask(std::string id);
    void setStartTask(std::string start_id);
    void run();
    void complete();
    void fail();

    JobStatus getStatus() const; // Add getStatus method
    std::string getSummary(bool verbose) const;
};
