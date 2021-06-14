#[no_mangle]
pub extern "C" fn hello() {
    println!("Hello world!");
}

#[repr(C)]
pub struct Test {
    pub t: u32
}

impl Test {
    pub fn hello(&mut self) {
        println!("got in {}", self.t);
        self.t += 12;
        println!("changed to {}", self.t);
    }
}

#[no_mangle]
pub extern "C" fn test_hello(test: &mut Test) {
    test.hello();
}
