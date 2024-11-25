#pragma once
#include "Task.hpp"

class EndTask : public Task
{
public:
    EndTask(std::string id, std::string label) : Task(id, label)
    {
        isTargetable = true;
        // createInputSocket("input_status_code", "Status Code", SocketType::Int); // Missing optional sockets
        onSuccessTask = nullptr;
    }
    ~EndTask() {}

    void execute()
    {
        status = TaskStatus::SUCCESS;
    }
};