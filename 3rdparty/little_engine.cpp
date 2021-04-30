#include "little_engine.h"

using namespace LE;

std::vector<int> Game::getInts() {
    return std::vector{1, 2, 3, 4, 5};
}

std::vector<Point> Game::getPoints() {
    std::vector result {
        Point(11, 22),
        Point(22, 33),
        Point(33, 44),
        Point(44, 55),
        Point(55, 66)
    };
    return result;
}
