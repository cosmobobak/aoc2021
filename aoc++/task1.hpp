#include <algorithm>
#include <numeric>
#include <iostream>
#include <string>
#include <vector>
#include <ranges>

#include "get_task.hpp"

namespace ranges = std::ranges;

void task1() {
    auto input = std::stringstream(get_task(1));

    // collect ints into a vector
    auto vec = std::vector<int>{};
    auto line = std::string{};
    while (std::getline(input, line)) {
        vec.push_back(std::stoi(line));
    }

    // part 1

    auto diffs = std::vector<int>(vec.size());
    auto end = std::adjacent_difference(vec.cbegin(), vec.cend(), diffs.begin(), std::greater<>{});
    auto count = std::accumulate(std::next(diffs.begin()), end, 0);

    std::cout << count << std::endl;

    // part 2

    // group into threes
    auto threesums = std::vector<int>{};
    for (auto i = 2; i < vec.size(); i ++) {
        threesums.push_back(vec[i] + vec[i - 1] + vec[i - 2]);
    }
    auto threediffs = std::vector<int>(threesums.size());
    auto end2 = std::adjacent_difference(threesums.cbegin(), threesums.cend(), threediffs.begin(), std::greater<>{});
    auto count2 = std::accumulate(std::next(threediffs.begin()), end2, 0);

    std::cout << count2 << std::endl;
}

