#pragma once

#include "little_engine.h"
#include "cxx-test/src/lib.rs.h"

std::unique_ptr<LE::Game> _shim_createGame();

rust::Vec<int> _shim_getInts(LE::Game& game);

rust::Vec<LE::Point> _shim_getPoints(LE::Game& game);