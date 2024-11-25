#include "Engine.hpp"

Engine::Engine() : running(false) {}

Engine::~Engine()
{
    stop();
}

void Engine::start()
{
    if (running.exchange(true))
        return; // Already running

    workerThread = std::thread(&Engine::processJobs, this);
}

void Engine::stop()
{
    running = false;
    if (workerThread.joinable())
    {
        workerThread.join();
    }
}

void Engine::addJob(const Job &job)
{
    std::lock_guard<std::mutex> lock(jobsMutex);
    jobs.push_back(job);
}

void Engine::processJobs()
{
    while (running)
    {
        std::lock_guard<std::mutex> lock(jobsMutex);
        for (auto &job : jobs)
        {
            if (job.getStatus() == JobStatus::PENDING)
            {
                job.run();
            }
        }
        std::this_thread::sleep_for(std::chrono::milliseconds(100)); // Adjust sleep duration as needed
    }
}