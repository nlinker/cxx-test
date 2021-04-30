
#include "little_engine.h"
#include "lib.h"

std::unique_ptr<LE::Game> _shim_createGame() {
    return std::unique_ptr<LE::Game>();
}

rust::Vec<int> _shim_getInts(LE::Game& game) {
    auto xs = game.getInts();

    rust::Vec<int> result;
    result.reserve(xs.size());
    for (auto &x: xs) {
        result.push_back(x);
    }
    return result;
}

rust::Vec<LE::Point> _shim_getPoints(LE::Game& game) {
    auto xs = game.getPoints();

    rust::Vec<LE::Point> result;
    result.reserve(xs.size());
    for (auto &x: xs) {
        result.push_back(x);
    }
    return result;
}
