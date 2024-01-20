#[allow(non_snake_case)]
use std::sync::Arc;
use vulkano::{
    instance::{Instance, InstanceCreateInfo, InstanceExtensions},
    library::VulkanLibrary,
    Version,
};
pub struct Vulkan {
    // vulkan library instance
    library: Arc<VulkanLibrary>,
    instance: Arc<Instance>,
}
impl Vulkan {
    pub fn new() -> Vulkan {
        let library = VulkanLibrary::new().expect("no local vulkan dll!");
        Vulkan {
            library: library.clone(),
            instance: Instance::new(
                library, 
                InstanceCreateInfo::application_from_cargo_toml()
            ).expect("Instance creation failed"),
        }
    }
}
