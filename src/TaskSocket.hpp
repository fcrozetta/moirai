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

// ValueData type alias
using ValueData = std::variant<std::monostate, int, float, std::string, bool, std::list<int>, std::list<float>, std::list<std::string>, std::list<bool>>;

// Forward declaration of BaseTask
class BaseTask;

// Base Socket class
class Socket
{
protected:
    ValueData value;  // The actual value of the socket
    BaseTask *parent; // Pointer to the parent task for resolving inputs

public:
    std::string id;
    std::string label;
    bool isResolved = false;
    SocketType type;

    Socket(const std::string &id, const std::string &label, SocketType type);
    virtual ~Socket();

    virtual void validate() const = 0;
    void resolve();
    bool isCompatible(Socket *otherSocket);
    void setValue(ValueData data);
    bool hasValue();

    template <typename T>
    T getValue() const;
};

// Derived OutputSocket class
class OutputSocket : public Socket
{
public:
    OutputSocket(const std::string &id, const std::string &label, SocketType type);
    void validate() const override;
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
    void setSource(OutputSocket *source);
    OutputSocket *getSource();
};
