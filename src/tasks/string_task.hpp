#pragma once

#include <iostream>
#include <list>
#include "base_task.hpp"

class StringTask : public BaseTask
{
public:
    StringTask(std::string id, std::string label) : BaseTask(id, label)
    {
        auto string_input = InputSocket("input_string", "String", SocketType::String);
        string_input.allowInputOverwrite = true;
        string_input.displaySocket = false;
        this->inputs.emplace_back(string_input);

        auto string_output = OutputSocket("output_string", "Output", SocketType::String);
        this->ouputs.emplace_back(string_output);
    }
    ~StringTask() {}
    void execute()
    {
        auto input_value = getInputSocket("input_string")->getValue<std::string>();
        getOutputSocket("output_string")->setValue(input_value);
        this->status = TaskStatus::SUCCESS;
    }
};