use std::{ffi::CString};

use std::os::raw::{
    c_void,
    c_char,
    c_int
};

#[link(name="rust_spmc_interface")]
extern "C" {
    fn spmc_connect_shm(path: *const c_char) -> *mut c_void;
    fn spmc_get_reader(spmc: *mut c_void) -> *mut c_void;
    fn spmc_reader_read(reader: *mut c_void) -> *mut c_void;
    fn spmc_reader_pop(reader: *mut c_void) -> c_void;
    fn spmc_reader_wait(reader: *mut c_void) -> c_void;
}
#[repr(C)]
struct Test {
    a: c_int,
    b: c_int,
    c: c_int
}
trait MesssageHandler {
    fn on_message(&self, msg: &Test);
}
#[derive(Debug)]
struct Strategy {
}

struct SPMCVarQ {
}

impl SPMCVarQ {
    fn start<T:MesssageHandler>(path: &str, s: &T) {
        let p = CString::new(path).expect("path required");
        let q = unsafe { spmc_connect_shm(p.as_ptr()) };
        let r = unsafe { spmc_get_reader(q) };
        loop {
            unsafe {
                spmc_reader_wait(r);
                let msg = spmc_reader_read(r) ;
                if msg.is_null() {
                    // println!("null");
                    continue;
                }
                // let data: &mut Test = &mut *(msg as *mut Test);
                let data: &Test = std::mem::transmute(msg);
                s.on_message(data);
                spmc_reader_pop(r);
            }
        }
    }
}

impl MesssageHandler for Strategy {
    fn on_message(&self, data: &Test) {
        println!("msg recv {} {} {}", data.a, data.b, data.c);
    }
}

fn main() {
    let s = Strategy{};
    SPMCVarQ::start("/hq_test", &s);
}
  
