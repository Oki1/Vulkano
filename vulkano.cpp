#define GLFW_INCLUDE_VULKAN
#include <GLFW/glfw3.h>

#include <string>
#include <iostream>
#include <vector>

int main(void) {
    //setup glfw window
    std::string name("Vulkan uwu");
    const unsigned int WIDTH = 1920;
    const unsigned int HEIGHT = 1080;
    glfwInit();

    //setup validation layers for debugging
    const std::vector<const char*> validationLayers = {
        "VK_LAYER_KHRONOS_validation"
    };
    #ifdef NDEBUG
        const bool enableValidationLayers = false;
    #else
        std::cout << "Debug mode detected, validation layers enabled\n";
        const bool enableValidationLayers = true;
    #endif

    glfwWindowHint(GLFW_CLIENT_API, GLFW_NO_API);
    GLFWwindow* window = glfwCreateWindow(WIDTH, HEIGHT, name.c_str(), nullptr, nullptr);

    //init vulkan
    VkInstance vulkanInstance;
    VkApplicationInfo appInfo {
        VK_STRUCTURE_TYPE_APPLICATION_INFO,
        nullptr,
        name.c_str(),
        VK_MAKE_VERSION(0,0,1),
        "No engine",
        VK_MAKE_VERSION(1,0,0),
        VK_API_VERSION_1_0
    };

    VkInstanceCreateInfo createInfo;
    createInfo.sType = VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO;
    createInfo.pApplicationInfo = &appInfo;
    uint32_t glfwExtensionCount = 0;
    const char** glfwExtensions;
    glfwExtensions = glfwGetRequiredInstanceExtensions(&glfwExtensionCount);
    createInfo.enabledExtensionCount = glfwExtensionCount;
    createInfo.ppEnabledLayerNames = glfwExtensions;
    createInfo.enabledLayerCount = 0;
    if(vkCreateInstance(&createInfo, nullptr, &vulkanInstance) != VK_SUCCESS) {
        std::cerr << "Failed to create Vulkan instance!" << std::endl;
        return 1;
    }

    //main loop
    while (!glfwWindowShouldClose(window)) {
        glfwPollEvents();
    }

    //cleanup
    //vk
    vkDestroyInstance(vulkanInstance, nullptr);
    //glfw
    glfwDestroyWindow(window);
    glfwTerminate();
}
