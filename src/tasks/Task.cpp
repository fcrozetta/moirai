#include "Task.hpp"
#include <stdexcept>

// Constructor
Task::Task(std::string id, std::string label) : id(id), label(label) {}

// Destructor
Task::~Task() {}

// Optional pre-execution hook
void Task::preExecute() {}

// Optional post-execution hook
void Task::postExecute() {}

// Run the task
void Task::run()
{
    if (status == TaskStatus::RUNNING || status == TaskStatus::SUCCESS)
    {
        return; // Avoid running the same task multiple times
    }

    status = TaskStatus::RUNNING;
    std::cout << "Running " << label << std::endl;
    for (auto &&i : inputs)
    {
        if (!i.isResolved)
        {
            i.resolve();
        }
    }

    preExecute();
    try
    {
        execute();
        status = TaskStatus::SUCCESS;
    }
    catch (const std::exception &e)
    {
        status = TaskStatus::FAILED;
        throw;
    }
    postExecute();

    if (status == TaskStatus::SUCCESS && onSuccessTask)
    {
        onSuccessTask->run();
    }
    else if (status == TaskStatus::FAILED && onFailureTask)
    {
        onFailureTask->run();
    }
}

void Task::setOnSuccess(Task *task)
{
    onSuccessTask = task;
}

void Task::setOnFailure(Task *task)
{
    onFailureTask = task;
}

Task *Task::getOnSuccessTask() const
{
    return onSuccessTask;
}

Task *Task::getOnFailureTask() const
{
    return onFailureTask;
}

void Task::setStatus(TaskStatus newStatus)
{
    status = newStatus;
}

TaskStatus Task::getStatus() const
{
    return status;
}

InputSocket *Task::createInputSocket(std::string id, std::string label, SocketType type)
{
    this->inputs.emplace_back(InputSocket(id, label, type));
    InputSocket *input = getInputSocket(id);
    input->addParent(this);
    return input;
}

OutputSocket *Task::createOutputSocket(std::string id, std::string label, SocketType type)
{
    this->outputs.emplace_back(OutputSocket(id, label, type));
    OutputSocket *output = getOutputSocket(id);
    output->addParent(this);
    return output;
}

// Get an input socket by ID
InputSocket *Task::getInputSocket(std::string id)
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

// Get an output socket by ID
OutputSocket *Task::getOutputSocket(std::string id)
{
    for (auto &&i : this->outputs)
    {
        if (i.id == id)
        {
            return &i;
        }
    }
    throw std::logic_error("Output non-existent");
}

// Resolve task inputs
void Task::resolve()
{
    status = TaskStatus::RESOLVING;
    for (auto &&i : inputs)
    {
        if (i.isResolved)
        {
            continue;
        }

        if (i.hasValue())
        {
            i.isResolved = true;
            break;
        }

        auto *source = i.getSource();
        if (source == nullptr)
        {
            throw SocketInputSourceNullPointer("The socket has no valid source");
        }

        if (!i.isCompatible(source))
        {
            throw SocketCompatibilityException("Sockets are not compatible");
        }

        if (!source->isResolved)
        {
            source->resolve();
        }

        i.resolveRawValue();
        i.isResolved = true;
    }
}

std::string Task::getId() const
{
    return id;
}

const std::string &Task::getLabel() const
{
    return label;
}

std::string Task::getStatusString() const
{
    switch (status)
    {
    case TaskStatus::PENDING:
        return "Pending";
    case TaskStatus::RUNNING:
        return "Running";
    case TaskStatus::SUCCESS:
        return "Success";
    case TaskStatus::FAILED:
        return "Failed";
    default:
        return "Unknown";
    }
}
