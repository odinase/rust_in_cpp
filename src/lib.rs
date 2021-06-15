#[repr(C)]
pub struct Test {
    pub t: u32,
    pub a: u32,
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
