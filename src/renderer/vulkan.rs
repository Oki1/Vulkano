use std::sync::Arc;
use vulkano::device::DeviceExtensions;
use vulkano::instance::debug::{
    DebugUtilsMessenger, DebugUtilsMessengerCallback, DebugUtilsMessengerCreateInfo,
};
use vulkano::{
    device::{
        physical::{PhysicalDevice, PhysicalDeviceType},
        Device, DeviceCreateInfo, Features, Queue, QueueCreateInfo, QueueFlags,
    },
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
    physicalDevice: Arc<PhysicalDevice>,
    //the debug messenger that is called when a validation layer returns an error
    //It can also be called by other vulkan things idk
    #[cfg(debug_assertions)]
    _debugMessengerCallback: DebugUtilsMessenger,
    queues: Vec<Arc<Queue>>,
}
impl Vulkan {
    pub fn new(app_name: String, handle: Arc<Window>) -> Vulkan {
        let library = VulkanLibrary::new().expect("no local vulkan dll!");

        //get required instance extensions
        let mut extensions = Surface::required_extensions(&handle);
        extensions.khr_surface = true;

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

        //surface creation
        let surface: Arc<Surface> =
            Surface::from_window(instance.clone(), handle).expect("Surface creation failed!");

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

        //create surface
        let surface: Arc<Surface>= Surface::from_window(instance.clone(), handle).expect("Surface creation failed!");

        //pick physical device
        let physicalDevice: Arc<PhysicalDevice>;
        {
            let suitable: Vec<Arc<PhysicalDevice>> = instance.clone()
                .enumerate_physical_devices()
                .expect("Physical devices could not be enumerated!")
                .filter(|pDevice| -> bool {
                    //MUST HAVES!
                    //check physical device extension support (if we have no swapchain support we
                    //cant render to the screen. Ex. server graphics cards)
                    let exts = pDevice.supported_extensions();
                    if !exts.khr_swapchain {
                        return false
                    }

                    pDevice.queue_family_properties()
                        //filter based on queue families
                        .iter().enumerate().any(|(index, family)| {
                            family.queue_flags.contains(QueueFlags::GRAPHICS) && 
                                pDevice.surface_support(index.try_into().unwrap(), &surface).unwrap_or_else(
                                    |_| panic!("Queue family {} (Physical device '{}') surface support lookup failed!", index, pDevice.properties().device_name))
                        })
                        // .any(|(index, family)| family.queue_flags.contains(QueueFlags::GRAPHICS))
                })
                .collect();
            assert!(!suitable.is_empty(), "No suitable physical devices found.");
            //disgusting but not knowledgable enoug in rust
            physicalDevice = suitable[suitable
                .iter()
                .position(|pDevice| -> bool {
                    //NICE TO HAVES
                    matches!(pDevice.properties().device_type, PhysicalDeviceType::DiscreteGpu)
                })
                .unwrap_or(0)]
            .clone();
            #[cfg(debug_assertions)]
            println!(
                "Using device \"{}\"",
                physicalDevice.properties().device_name
            );
        }

        //pick queue family in chosen physical device
        let qFamIndex: u32 = physicalDevice
            .queue_family_properties()
            .iter()
            .position(|qf| qf.queue_flags.contains(QueueFlags::GRAPHICS))
            .unwrap()
            .try_into()
            .unwrap();

        //create logical device
        let device: Arc<Device>;
        let queues: Vec<Arc<Queue>>;
        {
            let qIt;
            (device, qIt) = Device::new(
                physicalDevice.clone(),
                DeviceCreateInfo {
                    queue_create_infos: vec![QueueCreateInfo {
                        queue_family_index: qFamIndex,
                        ..Default::default()
                    }],
                    //currently device features are empty. Will add features once i need them
                    enabled_extensions: DeviceExtensions::empty(),
                    enabled_features: Features::empty(),
                    ..Default::default()
                },
            )
            .expect("Device creation failed");
            queues = qIt.collect();
        }

        Vulkan {
            library: library.clone(),
            instance: instance.clone(),
            surface: surface.clone(),
            physicalDevice: physicalDevice.clone(),
            queues,
            #[cfg(debug_assertions)]
            _debugMessengerCallback,
        }
    }
}
