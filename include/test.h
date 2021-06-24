#ifndef TEST_H
#define TEST_H

#include "rust_interface.h"
#include <vector>

class Test : public rust_interface::Test
{
    public:
    using rust_interface::Test::Test;
    void hello()
    {
        rust_interface::hello(this);
    }
    void test_hello()
    {
        rust_interface::test_hello(this);
    }
    double heyo(double *arr, uintptr_t n) const
    {
        return rust_interface::heyo(this, arr, n);
    }
};

class ArrayFlipper : public rust_interface::ArrayFlipper {
    private:
    void align_data() {
        data = vec.data();
        len = vec.size();
    }
    public:
    std::vector<double> vec;
    ArrayFlipper(int len) : rust_interface::ArrayFlipper(nullptr, 0, 0) {
        for (int i = 0; i < len; i++) {
            vec.push_back(i);
        }
        data = vec.data();
        cap = vec.capacity();
    }

    void flip() {
        align_data();
        rust_interface::flip(this);
    }

    void square() {
        align_data();
        rust_interface::square(this);
    }

    void double_length() {
        cap *= 2;
        vec.reserve(cap);
        align_data();
        rust_interface::double_length(this);
    }

    void matrix_vec_product() {
        align_data();
        rust_interface::matrix_vec_product(this);
    }
};


#endif // TEST_H