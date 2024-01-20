use std::sync::Arc;
use vulkano::{
    instance::{Instance, InstanceCreateInfo, InstanceExtensions},
    library::VulkanLibrary,
    swapchain::Surface,
    Version,
};
use winit::window::Window;

fn checkValidationLayers(library: Arc<VulkanLibrary>, layers: &Vec<String>) -> Result<(), String> {
    //check if all layers in LAYERS are available in the vulkan installation
    let mut allLayers = library.layer_properties().unwrap();
    for layer in layers {
        if !allLayers.any(|lay| lay.name() == layer) {
            return Err(format!(
                "Validation layer \"{}\" could not be found.",
                layer
            ));
        }
    }
    return Ok(());
}

pub struct Vulkan {
    // vulkan library instance
    library: Arc<VulkanLibrary>,
    instance: Arc<Instance>,
    surface: Arc<Surface>,
}
impl Vulkan {
    pub fn new(app_name: String, handle: Arc<Window>) -> Vulkan {
        let library = VulkanLibrary::new().expect("no local vulkan dll!");


        //get required instance extensions
        let mut extensions = Surface::required_extensions(&handle); 
        
        let mut createInfo = InstanceCreateInfo {
            application_name: Some(app_name),
            ..Default::default()
        };

        // if debug mode is activated add required validation layers
        if cfg!(debug_assertions) {
            let validationLayers: Vec<String> = vec!["VK_LAYER_KHRONOS_validation".to_string()];
            checkValidationLayers(library.clone(), &validationLayers).unwrap();
            extensions.ext_debug_utils = true;
            createInfo.enabled_layers = validationLayers;
        }
        createInfo.enabled_extensions = extensions;
        
        let instance = Instance::new(
                    library.clone(), //todo: add validation layers
                    createInfo,
                ).expect("Instance creation failed");
            
        Vulkan {
            library: library.clone(),
            instance: instance.clone(),
            surface: Surface::from_window(instance, handle).expect("Surface creation failed!"),
        }
    }
}
