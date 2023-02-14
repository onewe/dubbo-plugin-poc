use std::any::Any;

use extensions::{Extension, ExtensionEnum, RegistryExtension};

#[repr(C)]
pub struct ZookeeperRegistryExtension;

impl Extension for ZookeeperRegistryExtension {
    fn name(&self) -> &str {
        "ZookeeperRegistryExtension"
    }

    fn destroy(&mut self) {
        println!("invoke the zookeeper registry extension destroy method");
    }

    fn as_any_ref(&self) -> &dyn std::any::Any {
        self as &dyn Any
    }

    fn as_any_ref_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn Any
    }
}

impl RegistryExtension for ZookeeperRegistryExtension {
    fn register(&self) {
        println!("invoke NacosRegistryExtension method")
    }
}

#[no_mangle]
pub extern "C" fn _dubbo_extension() -> *mut ExtensionEnum {
    let extension = Box::new(ZookeeperRegistryExtension);
    let extension = ExtensionEnum::Registry(extension);
    Box::into_raw(Box::new(extension))
}
