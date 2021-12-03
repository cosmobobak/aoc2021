#include <iostream>
#include <string>
#include <vector>
#include <utility>

#include "task1.hpp"

auto main() -> int {
    auto stack_str = std::string("hello world! more text to avoid SSO");
    auto addr_of_stack_buffer = stack_str.data();
    auto heap_str = new std::string(std::move(stack_str));
    auto addr_of_heap_buffer = heap_str->data();
    std::cout << "stack: " << (size_t)addr_of_stack_buffer << std::endl;
    std::cout << "heap: " << (size_t)addr_of_heap_buffer << std::endl;
}
