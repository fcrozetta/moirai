#pragma once
#include "Task.hpp"

class StartTask : public Task
{
public:
    StartTask(std::string id, std::string label) : Task(id, label)
    {
        isTargetable = false;
    }

    ~StartTask() {}

    void execute()
    {
        status = TaskStatus::SUCCESS;
    }
};
