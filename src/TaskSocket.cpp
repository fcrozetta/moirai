#include "TaskSocket.hpp"
#include <stdexcept>
#include "tasks/BaseTask.hpp"

// Socket class implementation
Socket::Socket(const std::string &id, const std::string &label, SocketType type)
    : parent(nullptr), id(id), label(label), type(type) {}

Socket::~Socket() = default;

void Socket::resolve()
{
    if (!isResolved && parent)
    {
        parent->run();
    }
}

bool Socket::isCompatible(Socket *otherSocket)
{
    return type == otherSocket->type;
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
