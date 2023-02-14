use std::any::Any;
pub trait Extension: Any {
    fn name(&self) -> &str;

    fn destroy(&mut self);

    fn as_any_ref(&self) -> &dyn Any;

    fn as_any_ref_mut(&mut self) -> &mut dyn Any;
}

pub trait RegistryExtension: Extension {
    fn register(&self);
}

pub trait ProtocolExtension: Extension {
    fn invoke(&self);
}

pub enum ExtensionEnum {
    Registry(Box<dyn RegistryExtension>),
    Protocol(Box<dyn ProtocolExtension>),
}

impl ExtensionEnum {
    pub fn extension_name(&self) -> String {
        match self {
            ExtensionEnum::Protocol(e) => String::from(e.name()),
            ExtensionEnum::Registry(e) => String::from(e.name()),
        }
    }
}
