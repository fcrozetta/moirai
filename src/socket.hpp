#ifndef SOCKET_HPP
#define SOCKET_HPP
#include <string>
#include <memory>
#include <vector>
#include <optional>
#include <stdexcept>
#include <iostream>
#include <variant>
#include <list>

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

// This is to create a response with every possible option...I hate the way i implemented this
using ValueData = std::variant<int, float, std::string, bool, std::list<int>, std::list<float>, std::list<std::string>, std::list<bool>>;

// Base Socket class
class Socket
{
private:
    ValueData value;

public:
    std::string id;
    std::string label;
    SocketType type;

    // Constructor for non-list types
    Socket(const std::string &id, const std::string &label, SocketType type)
        : id(id), label(label), type(type)
    {
    }

    virtual ~Socket() = default;

    // Virtual method for type-specific validation or other behaviors
    virtual void validate() const = 0;

    void setValue(ValueData data)
    {
        value = std::move(data);
    }

    // Getter function to infer and return the value
    template <typename T>
    T getValue() const
    {

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
    OutputSocket(const std::string &id, const std::string &label, SocketType type)
        : Socket(id, label, type)
    {
    }

    void validate() const override
    {
        // Add validation logic specific to OutputSocket, if needed
    }
};

// Derived InputSocket class
class InputSocket : public Socket
{
private:
    OutputSocket *source = nullptr;
    bool isResolved = false;

public:
    bool displaySocket = true;        // Should the socket itself be shown? if false, this is an internal setting.
    bool allowInputOverwrite = false; // Can the input be overwritten by the user, instead of loading it from socket?
    InputSocket(const std::string &id, const std::string &label, SocketType type)
        : Socket(id, label, type) {}

    void validate() const override
    {
        // Add validation logic specific to InputSocket, if needed
    }

    bool validateInputSource()
    {
        if (isResolved)
        {
            return type == source->type;
        }
        return false;
    }
};

#endif