#ifndef PRINT_TASK_HPP
#define PRINT_TASK_HPP

#include <iostream>
#include <list>
#include "base_task.hpp"

class PrintTask : public BaseTask
{
public:
    PrintTask(std::string id, std::string label) : BaseTask(id, label)
    {
        auto string_input = InputSocket("input_string", "String", SocketType::String);
        string_input.allowInputOverwrite = true;
        string_input.displaySocket = true;
        this->inputs.emplace_back(string_input);
    }
    ~PrintTask() {}
    void execute()
    {
        auto input_value = getInputSocket("input_string")->getValue<std::string>();
        std::cout << input_value << std::endl;
        this->status = TaskStatus::SUCCESS;
    }
};

#endif