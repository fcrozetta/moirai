#pragma once
#include <iostream>

using namespace std;

class SocketValueException : public std::exception
{
public:
    SocketValueException(const std::string &message) : message_(message) {}

    const char *what() const throw()
    {
        return message_.c_str();
    }

private:
    std::string message_;
};

class SocketInputSourceNullPointer : public std::exception
{
public:
    SocketInputSourceNullPointer(const std::string &message) : message_(message) {}

    const char *what() const throw()
    {
        return message_.c_str();
    }

private:
    std::string message_;
};

class SocketCompatibilityException : public std::exception
{
public:
    SocketCompatibilityException(const std::string &message) : message_(message) {}

    const char *what() const throw()
    {
        return message_.c_str();
    }

private:
    std::string message_;
};
