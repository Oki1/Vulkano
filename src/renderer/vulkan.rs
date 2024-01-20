use std::sync::Arc;
use vulkano::{
    instance::{Instance, InstanceCreateInfo, InstanceExtensions},
    library::VulkanLibrary,
    Version,
};


fn checkValidationLayers(library: Option<&VulkanLibrary>) -> bool{
    const LAYERS: [&str;1] = ["VK_LAYER_KHRONOS_validation"];

    //check if all layers in LAYERS are available in the vulkan installation
    if library.is_some(){
        let mut allLayers = library.unwrap().layer_properties().unwrap();
        for layer in LAYERS {
            println!("Layer name: {}", layer);
            if !allLayers.any(
                |lay| lay.name() == layer
                ) {
                return false
            }
        }
        return true
    }
    false
}

pub struct Vulkan {
    // vulkan library instance
    library: Arc<VulkanLibrary>,
    instance: Arc<Instance>
}
impl Vulkan {

    pub fn new(app_name: String) -> Vulkan {
        let library = VulkanLibrary::new().expect("no local vulkan dll!");
        if cfg!(debug_assertions){
            let validated: bool = self::checkValidationLayers(Some(&library));
            println!("{}", validated);
        }
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
