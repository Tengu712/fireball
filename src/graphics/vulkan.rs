#[cfg(target_os = "linux")]
mod funs_linux;
#[cfg(target_os = "windows")]
mod funs_windows;
mod structs;
mod type_const;

#[cfg(target_os = "linux")]
use funs_linux::*;
#[cfg(target_os = "windows")]
use funs_windows::*;
use structs::*;
use type_const::*;

use std::ffi::CString;
use std::os::raw::*;

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
        apiVersion: VK_API_VERSION_1_1,
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
