#pragma once

#include "Task.hpp"

class StringTask : public Task
{
public:
    StringTask(std::string id, std::string label) : Task(id, label)
    {
        auto string_input = createInputSocket("input_string", "String", SocketType::String);
        string_input->allowInputOverwrite = true;
        string_input->displaySocket = false;

        createOutputSocket("output_string", "Output", SocketType::String);
    }
    ~StringTask() {}
    void execute()
    {
        auto input_value = getInputSocket("input_string")->getValue<std::string>();
        auto o = getOutputSocket("output_string");
        o->setValue(input_value);
        o->isResolved = true;
        this->status = TaskStatus::SUCCESS;
    }
};