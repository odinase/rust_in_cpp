#include "test.h"
#include <iostream>
#include <vector>

int main()
{
    rust_interface::Enum e = rust_interface::Enum::B(8);
    rust_interface::enum_test(&e);
    rust_interface::Enum e2 = rust_interface::Enum::A(34);
    rust_interface::enum_test(&e2);  
    // ArrayFlipper a(9);
    // std::cout << "Before flip\n";
    // for (const auto& aa: a.vec) {
    //     std::cout << aa << " ";
    // }
    // std::cout << std::endl;
    // a.flip();
    // std::cout << "After flip\n";
    // for (const auto& aa: a.vec) {
    //     std::cout << aa << " ";
    // }
    // std::cout << std::endl;
    // a.square();
    // std::cout << "After square\n";
    // for (const auto& aa: a.vec) {
    //     std::cout << aa << " ";
    // }
    // std::cout << std::endl;
    // // a.double_length();
    // // std::cout << "After double length\n";
    // // double *d = a.data;
    // // for (int i = 0; i < a.len; i++) {
    // //     std::cout << *(d+i) << " ";
    // // }
    // // std::cout << std::endl;
    // a.matrix_vec_product();
    // std::cout << "After matrix vec product\n";
    // double *d = a.data;
    // for (int i = 0; i < a.len; i++) {
    //     std::cout << *(d+i) << " ";
    // }
    // std::cout << std::endl;
}