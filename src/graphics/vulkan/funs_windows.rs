use super::*;

#[link(name = "vulkan-1")]
extern "C" {
    pub fn vkCreateInstance(
        pCreateInfo: *const VkInstanceCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pInstance: *mut VkInstance,
    ) -> VkResult;
    pub fn vkEnumeratePhysicalDevices(
        instance: VkInstance,
        pPhysicalDeviceCount: *mut u32,
        pPhysicalDevices: *mut VkPhysicalDevice,
    ) -> VkResult;
    pub fn vkGetPhysicalDeviceMemoryProperties(
        physicalDevice: VkPhysicalDevice,
        pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties,
    ) -> c_void;
}
