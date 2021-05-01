#pragma once

#include <vector>

namespace LE {

struct Point {
    int _x;
    int _y;
    Point(int x, int y): _x(x), _y(y) {}
};

class Game {
public:
    std::vector<int> getInts();
    std::vector<Point> getPoints();
    void debug() const;
};

typedef Game *GamePtr;

}

