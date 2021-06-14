// hello.cpp
struct Test;

extern "C" void hello();
extern "C" void test_hello(Test *test);

#include <cstdint>
#include <iostream>

class Test
{
public:
    std::uint32_t t;

    void yoyo()
    {
        test_hello(this);
    }
};

int main()
{
    Test t = Test{10};
    t.yoyo();
    std::cout << "t has now " << t.t << "\n";
}