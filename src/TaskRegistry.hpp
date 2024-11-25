#pragma once
#include <unordered_map>
#include <functional>
#include "common.hpp"
#include "tasks/Task.hpp" // Include the correct Task header

class TaskRegistry
{
public:
    static TaskRegistry &getInstance()
    {
        static TaskRegistry instance;
        return instance;
    }

    template <typename T>
    void registerTask(const std::string &taskName)
    {
        static_assert(std::is_base_of<Task, T>::value, "T must be a subclass of Task");
        registry[taskName] = [](const std::string &id, const std::string &label) -> Task *
        { return new T(id, label); };
    }

    Task *createTask(const std::string &taskName, const std::string &id, const std::string &label)
    {
        if (registry.find(taskName) != registry.end())
        {
            return registry[taskName](id, label);
        }
        return nullptr;
    }

    std::vector<std::string> listTasks() const
    {
        std::vector<std::string> taskNames;
        for (const auto &entry : registry)
        {
            taskNames.push_back(entry.first);
        }
        return taskNames;
    }

private:
    using TaskFactory = std::function<Task *(const std::string &, const std::string &)>;
    using RegistryMap = std::unordered_map<std::string, TaskFactory>;

    RegistryMap registry;

    TaskRegistry() = default;
    TaskRegistry(const TaskRegistry &) = delete;
    TaskRegistry &operator=(const TaskRegistry &) = delete;
};
