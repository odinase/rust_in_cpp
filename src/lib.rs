#![feature(vec_into_raw_parts)]

use std::ptr;
use ndarray::prelude::*;

#[repr(C)]
pub struct Test {
    pub t: u32,
    pub a: u32,
}


#[repr(C)]
pub enum Enum {
    A(u32),
    B(u32),
} 


#[no_mangle]
pub extern "C" fn enum_test(e: &Enum) {
    match e {
        Enum::A(a) => println!("Got A and {}", a),
        Enum::B(b) => println!("Got B and {}", b),
    }
}

#[repr(C)]
pub struct ArrayFlipper {
    pub data: *mut f64,
    pub len: usize,
    pub cap: usize,
}

impl ArrayFlipper {
    pub fn new() -> Self {
        ArrayFlipper {
            data: ptr::null_mut(),
            len: 0,
            cap: 0,
        }
    }
    
    #[no_mangle]
    pub extern "C" fn flip(&mut self) {
        let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.len) };
        s.reverse();
    }

    #[no_mangle]
    pub extern "C" fn square(&mut self) {
        let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.len) };
        for ss in s {
            *ss = (*ss)*(*ss);
        }
    }

    #[no_mangle]
    pub extern "C" fn double_length(&mut self) {
        let mut s = unsafe { Vec::from_raw_parts(self.data, self.len, self.cap) };
        let mut i = 0;
        while i < self.len {
            s.push(s[i]);
            i += 1;
        }
        self.len += i;
        // let s: Vec<_> = s.iter().chain(s.iter()).collect();
        let _ = s.into_raw_parts();
    }

    #[no_mangle]
    pub extern "C" fn matrix_vec_product(&mut self) {
        let mut a = unsafe {ArrayViewMut::from_shape_ptr((3, 3), self.data) };
        a.assign(&a.dot(&arr1(&[1., 2., 3.])));
        self.len = 3;
    }
}

#[repr(C)]
pub struct Test2 {
    pub c: u32,
}

#[repr(C)]
#[derive(Debug)]
pub enum Testy {
    Go,
    Stop
  }

impl Test {
    #[no_mangle]
    pub extern "C" fn hello(&mut self) {
        println!("got in {}", self.t);
        self.t += 12;
        println!("changed to {}", self.t);
    }
}

mod cpp_interface {
    use super::*;
    // use libc::{c_double, size_t};
    #[no_mangle]
    pub extern "C" fn test_hello(test: &mut Test) {
        test.hello();
    }
    #[no_mangle]
    pub extern "C" fn heyo(test: &Test, arr: *mut f64, n: usize) -> f64 {
        let v = unsafe { std::slice::from_raw_parts(arr, n) };
        for (k, vv) in v.iter().enumerate() {
            println!("{} element is {}", k, vv);
        }
        println!("t.a is {}", test.a);

        return 1.2;
    }

    #[no_mangle]
    pub extern "C" fn heyheyhey(test2: &Test2, testyy: Testy) {
        println!("{}, {:?}", test2.c, testyy);
    }

}
