// use std::ffi::CString;
use std::os::raw::*;

#[allow(non_camel_case_types)]
type xcb_colormap_t = u32;
#[allow(non_camel_case_types)]
type xcb_connection_t = c_void; // !
#[allow(non_camel_case_types)]
type xcb_keycode_t = u8;
#[allow(non_camel_case_types)]
type xcb_visualid_t = u32;
#[allow(non_camel_case_types)]
type xcb_window_t = u32;

const XCB_COPY_FROM_PARENT: u8 = 0;
const XCB_CW_EVENT_MASK: u32 = 2048;
const XCB_EVENT_MASK_EXPOSURE: u32 = 32768;
const XCB_WINDOW_CLASS_INPUT_OUTPUT: u16 = 1;

#[repr(C)]
struct xcb_generic_event_t {
    response_type: u8,
    pad0: u8,
    sequence: u16,
    pad: [u32; 7],
    full_sequence: u32,
}
#[repr(C)]
struct xcb_screen_iterator_t {
    data: *const xcb_screen_t,
    rem: c_int,
    index: c_int,
}
#[repr(C)]
struct xcb_screen_t {
    root: xcb_window_t,
    default_colormap: xcb_colormap_t,
    white_pixel: u32,
    black_pixel: u32,
    current_input_masks: u32,
    width_in_pixels: u16,
    height_in_pixels: u16,
    width_in_millimeters: u16,
    height_in_millimeters: u16,
    min_installed_maps: u16,
    max_installed_maps: u16,
    root_visual: xcb_visualid_t,
    backing_stores: u8,
    save_unders: u8,
    root_depth: u8,
    allowed_depths_len: u8,
}
#[repr(C)]
struct xcb_setup_t {
    status: u8,
    pad0: u8,
    protocol_major_version: u16,
    protocol_minor_version: u16,
    length: u16,
    release_number: u32,
    resource_id_base: u32,
    resource_id_mask: u32,
    motion_buffer_size: u32,
    vendor_len: u16,
    maximum_request_length: u16,
    roots_len: u8,
    pixmap_formats_len: u8,
    image_byte_order: u8,
    bitmap_format_bit_order: u8,
    bitmap_format_scanline_unit: u8,
    bitmap_format_scanline_pad: u8,
    min_keycode: xcb_keycode_t,
    max_keycode: xcb_keycode_t,
    pad1: [u8; 4],
}
#[repr(C)]
struct xcb_void_cookie_t {
    sequence: c_uint,
}

#[link(name = "xcb")]
extern "C" {
    fn xcb_connect(displayname: *const c_char, screenp: *mut c_int) -> *const xcb_connection_t;
    fn xcb_create_window(
        c: *const xcb_connection_t,
        depth: u8,
        wid: xcb_window_t,
        parent: xcb_window_t,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        border_width: u16,
        _class: u16,
        visual: xcb_visualid_t,
        value_mask: u32,
        value_list: *const c_void,
    ) -> xcb_void_cookie_t;
    fn xcb_flush(c: *const xcb_connection_t) -> c_int;
    fn xcb_get_setup(c: *const xcb_connection_t) -> *const xcb_setup_t;
    fn xcb_generate_id(c: *const xcb_connection_t) -> u32;
    fn xcb_map_window(c: *const xcb_connection_t, window: xcb_window_t) -> xcb_void_cookie_t;
    fn xcb_setup_roots_iterator(R: *const xcb_setup_t) -> xcb_screen_iterator_t;
    fn xcb_wait_for_event(c: *const xcb_connection_t) -> *const xcb_generic_event_t;
}

pub struct Window {
    connection: *const xcb_connection_t,
}
impl super::WindowImpl for Window {
    fn new(width: i32, height: i32, _: &'static str, _: bool) -> Self {
        let connection = unsafe { xcb_connect(std::ptr::null(), std::ptr::null_mut()) };
        let screen = unsafe { xcb_setup_roots_iterator(xcb_get_setup(connection)).data };
        let wid = unsafe { xcb_generate_id(connection) };
        unsafe {
            xcb_create_window(
                connection,
                XCB_COPY_FROM_PARENT,
                wid,
                (*screen).root,
                0,
                0,
                width as u16,
                height as u16,
                10,
                XCB_WINDOW_CLASS_INPUT_OUTPUT,
                (*screen).root_visual,
                XCB_CW_EVENT_MASK,
                [XCB_EVENT_MASK_EXPOSURE].as_ptr() as *const c_void,
            )
        };
        unsafe { xcb_map_window(connection, wid) };
        unsafe { xcb_flush(connection) };
        Self { connection }
    }
    fn run(self, f: fn()) {
        loop {
            let event = unsafe { xcb_wait_for_event(self.connection) };
            if event == std::ptr::null() {
                break;
            }
            match unsafe { (*event).response_type & !0x80 } {
                _ => (),
            }
            f();
        }
    }
}
