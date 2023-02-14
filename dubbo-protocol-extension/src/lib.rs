use std::any::Any;

use extensions::{Extension, ExtensionEnum, ProtocolExtension};

#[repr(C)]
pub struct DubooProtocolExtension;

impl DubooProtocolExtension {
    pub fn connect(&self) {
        println!("dubbo protocol is connecting")
    }
}

impl Extension for DubooProtocolExtension {
    fn name(&self) -> &str {
        "DubooProtocolExtension"
    }

    fn destroy(&mut self) {
        println!("invoke the dubbo registry extension destroy method");
    }

    fn as_any_ref(&self) -> &dyn std::any::Any {
        self as &dyn Any
    }

    fn as_any_ref_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn Any
    }
}

impl ProtocolExtension for DubooProtocolExtension {
    fn invoke(&self) {
        println!("invoke DubooProtocolExtension method")
    }
}

#[no_mangle]
pub extern "C" fn _dubbo_extension() -> *mut ExtensionEnum {
    let extension = Box::new(DubooProtocolExtension);
    let extension = ExtensionEnum::Protocol(extension);
    Box::into_raw(Box::new(extension))
}
