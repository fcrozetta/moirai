#pragma once
#include <iostream>
#include <list>
#include "Task.hpp"

class PrintTask : public Task
{
public:
    PrintTask(std::string id, std::string label) : Task(id, label)
    {
        isTargetable = true;
        auto string_input = createInputSocket("input_string", "String", SocketType::String);
        string_input->allowInputOverwrite = true;
        string_input->displaySocket = true;
    }
    ~PrintTask() {}
    void execute()
    {
        auto input_value = getInputSocket("input_string")->getValue<std::string>();
        std::cout << input_value << std::endl;
        this->status = TaskStatus::SUCCESS;
    }
};