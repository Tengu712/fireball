use super::*;

#[allow(non_camel_case_types)]
pub type PFN_vkAllocationFunction = *const c_void; // !
#[allow(non_camel_case_types)]
pub type PFN_vkFreeFunction = *const c_void; // !
#[allow(non_camel_case_types)]
pub type PFN_vkInternalAllocationNotification = *const c_void; // !
#[allow(non_camel_case_types)]
pub type PFN_vkInternalFreeNotification = *const c_void; // !
#[allow(non_camel_case_types)]
pub type PFN_vkReallocationFunction = *const c_void; // !
pub type VkBool32 = u32;
/// Incomplete type. `typedef struct VkCommandPool_T* VkCommandPool;`
#[cfg(target_pointer_width = "64")]
pub type VkCommandPool = *const c_void;
#[cfg(target_pointer_width = "32")]
pub type VkCommandPool = u64;
pub type VkCommandPoolCreateFlags = VkFlags;
/// Incomplete type. `typedef struct VkDevice_T* VkDevice;`
pub type VkDevice = *const c_void;
pub type VkDeviceCreateFlags = VkFlags;
pub type VkDeviceQueueCreateFlags = VkFlags;
pub type VkDeviceSize = u64;
pub type VkFlags = u32;
/// Incomplete type. `typedef struct VkInstance_T* VkInstance;`
pub type VkInstance = *const c_void;
pub type VkInstanceCreateFlags = VkFlags;
pub type VkMemoryHeapFlags = VkFlags;
pub type VkMemoryPropertyFlags = VkFlags;
/// Incomplete type. `typedef struct VkPhysicalDevice_T* VkPhysicalDevice;`
pub type VkPhysicalDevice = *const c_void;
pub type VkQueueFlags = VkFlags;
pub type VkResult = i32;
pub type VkStructureType = u32;

pub const VK_API_VERSION_1_1: u32 = VK_MAKE_API_VERSION(0, 1, 1, 0);
pub const VK_MAX_EXTENSION_NAME_SIZE: usize = 256;
pub const VK_MAX_MEMORY_HEAPS: usize = 16;
pub const VK_STRUCTURE_TYPE_APPLICATION_INFO: VkStructureType = 0;
pub const VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO: VkStructureType = 1;
pub const VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO: VkStructureType = 2;
pub const VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO: VkStructureType = 3;
pub const VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO: VkStructureType = 39;
pub const VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT: VkCommandPoolCreateFlags = 2;
pub const VK_SUCCESS: VkResult = 0;
pub const VK_QUEUE_GRAPHICS_BIT: u32 = 1;
#[allow(non_snake_case)]
pub const fn VK_MAKE_API_VERSION(variant: u32, major: u32, minor: u32, patch: u32) -> u32 {
    (variant << 29) | (major << 22) | (minor << 12) | (patch)
}
#[allow(non_snake_case)]
pub const fn VK_MAKE_VERSION(major: u32, minor: u32, patch: u32) -> u32 {
    (major << 22) | (minor << 12) | (patch)
}
