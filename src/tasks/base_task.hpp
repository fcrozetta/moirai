#ifndef BASE_TASK_HPP
#define BASE_TASK_HPP

#include <iostream>
#include <list>
#include "../socket.hpp"

enum class TaskStatus
{
    PENDING,          // Task is initialized but not yet run
    RUNNING,          // Task is currently running
    SUCCESS,          // Task completed successfully
    FAILED,           // Task failed due to a generic error
    TIMEOUT,          // Task exceeded its timeout limit
    VALIDATION_ERROR, // Error related to input/output validation
    EXECUTION_ERROR   // Error during task execution
};

class BaseTask
{
protected:
    std::string id;
    std::string label;
    TaskStatus status = TaskStatus::PENDING;

public:
    std::list<InputSocket> inputs;
    std::list<OutputSocket> ouputs;
    BaseTask(std::string id, std::string label) : id(id), label(label) {}
    ~BaseTask() {}
    void preExecute() {}
    virtual void execute() = 0;
    void postExecute() {}

    void run()
    {
        // TODO: Implement error handling here, please
        preExecute();
        execute();
        postExecute();
    }

    InputSocket *getInputSocket(std::string id)
    {
        for (auto &&i : this->inputs)
        {
            if (i.id == id)
            {
                return &i;
            }
        }
        throw std::logic_error("Input non-existent");
    }

    OutputSocket *getOutputSocket(std::string id)
    {
        for (auto &&i : this->ouputs)
        {
            if (i.id == id)
            {
                return &i;
            }
        }
        throw std::logic_error("Output non-existent");
    }
};

#endif