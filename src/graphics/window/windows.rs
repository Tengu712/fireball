use std::ffi::CString;
use std::os::raw::*;

type ATOM = u16;
type BOOL = c_int;
type DWORD = c_ulong;
type HANDLE = PVOID;
type HBRUSH = HANDLE;
type HCURSOR = HICON;
type HICON = HANDLE;
type HINSTANCE = HANDLE;
type HMENU = HANDLE;
type HMODULE = HINSTANCE;
type HWND = HANDLE;
type LONG = c_long;
type LPARAM = isize;
type LPCSTR = *const c_char;
type LPVOID = *const c_void;
type LPMSG = *mut MSG;
type LRESULT = isize;
type PVOID = *const c_void;
type UINT = u32;
type WNDPROC = *const c_void; // !
type WPARAM = usize;

const PM_REMOVE: UINT = 0x0001;
const SW_SHOWDEFAULT: c_int = 10;
const SW_SHOWMAXIMIZED: c_int = 3;
const WM_CLOSE: UINT = 0x0010;
const WM_DESTROY: UINT = 0x0002;
const WM_QUIT: UINT = 0x0012;
const WS_MINIMIZEBOX: c_ulong = 0x00020000;
const WS_OVERLAPPED: c_ulong = 0x00000000;
const WS_POPUP: c_ulong = 0x80000000;
const WS_SYSMENU: c_ulong = 0x00080000;

#[repr(C)]
#[allow(non_snake_case)]
struct MSG {
    hwnd: HWND,
    message: UINT,
    wParam: WPARAM,
    lParam: LPARAM,
    time: DWORD,
    pt: POINT,
    lPrivate: DWORD,
}
impl Default for MSG {
    fn default() -> Self {
        Self {
            hwnd: std::ptr::null(),
            message: 0,
            wParam: 0,
            lParam: 0,
            time: 0,
            pt: POINT { x: 0, y: 0 },
            lPrivate: 0,
        }
    }
}
#[repr(C)]
#[allow(non_snake_case)]
struct POINT {
    x: LONG,
    y: LONG,
}
#[repr(C)]
#[allow(non_snake_case)]
struct WNDCLASSEXA {
    cbSize: UINT,
    style: UINT,
    lpfnWndProc: WNDPROC,
    cbClsExtra: c_int,
    cbWndExtra: c_int,
    hInstance: HINSTANCE,
    hIcon: HICON,
    hCursor: HCURSOR,
    hbrBackground: HBRUSH,
    lpszMenuName: LPCSTR,
    lpszClassName: LPCSTR,
    hIconSm: HICON,
}

#[link(name = "user32")]
extern "stdcall" {
    fn CreateWindowExA(
        dwExStyle: DWORD,
        lpClassName: LPCSTR,
        lpWindowName: LPCSTR,
        dwStyle: DWORD,
        X: c_int,
        Y: c_int,
        nWidth: c_int,
        nHeight: c_int,
        hWndParent: HWND,
        hMenu: HMENU,
        hInstance: HINSTANCE,
        lpParam: LPVOID,
    ) -> HWND;
    fn DefWindowProcA(hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
    fn DispatchMessageA(lpMsg: *const MSG) -> LRESULT;
    fn PeekMessageA(
        lpMsg: LPMSG,
        hWnd: HWND,
        wMsgFilterMin: UINT,
        wMsgFilterMax: UINT,
        wRemoveMsg: UINT,
    ) -> BOOL;
    fn PostQuitMessage(nExitCode: c_int) -> c_void;
    fn RegisterClassExA(unnamedParam1: *const WNDCLASSEXA) -> ATOM;
    fn ShowCursor(bShow: BOOL) -> c_int;
    fn ShowWindow(hWnd: HWND, nCmdShow: c_int) -> BOOL;
    fn TranslateMessage(lpMsg: *const MSG) -> BOOL;
}
#[link(name = "kernel32")]
extern "stdcall" {
    fn GetModuleHandleA(lpModuleName: LPCSTR) -> HMODULE;
}

fn wnd_proc(h_wnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_CLOSE | WM_DESTROY => {
            unsafe { PostQuitMessage(0) };
            return 0;
        }
        _ => return unsafe { DefWindowProcA(h_wnd, msg, wparam, lparam) },
    }
}

pub struct Window {
    hwnd: HWND,
}
impl Window {
    pub fn new(width: i32, height: i32, title: &'static str, is_windowed: bool) -> Self {
        let (style, cmd_show) = if is_windowed {
            (WS_OVERLAPPED | WS_SYSMENU | WS_MINIMIZEBOX, SW_SHOWDEFAULT)
        } else {
            (WS_POPUP, SW_SHOWMAXIMIZED)
        };
        let wndcls_cstr = CString::new("WNDCLS_NAME").unwrap();
        let title_cstr = CString::new(title).unwrap();
        let h_inst = unsafe { GetModuleHandleA(std::ptr::null()) };
        let wcex = WNDCLASSEXA {
            cbSize: std::mem::size_of::<WNDCLASSEXA>() as UINT,
            style: 0x0040,
            lpfnWndProc: wnd_proc as *const c_void,
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: h_inst,
            hIcon: std::ptr::null(),
            hCursor: std::ptr::null(),
            hbrBackground: std::ptr::null(),
            lpszMenuName: std::ptr::null(),
            lpszClassName: wndcls_cstr.as_ptr(),
            hIconSm: std::ptr::null(),
        };
        if unsafe { RegisterClassExA(&wcex) == 0 } {
            panic!("[fatal error] failed to register window class.");
        }
        let hwnd = unsafe {
            CreateWindowExA(
                0,
                wndcls_cstr.as_ptr(),
                title_cstr.as_ptr(),
                style,
                0,
                0,
                width,
                height,
                std::ptr::null(),
                std::ptr::null(),
                h_inst,
                std::ptr::null(),
            )
        };
        if hwnd == std::ptr::null() {
            panic!("[fatal error] failed to create window.");
        }
        unsafe { ShowWindow(hwnd, cmd_show) };
        unsafe { ShowCursor(if is_windowed { 1 } else { 0 }) };
        Self { hwnd }
    }
    pub fn run(self, f: fn()) {
        let mut msg = Default::default();
        loop {
            if unsafe { PeekMessageA(&mut msg, self.hwnd, 0, 0, PM_REMOVE) != 0 } {
                if msg.message == WM_QUIT {
                    return;
                }
                unsafe { TranslateMessage(&msg) };
                unsafe { DispatchMessageA(&msg) };
                continue;
            }
            f();
        }
    }
}
