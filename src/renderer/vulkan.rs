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
    pub fn new(app_name: String) -> Vulkan {
        let library = VulkanLibrary::new().expect("no local vulkan dll!");
        Vulkan {
            library: library.clone(),
            instance: Instance::new(
                library,
                //todo: add validation layers
                InstanceCreateInfo{
                    application_name: Some(app_name),
                    ..Default::default()
                },
            ).expect("Instance creation failed"),
        }
    }
}
