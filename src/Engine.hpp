#pragma once

#include "Job.hpp"
#include "common.hpp" // Include common header

class Engine
{
public:
    Engine();
    ~Engine();

    void start();
    void stop();
    void addJob(const Job &job);

private:
    void processJobs();

    std::vector<Job> jobs;
    std::thread workerThread;
    std::atomic<bool> running;
    std::mutex jobsMutex;
};