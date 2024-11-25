
#pragma once
#include "Task.hpp"

class FailTask : public Task
{
public:
    FailTask(const std::string &id, const std::string &label) : Task(id, label) {}

    void execute() override
    {
        status = TaskStatus::FAILED;
        throw std::runtime_error("Task failed intentionally");
    }
};