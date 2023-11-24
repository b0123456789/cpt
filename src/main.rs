#[warn(unused_imports)]
use autocxx::prelude::*;
use autocxx::subclass::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

include_cpp! {
    #include "ThostFtdcTraderApi.h"
    #include "ThostFtdcMdApi.h"
    #include "ThostFtdcUserApiStruct.h"
    #include "ThostFtdcUserApiDataType.h"
    safety!(unsafe) // unsafety policy; see docs
}


fn main() {
    println!("Hello, world!");
}
