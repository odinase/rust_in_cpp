#ifndef RUST_INTERFACE_H
#define RUST_INTERFACE_H

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

namespace rust_interface {

enum class Testy {
  Go,
  Stop,
};

struct Enum {
  enum class Tag {
    A,
    B,
  };

  struct A_Body {
    uint32_t _0;

    A_Body(uint32_t const& _0)
      : _0(_0)
    {}

  };

  struct B_Body {
    uint32_t _0;

    B_Body(uint32_t const& _0)
      : _0(_0)
    {}

  };

  Tag tag;
  union {
    A_Body a;
    B_Body b;
  };
};

struct ArrayFlipper {
  double *data;
  uintptr_t len;
  uintptr_t cap;

  ArrayFlipper(double *const& data,
               uintptr_t const& len,
               uintptr_t const& cap)
    : data(data),
      len(len),
      cap(cap)
  {}

};

struct Test {
  uint32_t t;
  uint32_t a;

  Test(uint32_t const& t,
       uint32_t const& a)
    : t(t),
      a(a)
  {}

};

struct Test2 {
  uint32_t c;

  Test2(uint32_t const& c)
    : c(c)
  {}

};

extern "C" {

void enum_test(const Enum *e);

void flip(ArrayFlipper *self);

void square(ArrayFlipper *self);

void double_length(ArrayFlipper *self);

void matrix_vec_product(ArrayFlipper *self);

void hello(Test *self);

void test_hello(Test *test);

double heyo(const Test *test, double *arr, uintptr_t n);

void heyheyhey(const Test2 *test2, Testy testyy);

} // extern "C"

} // namespace rust_interface

#endif // RUST_INTERFACE_H
