#![allow(unused)]

use std::ffi::{c_double, c_float};

use autocxx::prelude::*;
use cxx::CxxString;
use defer_lite::defer;
use std::pin::Pin;

include_cpp! {
    #include "myclass.h"
    safety!(unsafe_ffi)
    generate!("MyClass_new")
    generate!("MyClass_delete")
    generate!("MyClass_myFunction")
    generate!("MyClass_Point")
    generate!("MyClass_roar")
}

#[autocxx::extern_rust::extern_rust_type]
pub struct Dinosaur {
    carnivore: bool,
}

impl Dinosaur {
    #[autocxx::extern_rust::extern_rust_function]
    fn roar(&self) {
        println!("Roar\n");
    }

    #[autocxx::extern_rust::extern_rust_function]
    fn eat(&self, value: Box<Value>) {
        println!(
            "{}:{},{},{}",
            value.as_ref().is_ok,
            value.as_ref().intval,
            value.as_ref().floatval,
            value.as_ref().strs
        );
    }
}

#[repr(C)]
#[autocxx::extern_rust::extern_rust_type]
pub struct Value {
    is_ok: bool,
    intval: i32,
    floatval: f32,
    strs: String,
}

#[autocxx::extern_rust::extern_rust_function]
pub fn new_value(is_ok: bool, intval: i32, floatval: f32, strs: String) -> Box<Value> {
    Box::new(Value {
        is_ok,
        intval,
        floatval,
        strs,
    })
}

#[autocxx::extern_rust::extern_rust_function]
fn print_value(value: Box<Value>) {
    println!(
        "{}:{},{},{}",
        value.as_ref().is_ok,
        value.as_ref().intval,
        value.as_ref().floatval,
        value.as_ref().strs
    );
}

fn main() {
    let d: Dinosaur = Dinosaur { carnivore: false };
    let mclass = unsafe { ffi::MyClass_new(&d as *const Dinosaur as *mut Dinosaur) };
    defer! {
        unsafe {
            ffi::MyClass_delete(mclass )
        };
    }

    let rs = unsafe { ffi::MyClass_myFunction(mclass, c_int(1), c_int(1000)) };

    print!("{}\n", rs.0.to_owned());

    unsafe {
        ffi::MyClass_roar(mclass);
        ffi::MyClass_Point(mclass);
    }
}
