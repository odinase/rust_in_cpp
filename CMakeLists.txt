cmake_minimum_required(VERSION 3.0.2)
project(rust_in_cpp)

## Compile as C++17
add_compile_options(-std=c++17)

set(CMAKE_CXX_FLAGS "-O2")

include_directories(
    include
)

set(CMAKE_INSTALL_RPATH "${CMAKE_CURRENT_SOURCE_DIR}/target/debug")
link_directories("${CMAKE_CURRENT_SOURCE_DIR}/target/debug")

add_executable(hello
    hello.cpp
)

target_link_libraries(hello
    rust_in_cpp
)