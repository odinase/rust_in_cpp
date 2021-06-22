#include "test.h"
#include <iostream>
#include <vector>

int main()
{
    ArrayFlipper a(10);
    std::cout << "Before flip\n";
    for (const auto& aa: a.vec) {
        std::cout << aa << " ";
    }
    std::cout << std::endl;
    a.flip();
    std::cout << "After flip\n";
    for (const auto& aa: a.vec) {
        std::cout << aa << " ";
    }
    std::cout << std::endl;
    a.square();
    std::cout << "After square\n";
    for (const auto& aa: a.vec) {
        std::cout << aa << " ";
    }
    std::cout << std::endl;
    a.double_length();
    std::cout << "After double length\n";
    for (const auto& aa: a.vec) {
        std::cout << aa << " ";
    }
    std::cout << std::endl;
}