use std::{collections::HashMap, env};

use extensions::ExtensionEnum;

pub struct ExtensionLoader {
    extensions: &'static mut HashMap<String, Box<ExtensionEnum>>,
}

impl ExtensionLoader {
    pub fn new() -> Self {
        let extensions = Self::load_all_extension();
        let extensions = Box::leak(Box::new(extensions));

        Self { extensions }
    }

    fn load_all_extension() -> HashMap<String, Box<ExtensionEnum>> {
        let mut extensions = HashMap::default();

        let mut path = env::current_dir().unwrap();
        path.pop();

        let parent_path = path.to_str().unwrap();

        let dubbo_protocol_extension = format!(
            "{}{}",
            parent_path, "/target/debug/libdubbo_protocol_extension.dylib"
        );
        let http_protocol_extension = format!(
            "{}{}",
            parent_path, "/target/debug/libhttp_protocol_extension.dylib"
        );
        let nacos_registry_extension = format!(
            "{}{}",
            parent_path, "/target/debug/libnacos_registry_extension.dylib"
        );
        let zookeeper_registry_extension = format!(
            "{}{}",
            parent_path, "/target/debug/libzookeeper_registry_extension.dylib"
        );

        let path_vec = vec![
            dubbo_protocol_extension,
            http_protocol_extension,
            nacos_registry_extension,
            zookeeper_registry_extension,
        ];

        for path in path_vec {
            let extension = unsafe {
                let lib = libloading::Library::new(path).unwrap();
                let func: libloading::Symbol<unsafe extern "C" fn() -> *mut ExtensionEnum> =
                    lib.get(b"_dubbo_extension").unwrap();
                let extension_ptr = func();
                let extension = Box::from_raw(extension_ptr);
                let extension_name = extension.extension_name();
                (extension, extension_name)
            };

            extensions.insert(extension.1, extension.0);
        }

        extensions
    }

    pub fn get_extension<T: 'static>(&self, name: &str) -> Option<&T> {
        let e = self.extensions.get(name).map(|e| match e.as_ref() {
            ExtensionEnum::Protocol(e) => {
                let any = e.as_any_ref();
                any.downcast_ref::<T>()
            }
            ExtensionEnum::Registry(e) => {
                let any = e.as_any_ref();
                any.downcast_ref::<T>()
            }
        });

        e?
    }

    pub fn foreach(&self, func: fn(&String, &Box<ExtensionEnum>)) {
        for (k, v) in self.extensions.iter() {
            func(k, v);
        }
    }
}

#[cfg(test)]
mod tests {
    use dubbo_protocol_extension::DubooProtocolExtension;

    use crate::ExtensionLoader;

    #[test]
    fn test_loader() {
        let loader = ExtensionLoader::new();
        loader.foreach(|k, v| {
            println!("extension name: {}", k);
            let v = v.as_ref();
            match v {
                extensions::ExtensionEnum::Protocol(e) => e.invoke(),
                extensions::ExtensionEnum::Registry(e) => e.register(),
            }
        });

        let extension = loader
            .get_extension::<DubooProtocolExtension>("DubooProtocolExtension")
            .unwrap();
        extension.connect();
    }
}
