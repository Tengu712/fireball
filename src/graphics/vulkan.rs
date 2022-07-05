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
type VkFlags = u32;
/// Incomplete type. `typedef struct VkInstance_T* VkInstance;`
type VkInstance = *const c_void;
type VkInstanceCreateFlags = VkFlags;
type VkResult = i32;
type VkStructureType = u32;

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

#[link(name = "vulkan-1")]
extern "C" {
    fn vkCreateInstance(
        pCreateInfo: *const VkInstanceCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pInstance: *mut VkInstance,
    ) -> VkResult;
}

pub struct Vulkan;
impl Vulkan {
    pub fn new(appname: &'static str) -> Self {
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
        if res != VK_SUCCESS {
            panic!("[fatal error] failed to create vulkan instance. : {}", res);
        }
        Self
    }
}
