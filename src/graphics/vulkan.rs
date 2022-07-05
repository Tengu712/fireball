use std::ffi::CString;
use std::os::raw::*;

#[allow(non_camel_case_types)]
type PFN_vkAllocationFunction = *const c_void; // !
#[allow(non_camel_case_types)]
type PFN_vkFreeFunction = *const c_void; // !
#[allow(non_camel_case_types)]
type PFN_vkInternalAllocationNotification = *const c_void; // !
#[allow(non_camel_case_types)]
type PFN_vkInternalFreeNotification = *const c_void; // !
#[allow(non_camel_case_types)]
type PFN_vkReallocationFunction = *const c_void; // !
type VkDeviceSize = u64;
type VkFlags = u32;
/// Incomplete type. `typedef struct VkInstance_T* VkInstance;`
type VkInstance = *const c_void;
type VkInstanceCreateFlags = VkFlags;
type VkMemoryHeapFlags = VkFlags;
type VkMemoryPropertyFlags = VkFlags;
/// Incomplete type. `typedef struct VkPhysicalDevice_T* VkPhysicalDevice;`
type VkPhysicalDevice = *const c_void;
type VkResult = i32;
type VkStructureType = u32;

const VK_MAX_MEMORY_HEAPS: usize = 16;
const VK_STRUCTURE_TYPE_APPLICATION_INFO: VkStructureType = 0;
const VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO: VkStructureType = 1;
const VK_SUCCESS: VkResult = 0;

#[allow(non_snake_case)]
fn VK_API_VERSION_1_1() -> u32 {
    VK_MAKE_API_VERSION(0, 1, 1, 0)
}
#[allow(non_snake_case)]
fn VK_MAKE_API_VERSION(variant: u32, major: u32, minor: u32, patch: u32) -> u32 {
    (variant << 29) | (major << 22) | (minor << 12) | (patch)
}
#[allow(non_snake_case)]
fn VK_MAKE_VERSION(major: u32, minor: u32, patch: u32) -> u32 {
    (major << 22) | (minor << 12) | (patch)
}

#[repr(C)]
#[allow(non_snake_case)]
struct VkAllocationCallbacks {
    pUserData: *const c_void,
    pfnAllocation: PFN_vkAllocationFunction,
    pfnReallocation: PFN_vkReallocationFunction,
    pfnFree: PFN_vkFreeFunction,
    pfnInternalAllocation: PFN_vkInternalAllocationNotification,
    pfnInternalFree: PFN_vkInternalFreeNotification,
}
#[repr(C)]
#[allow(non_snake_case)]
struct VkApplicationInfo {
    sType: VkStructureType,
    pNext: *const c_void,
    pApplicationName: *const c_char,
    applicationVersion: u32,
    pEngineName: *const c_char,
    engineVersion: u32,
    apiVersion: u32,
}
#[repr(C)]
#[allow(non_snake_case)]
struct VkMemoryHeap {
    size: VkDeviceSize,
    flags: VkMemoryHeapFlags,
}
#[repr(C)]
#[allow(non_snake_case)]
struct VkMemoryType {
    propertyFlags: VkMemoryPropertyFlags,
    heapIndex: u32,
}
#[repr(C)]
#[allow(non_snake_case)]
struct VkInstanceCreateInfo {
    sType: VkStructureType,
    pNext: *const c_void,
    flags: VkInstanceCreateFlags,
    pApplicationInfo: *const VkApplicationInfo,
    enabledLayerCount: u32,
    ppEnabledLayerNames: *const *const c_char,
    enabledExtensionCount: u32,
    ppEnabledExtensionNames: *const *const c_char,
}
#[repr(C)]
#[allow(non_snake_case)]
struct VkPhysicalDeviceMemoryProperties {
    memoryTypeCount: u32,
    memoryTypes: [VkMemoryType; VK_MAX_MEMORY_HEAPS],
    memoryHeapCount: u32,
    memoryHeaps: [VkMemoryHeap; VK_MAX_MEMORY_HEAPS],
}

#[link(name = "vulkan-1")]
extern "C" {
    fn vkCreateInstance(
        pCreateInfo: *const VkInstanceCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pInstance: *mut VkInstance,
    ) -> VkResult;
    fn vkEnumeratePhysicalDevices(
        instance: VkInstance,
        pPhysicalDeviceCount: *mut u32,
        pPhysicalDevices: *mut VkPhysicalDevice,
    ) -> VkResult;
    fn vkGetPhysicalDeviceMemoryProperties(
        physicalDevice: VkPhysicalDevice,
        pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties,
    ) -> c_void;
}

pub struct Vulkan;
impl Vulkan {
    pub fn new(appname: &'static str) -> Self {
        let instance = create_instance(appname);
        let (_, _) = select_physical_device(&instance);
        Self
    }
}
fn create_instance(appname: &'static str) -> VkInstance {
    let appname_cstr = CString::new(appname).unwrap();
    let app_info = VkApplicationInfo {
        sType: VK_STRUCTURE_TYPE_APPLICATION_INFO,
        pNext: std::ptr::null(),
        pApplicationName: appname_cstr.as_ptr(),
        applicationVersion: 0,
        pEngineName: appname_cstr.as_ptr(),
        engineVersion: VK_MAKE_VERSION(1, 0, 0),
        apiVersion: VK_API_VERSION_1_1(),
    };
    let create_info = VkInstanceCreateInfo {
        sType: VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
        pNext: std::ptr::null(),
        flags: 0,
        pApplicationInfo: &app_info,
        enabledLayerCount: 0,
        ppEnabledLayerNames: std::ptr::null(),
        enabledExtensionCount: 0,
        ppEnabledExtensionNames: std::ptr::null(),
    };
    let mut instance = std::ptr::null();
    let res = unsafe { vkCreateInstance(&create_info, std::ptr::null(), &mut instance) };
    assert!(
        res == VK_SUCCESS,
        "[fatal error] failed to create vulkan instance. : {}",
        res
    );
    instance
}
fn select_physical_device(
    instance: &VkInstance,
) -> (VkPhysicalDevice, VkPhysicalDeviceMemoryProperties) {
    let mut cnt = 0;
    let res = unsafe { vkEnumeratePhysicalDevices(*instance, &mut cnt, std::ptr::null_mut()) };
    assert!(
        res == VK_SUCCESS,
        "[fatal error] failed to get the number of physical devices. : {}",
        res
    );
    let devices = unsafe {
        std::alloc::alloc(std::alloc::Layout::new::<VkPhysicalDevice>()) as *mut VkPhysicalDevice
    };
    let res = unsafe { vkEnumeratePhysicalDevices(*instance, &mut cnt, devices) };
    assert!(
        res == VK_SUCCESS,
        "[fatal error] failed to enumerate physical devices. : {}",
        res
    );
    let device = unsafe { *devices };
    let mut props = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
    unsafe { vkGetPhysicalDeviceMemoryProperties(device, &mut props) };
    (device, props)
}
