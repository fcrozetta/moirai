#include "socket.hpp"
#include "core/engine.hpp"
#include "tasks/base_task.hpp"
#include "tasks/string_task.hpp"
#include "tasks/print_task.hpp"

int main()
{
    Engine engine;

    engine.start();
    engine.stop();

    auto stringTask = StringTask("task1", "String task");
    // auto printTask = PrintTask("task2", "Print task");
    stringTask.run();
    // printTask.run();
    return 0;
}
