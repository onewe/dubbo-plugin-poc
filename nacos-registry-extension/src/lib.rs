use std::any::Any;

use extensions::{Extension, ExtensionEnum, RegistryExtension};

#[repr(C)]
pub struct NacosRegistryExtension;

impl Extension for NacosRegistryExtension {
    fn name(&self) -> &str {
        "NacosRegistryExtension"
    }

    fn destroy(&mut self) {
        println!("invoke the nacos registry extension destroy method");
    }

    fn as_any_ref(&self) -> &dyn std::any::Any {
        self as &dyn Any
    }

    fn as_any_ref_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn Any
    }
}

impl RegistryExtension for NacosRegistryExtension {
    fn register(&self) {
        println!("invoke NacosRegistryExtension method")
    }
}

#[no_mangle]
pub extern "C" fn _dubbo_extension() -> *mut ExtensionEnum {
    let extension = Box::new(NacosRegistryExtension);
    let extension = ExtensionEnum::Registry(extension);
    Box::into_raw(Box::new(extension))
}
