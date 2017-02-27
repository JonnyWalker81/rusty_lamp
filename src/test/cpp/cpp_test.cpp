#pragma once

#define CATCH_CONFIG_MAIN
#include "catch.hpp"
#include <iostream>
#include "cpp/record_test.hpp"

TEST_CASE("test") {
  std::cout << "test harness..." << std::endl;
}
