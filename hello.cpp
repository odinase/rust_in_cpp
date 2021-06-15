#include "rust_interface.h"
#include <iostream>
#include <vector>

int main()
{
    using rust_interface::Test;
    Test t(10, 10);
    std::cout << "t has now " << t.t << "\n";
    hello(&t);

    t.a = 12345;
}