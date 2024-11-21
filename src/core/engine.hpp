#ifndef ENGINE_HPP
#define ENGINE_HPP

#include <iostream>
#include <list>
#include "../job.hpp"

class Engine
{
private:
    std::list<Job> jobs;

public:
    Engine() {}
    ~Engine() {}

    void start()
    {
        std::cout << "Engine Started" << std::endl;
    }
    void stop()
    {
        std::cout << "Engine Stopped" << std::endl;
    }
};
#endif