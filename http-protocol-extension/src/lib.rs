use std::any::Any;

use extensions::{Extension, ExtensionEnum, ProtocolExtension};

#[repr(C)]
pub struct HttpProtocolExtension;

impl Extension for HttpProtocolExtension {
    fn name(&self) -> &str {
        "HttpProtocolExtension"
    }

    fn destroy(&mut self) {
        println!("invoke the http protocol extension destroy method");
    }

    fn as_any_ref(&self) -> &dyn std::any::Any {
        self as &dyn Any
    }

    fn as_any_ref_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn Any
    }
}

impl ProtocolExtension for HttpProtocolExtension {
    fn invoke(&self) {
        println!("invoke HttpProtocolExtension method")
    }
}

#[no_mangle]
pub extern "C" fn _dubbo_extension() -> *mut ExtensionEnum {
    let extension = Box::new(HttpProtocolExtension);
    let extension = ExtensionEnum::Protocol(extension);
    Box::into_raw(Box::new(extension))
}
