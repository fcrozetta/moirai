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
    CANCELED,         // Task canceled by user
    FAILED,           // Task failed due to a generic error
    TIMEOUT,          // Task exceeded its timeout limit
    VALIDATION_ERROR, // Error related to input/output validation
    EXECUTION_ERROR   // Error during task execution
};

// Task class declaration
class Task
{
protected:
    std::string id;
    std::string label;
    TaskStatus status = TaskStatus::PENDING;

    Task *onSuccessTask = nullptr;
    Task *onFailureTask = nullptr;

public:
    bool isTargetable = true; // Other tasks can connect here?
    std::list<InputSocket> inputs;
    std::list<OutputSocket> outputs;

    Task(std::string id, std::string label);
    virtual ~Task();

    std::string getId() const;

    const std::string &getLabel() const;

    void setOnSuccess(Task *task);

    void setOnFailure(Task *task);

    Task *getOnSuccessTask() const;

    Task *getOnFailureTask() const;

    void setStatus(TaskStatus newStatus);

    TaskStatus getStatus() const;

    std::string getStatusString() const; // Ensure getStatusString is declared

    InputSocket *createInputSocket(std::string id, std::string label, SocketType type);
    OutputSocket *createOutputSocket(std::string id, std::string label, SocketType type);

    InputSocket *getInputSocket(std::string id);
    OutputSocket *getOutputSocket(std::string id);

    void preExecute();
    virtual void execute() = 0;
    void postExecute();
    void run();
    void resolve();
};