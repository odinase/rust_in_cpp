// hello.cpp
struct Test;

extern "C" void hello();
extern "C" void test_hello(Test& test);

#include <cstdint>
#include <iostream>

struct Test {
    std::uint32_t t;
};

int main() {
    Test t = Test { 10 };
    test_hello(t);
    std::cout << "t has now " << t.t << "\n";
}