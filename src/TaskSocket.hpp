#pragma once
#include <variant>
#include "common.hpp"

// Supported types enumeration
enum class SocketType
{
    Int,
    Float,
    String,
    Boolean,
    IntList,
    FloatList,
    StringList,
    BooleanList
};

// Forward declaration of Task
class Task;

// Base Socket class
class Socket
{
protected:
    ValueData value; // The actual value of the socket
    Task *parent;    // Pointer to the parent task for resolving inputs

public:
    std::string id;
    std::string label;
    bool isResolved = false;
    SocketType type;

    Socket(const std::string &id, const std::string &label, SocketType type);
    virtual ~Socket();

    void addParent(Task *parent);
    Task *getParent();

    virtual void validate() const = 0;
    virtual void resolve() = 0;

    bool isCompatible(Socket *otherSocket);
    void setValue(ValueData data);
    bool hasValue();

    ValueData getRawValue();

    template <typename T>
    T getValue() const
    {
        if (std::holds_alternative<std::monostate>(value))
        {
            throw std::logic_error("Value was not resolved or was not set");
        }

        if (!std::holds_alternative<T>(value))
        {
            throw std::invalid_argument("Type mismatch in getValue");
        }
        return std::get<T>(value);
    }
};

// Derived OutputSocket class
class OutputSocket : public Socket
{
public:
    OutputSocket(const std::string &id, const std::string &label, SocketType type);
    void validate() const override;
    void resolve() override;
};

// Derived InputSocket class
class InputSocket : public Socket
{
private:
    OutputSocket *source = nullptr;

public:
    bool displaySocket = true;        // Should the socket itself be shown?
    bool allowInputOverwrite = false; // Can the input be overwritten by the user?
    InputSocket(const std::string &id, const std::string &label, SocketType type);

    void validate() const override;
    void resolve() override;
    void setSource(OutputSocket *source);
    OutputSocket *getSource();
    void resolveRawValue();
};
