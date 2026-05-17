use gfx_backend_vulkan as backend;
use gfx_hal::{adapter::Adapter, Instance};

pub fn get_gpus() -> String {
    let instance: gfx_backend_vulkan::Instance = backend::Instance::create("Graphics", 1).unwrap();
    let adapters: Vec<Adapter<backend::Backend>> = instance.enumerate_adapters();

    let mut names: Vec<String> = Vec::new();

    for adapter in adapters {
        names.push(adapter.info.name.to_string());
    }

    names[0].clone()
}