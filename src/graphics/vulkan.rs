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
        let (physical_device, _) = select_physical_device(&instance);
        let queue_family_index = get_device_queue_index(&physical_device);
        let logical_device = create_logical_device(&physical_device, queue_family_index);
        let _ = get_device_queue(&logical_device, queue_family_index);
        let _ = create_command_pool(&logical_device, queue_family_index);
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
    let mut cnt = 0;
    let res = unsafe {
        vkEnumerateInstanceExtensionProperties(std::ptr::null(), &mut cnt, std::ptr::null_mut())
    };
    check(res, "get the number of instance extension props");
    let mut props = Vec::with_capacity(cnt as usize);
    let res = unsafe {
        vkEnumerateInstanceExtensionProperties(std::ptr::null(), &mut cnt, props.as_mut_ptr())
    };
    check(res, "enumerate instance extension props");
    let mut extensions = Vec::with_capacity(cnt as usize);
    for i in 0..cnt {
        extensions.push(unsafe { (*props.get_unchecked(i as usize)).extensionName.as_ptr() });
    }
    #[cfg(not(debug_assertions))]
    let layers = [];
    #[cfg(debug_assertions)]
    let layers = ["VK_LAYER_KHRONOS_validation\0".as_ptr() as *const c_char];
    let create_info = VkInstanceCreateInfo {
        sType: VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
        pNext: std::ptr::null(),
        flags: 0,
        pApplicationInfo: &app_info,
        enabledLayerCount: layers.len() as u32,
        ppEnabledLayerNames: layers.as_ptr(),
        enabledExtensionCount: extensions.len() as u32,
        ppEnabledExtensionNames: extensions.as_ptr(),
    };
    let mut instance = std::ptr::null();
    let res = unsafe { vkCreateInstance(&create_info, std::ptr::null(), &mut instance) };
    check(res, "create vulkan instance");
    instance
}
fn select_physical_device(
    instance: &VkInstance,
) -> (VkPhysicalDevice, VkPhysicalDeviceMemoryProperties) {
    let mut cnt = 0;
    let res = unsafe { vkEnumeratePhysicalDevices(*instance, &mut cnt, std::ptr::null_mut()) };
    check(res, "get the number of physical devices");
    let mut devices = Vec::with_capacity(cnt as usize);
    let res = unsafe { vkEnumeratePhysicalDevices(*instance, &mut cnt, devices.as_mut_ptr()) };
    check(res, "enumerate physical devices");
    let device = unsafe { devices.get_unchecked(0) };
    let mut props = unsafe { std::mem::MaybeUninit::zeroed().assume_init() }; // !
    unsafe { vkGetPhysicalDeviceMemoryProperties(*device, &mut props) };
    (*device, props)
}
fn get_device_queue_index(physical_device: &VkPhysicalDevice) -> u32 {
    let mut cnt = 0;
    unsafe {
        vkGetPhysicalDeviceQueueFamilyProperties(*physical_device, &mut cnt, std::ptr::null_mut())
    };
    let mut props = Vec::with_capacity(cnt as usize);
    unsafe {
        vkGetPhysicalDeviceQueueFamilyProperties(*physical_device, &mut cnt, props.as_mut_ptr())
    };
    let mut queue = cnt;
    for i in 0..cnt {
        if unsafe { ((*props.get_unchecked(i as usize)).queueFlags & VK_QUEUE_GRAPHICS_BIT) > 0 } {
            queue = i;
            break;
        }
    }
    assert!(
        queue != cnt,
        "[fatal error] failed to enumerate device queue index.",
    );
    queue
}
fn create_logical_device(physical_device: &VkPhysicalDevice, queue_family_index: u32) -> VkDevice {
    let default_queue_priority = 1.0;
    let queue_create_info = VkDeviceQueueCreateInfo {
        sType: VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
        pNext: std::ptr::null(),
        flags: 0,
        queueFamilyIndex: queue_family_index,
        queueCount: 1,
        pQueuePriorities: &default_queue_priority,
    };
    let mut cnt = 0;
    let res = unsafe {
        vkEnumerateDeviceExtensionProperties(
            *physical_device,
            std::ptr::null(),
            &mut cnt,
            std::ptr::null_mut(),
        )
    };
    check(res, "get the number of device extension props");
    let mut props = Vec::with_capacity(cnt as usize);
    let res = unsafe {
        vkEnumerateDeviceExtensionProperties(
            *physical_device,
            std::ptr::null(),
            &mut cnt,
            props.as_mut_ptr(),
        )
    };
    check(res, "enumerate device extension props");
    let mut extensions = Vec::with_capacity(cnt as usize);
    for i in 0..cnt {
        extensions.push(unsafe { props.get_unchecked(i as usize).extensionName.as_ptr() });
    }
    let create_info = VkDeviceCreateInfo {
        sType: VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
        pNext: std::ptr::null(),
        flags: 0,
        queueCreateInfoCount: 1,
        pQueueCreateInfos: &queue_create_info,
        enabledLayerCount: 0,
        ppEnabledLayerNames: std::ptr::null(),
        enabledExtensionCount: cnt,
        ppEnabledExtensionNames: extensions.as_ptr(),
        pEnabledFeatures: std::ptr::null(),
    };
    let mut device = std::ptr::null();
    let res = unsafe {
        vkCreateDevice(
            *physical_device,
            &create_info,
            std::ptr::null(),
            &mut device,
        )
    };
    check(res, "create logical device");
    device
}
fn get_device_queue(logical_device: &VkDevice, queue_family_index: u32) -> VkQueue {
    let mut device_queue = std::ptr::null();
    unsafe { vkGetDeviceQueue(*logical_device, queue_family_index, 0, &mut device_queue) };
    device_queue
}
fn create_command_pool(logical_device: &VkDevice, queue_family_index: u32) -> VkCommandPool {
    let create_info = VkCommandPoolCreateInfo {
        sType: VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO,
        pNext: std::ptr::null(),
        flags: VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT,
        queueFamilyIndex: queue_family_index,
    };
    let mut command_pool = std::ptr::null();
    let res = unsafe {
        vkCreateCommandPool(
            *logical_device,
            &create_info,
            std::ptr::null(),
            &mut command_pool,
        )
    };
    check(res, "create command pool");
    command_pool
}
fn check(res: VkResult, msg: &'static str) {
    assert!(
        res == VK_SUCCESS,
        "[fatal error] failed to {}. : {}",
        msg,
        res
    );
}
