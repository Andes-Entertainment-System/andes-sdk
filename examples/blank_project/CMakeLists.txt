cmake_minimum_required(VERSION 3.10)
project(blank-project)

include_directories(include)
file(GLOB PROJECT_SOURCES src/*)

include($ENV{ANDES_SDK_DIR}/toolchain/cmake/app.cmake)