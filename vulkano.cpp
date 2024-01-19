#define GLFW_INCLUDE_VULKAN
#include <GLFW/glfw3.h>

#include <string>
#include <iostream>
#include <vector>

const unsigned int WIDTH = 1920;
const unsigned int HEIGHT = 1080;


#ifdef NDEBUG
    const bool enableValidationLayers = true;
    printf("Debug mode activated");
    const std::vector<const char*> validationLayers = {
        "VK_LAYER_KHRONOS_validation"
    };
    
    bool checkValidationLayerSupport() {
        int layerCount;
        vkEnumerateInstanceLayerProperties(&layerCount, nullptr);

        std::vector<VkLayerProperties> availableLayers(layerCount);
        vkEnumerateInstanceLayerProperties(&layerCount, availableLayers.data());

        for (const char* layerName : validationLayers) {
            bool found = false;
            for (const auto& layerProperties : availableLayers) {
                if(strcmp(layerName, layerProperties.layerName) == 0) {
                    found = true;
                    break;
                }
            }
            if(!found) {
                return false;
            }
        }
        return true;
    }
#else 
    const bool enableValidationLayers = false;

    const std::vector<const char*> validationLayers = {};
    bool checkValidationLayerSupport() { return false;}
#endif

int main(void) {
    //setup glfw window
    std::string name("Vulkan uwu");
    glfwInit();

    //setup validation layers for debugging

    glfwWindowHint(GLFW_CLIENT_API, GLFW_NO_API);
    GLFWwindow* window = glfwCreateWindow(WIDTH, HEIGHT, name.c_str(), nullptr, nullptr);

    //init vulkan
    VkInstance vkInstance;
    VkApplicationInfo appInfo{};
    appInfo.sType = VK_STRUCTURE_TYPE_APPLICATION_INFO;
    appInfo.pApplicationName = name.c_str();
    appInfo.applicationVersion = VK_MAKE_VERSION(1, 0, 0);
    //appInfo.pEngineName = "No Engine";
    appInfo.engineVersion = VK_MAKE_VERSION(1, 0, 0);
    appInfo.apiVersion = VK_API_VERSION_1_0;

    VkInstanceCreateInfo createInfo{};
    createInfo.sType = VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO;
    createInfo.pApplicationInfo = &appInfo;
    uint32_t glfwExtensionCount = 0;
    const char** glfwExtensions;
    glfwExtensions = glfwGetRequiredInstanceExtensions(&glfwExtensionCount);

    createInfo.enabledExtensionCount = glfwExtensionCount;
    createInfo.ppEnabledExtensionNames = glfwExtensions;
    if(enableValidationLayers) {
        if(!checkValidationLayerSupport()) {
            std::cerr << "Validation layers requested, but not available" << std::endl;
            return 1;
        }
        createInfo.enabledLayerCount = validationLayers.size();
        createInfo.ppEnabledLayerNames = validationLayers.data();
    } else {
        createInfo.enabledLayerCount = 0;
    }
    
    if(vkCreateInstance(&createInfo, nullptr, &vkInstance) != VK_SUCCESS) {
        std::cerr << "Failed to create Vulkan instance!" << std::endl;
        return 1;
    }

    //pick physical device
    VkPhysicalDevice physicalDevice = VK_NULL_HANDLE;
    uint32_t deviceCount;
    vkEnumeratePhysicalDevices(vkInstance, &deviceCount, nullptr);
    if(deviceCount == 0) {
        std::cerr << "Brutha you need a vulkan GPU" << std::endl;
        return 1;
    }
    printf("%i\n", (int)deviceCount);
    std::vector<VkPhysicalDevice> devices(deviceCount);
    vkEnumeratePhysicalDevices(vkInstance, &deviceCount, devices.data());
    
/*    for(VkPhysicalDevice device : devices) {
        //check for device
        VkPhysicalDeviceProperties physicalDeviceProps;
        vkGetPhysicalDeviceProperties(physicalDevice, &physicalDeviceProps);
        VkPhysicalDeviceFeatures physicalDeviceFeats;
        vkGetPhysicalDeviceFeatures(physicalDevice, &physicalDeviceFeats);

        if(physicalDeviceProps.deviceType == VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU && physicalDeviceFeats.geometryShader) {
            physicalDevice = device;
            break;
        }
    }
    if(physicalDevice == VK_NULL_HANDLE) {
        std::cerr << "No supported GPU found" << std::endl;
        return 1;
    }
*/
    //get device properties

    //main loop
    while (!glfwWindowShouldClose(window)) {
        glfwPollEvents();
    }

    //cleanup
    //vk
    vkDestroyInstance(vkInstance, nullptr);
    //glfw
    glfwDestroyWindow(window);
    glfwTerminate();
}
