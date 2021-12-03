#pragma once
#include <string>
#include <sstream>
#include <fstream>

auto get_task(int task_id) -> std::string {
    auto fname = "src/tasks/task" + std::to_string(task_id) + ".txt";
    std::ifstream ifs(fname);
    std::stringstream ss;
    ss << ifs.rdbuf();
    return ss.str();
}