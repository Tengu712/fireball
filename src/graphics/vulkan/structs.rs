use super::*;

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkAllocationCallbacks {
    pub pUserData: *const c_void,
    pub pfnAllocation: PFN_vkAllocationFunction,
    pub pfnReallocation: PFN_vkReallocationFunction,
    pub pfnFree: PFN_vkFreeFunction,
    pub pfnInternalAllocation: PFN_vkInternalAllocationNotification,
    pub pfnInternalFree: PFN_vkInternalFreeNotification,
}
#[repr(C)]
#[allow(non_snake_case)]
pub struct VkApplicationInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub pApplicationName: *const c_char,
    pub applicationVersion: u32,
    pub pEngineName: *const c_char,
    pub engineVersion: u32,
    pub apiVersion: u32,
}
#[repr(C)]
#[allow(non_snake_case)]
pub struct VkMemoryHeap {
    pub size: VkDeviceSize,
    pub flags: VkMemoryHeapFlags,
}
#[repr(C)]
#[allow(non_snake_case)]
pub struct VkMemoryType {
    pub propertyFlags: VkMemoryPropertyFlags,
    pub heapIndex: u32,
}
#[repr(C)]
#[allow(non_snake_case)]
pub struct VkInstanceCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkInstanceCreateFlags,
    pub pApplicationInfo: *const VkApplicationInfo,
    pub enabledLayerCount: u32,
    pub ppEnabledLayerNames: *const *const c_char,
    pub enabledExtensionCount: u32,
    pub ppEnabledExtensionNames: *const *const c_char,
}
#[repr(C)]
#[allow(non_snake_case)]
pub struct VkPhysicalDeviceMemoryProperties {
    pub memoryTypeCount: u32,
    pub memoryTypes: [VkMemoryType; VK_MAX_MEMORY_HEAPS],
    pub memoryHeapCount: u32,
    pub memoryHeaps: [VkMemoryHeap; VK_MAX_MEMORY_HEAPS],
}
