#include "BaseTask.hpp"
#include <stdexcept>

// Constructor
BaseTask::BaseTask(std::string id, std::string label) : id(id), label(label) {}

// Destructor
BaseTask::~BaseTask() {}

// Optional pre-execution hook
void BaseTask::preExecute() {}

// Optional post-execution hook
void BaseTask::postExecute() {}

// Run the task
void BaseTask::run()
{
    // TODO: Implement error handling here
    preExecute();
    execute();
    postExecute();
}

// Get an input socket by ID
InputSocket *BaseTask::getInputSocket(std::string id)
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
OutputSocket *BaseTask::getOutputSocket(std::string id)
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
void BaseTask::resolve()
{
    // ! This seems like the method where things start to crash O_O
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

        // i.setValue(source->getValue<>());
    }
}
