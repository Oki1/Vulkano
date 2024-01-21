use std::sync::Arc;
use vulkano::instance::debug::{
    DebugUtilsMessenger, DebugUtilsMessengerCallback, DebugUtilsMessengerCreateInfo,
};
use vulkano::{
    device::physical::{PhysicalDevice, PhysicalDeviceType},
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
                "Validation layer \"{}\" could not be found. Typo or no Vulkan SDK?",
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
    device: Arc<PhysicalDevice>,
    //the debug messenger that is called when a validation layer returns an error
    //It can also be called by other vulkan things idk
    #[cfg(debug_assertions)]
    _debugMessengerCallback: DebugUtilsMessenger,
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
        )
        .expect("Instance creation failed");

        //debug messenger
        //is safe unless the callback makes calls to the vulkan api (which it doesnt)

        #[cfg(debug_assertions)]
        let _debugMessengerCallback = unsafe {
            DebugUtilsMessenger::new(
                instance.clone(),
                DebugUtilsMessengerCreateInfo::user_callback(DebugUtilsMessengerCallback::new(
                    |mSeverity, mType, callbackData| {
                        println!("Debug callback: {:?}", callbackData.message);
                    },
                )),
            )
            .expect("messenger creation failed!")
        };

        //pick physical device
        let device: Arc<PhysicalDevice>;
        {
            let suitable: Vec<Arc<PhysicalDevice>> = instance
                .enumerate_physical_devices()
                .expect("Physical devices could not be enumerated!")
                .filter(|d| -> bool {
                    //MUST HAVES!
                    true
                }).collect();
        //    device = suitable[0].clone();
            //disgusting but not knowledgable enoug in rust 
            device = suitable[suitable.iter().position(|d| -> bool {
                //NICE TO HAVES
                matches!(d.properties().device_type, PhysicalDeviceType::DiscreteGpu)
            }).unwrap_or(0)].clone();
        }
        #[cfg(debug_assertions)]
        println!("Using device \"{}\"", device.properties().device_name);

        Vulkan {
            library: library.clone(),
            instance: instance.clone(),
            surface: Surface::from_window(instance, handle).expect("Surface creation failed!"),
            device: device.clone(),
            #[cfg(debug_assertions)]
            _debugMessengerCallback,
        }
    }
}
