#include "TaskSocket.hpp"
#include <stdexcept>
#include "tasks/Task.hpp"

// Socket class implementation
Socket::Socket(const std::string &id, const std::string &label, SocketType type)
    : parent(nullptr), id(id), label(label), type(type) {}

Socket::~Socket() = default;

void Socket::addParent(Task *parent)
{
    this->parent = parent;
}

Task *Socket::getParent()
{
    return this->parent;
}

bool Socket::isCompatible(Socket *otherSocket)
{
    return type == otherSocket->type;
}

ValueData Socket::getRawValue()
{
    return value;
}

void Socket::setValue(ValueData data)
{
    value = std::move(data);
}

bool Socket::hasValue()
{
    return !std::holds_alternative<std::monostate>(value);
}

// OutputSocket class implementation
OutputSocket::OutputSocket(const std::string &id, const std::string &label, SocketType type)
    : Socket(id, label, type) {}

void OutputSocket::validate() const
{
    // Add validation logic specific to OutputSocket, if needed
}

// InputSocket class implementation
InputSocket::InputSocket(const std::string &id, const std::string &label, SocketType type)
    : Socket(id, label, type) {}

void InputSocket::validate() const
{
    // Add validation logic specific to InputSocket, if needed
}

void InputSocket::setSource(OutputSocket *source)
{
    this->source = source;
}

OutputSocket *InputSocket::getSource()
{
    return this->source;
}

void OutputSocket::resolve()
{
    if (!isResolved)
    {
        parent->run();
    }
}

void InputSocket::resolve()
{
    if (isResolved)
    {
        return;
    }

    if (hasValue())
    {
        isResolved = true;
        return;
    }

    if (!source)
    {
        throw SocketInputSourceNullPointer("No valid source found");
    }

    source->resolve();
    resolveRawValue();
}

// Resolve value will retrieve source.getRawValue and add it on its own value.
void InputSocket::resolveRawValue()
{
    if (!source || !source->isResolved)
    {
        throw SocketValueException("Source not resolved");
    }

    value = source->getRawValue();
}
