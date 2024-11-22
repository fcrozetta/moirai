#pragma once
#include "../common.hpp"
#include "../TaskSocket.hpp"

// Enum for Task Status
enum class TaskStatus
{
    PENDING,          // Task is initialized but not yet run
    RESOLVING,        // Task is resolving its own inputs
    RUNNING,          // Task is currently running
    SUCCESS,          // Task completed successfully
    FAILED,           // Task failed due to a generic error
    TIMEOUT,          // Task exceeded its timeout limit
    VALIDATION_ERROR, // Error related to input/output validation
    EXECUTION_ERROR   // Error during task execution
};

// BaseTask class declaration
class BaseTask
{
protected:
    std::string id;
    std::string label;
    TaskStatus status = TaskStatus::PENDING;
    BaseTask *onSuccessTask = nullptr;
    BaseTask *onFailureTask = nullptr;

public:
    bool isTargetable = true; // Other tasks can connect here?
    std::list<InputSocket> inputs;
    std::list<OutputSocket> outputs;

    BaseTask(std::string id, std::string label);
    virtual ~BaseTask();

    void preExecute();
    virtual void execute() = 0;
    void postExecute();
    void run();
    InputSocket *getInputSocket(std::string id);
    OutputSocket *getOutputSocket(std::string id);
    void resolve();
};