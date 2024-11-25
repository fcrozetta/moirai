#include <pybind11/pybind11.h>
#include <pybind11/stl.h>
#include "TaskRegistry.hpp"
#include "Job.hpp"
#include "tasks/Task.hpp"
#include "Engine.hpp"
#include "TaskSocket.hpp" // Include the correct header for InputSocket and OutputSocket

namespace py = pybind11;

class ConcreteTask : public Task
{
public:
    ConcreteTask(const std::string &id, const std::string &label) : Task(id, label) {}
    void execute() override
    {
        // Implementation of the execute method
    }
};

void registerTaskWrapper(TaskRegistry &registry, const std::string &taskName)
{
    registry.registerTask<ConcreteTask>(taskName);
}

PYBIND11_MODULE(moirai_module, m)
{
    py::class_<TaskRegistry>(m, "TaskRegistry")
        .def("getInstance", &TaskRegistry::getInstance, py::return_value_policy::reference)
        .def("registerTask", &registerTaskWrapper, py::arg("taskName"))
        .def("createTask", &TaskRegistry::createTask, py::arg("taskName"), py::arg("id"), py::arg("label"), py::return_value_policy::reference)
        .def("listTasks", &TaskRegistry::listTasks);

    py::class_<Job>(m, "Job")
        .def(py::init<const std::string &, const std::string &>(), py::arg("id"), py::arg("label"))
        .def("addTask", &Job::addTask, py::arg("task"))
        .def("setStartTask", &Job::setStartTask, py::arg("task"));

    py::class_<Task>(m, "Task")
        .def("setOnSuccess", &Task::setOnSuccess, py::arg("task"))
        .def("getInputSocket", &Task::getInputSocket, py::return_value_policy::reference)
        .def("getOutputSocket", &Task::getOutputSocket, py::return_value_policy::reference);

    py::class_<InputSocket>(m, "InputSocket");

    py::class_<OutputSocket>(m, "OutputSocket");

    py::class_<Engine>(m, "Engine")
        .def(py::init<>())
        .def("addJob", &Engine::addJob, py::arg("job"))
        .def("start", &Engine::start)
        .def("stop", &Engine::stop);
}