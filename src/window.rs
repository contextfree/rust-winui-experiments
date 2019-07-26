// The code in this file is derived from the winit project ( https://github.com/rust-windowing/winit ), 
// created by the winit contributors including Pierre Krieger and Francesca Plebani. 
// It has been extensively modified to remove most functionality not needed by the present project.
// winit is licensed under Apache License 2.0 which can be found in this project as "LICENSE_winit"

use libc;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::{
    assert_eq, debug_assert_eq, f64, format, io, isize, mem, panic, ptr, u16, u32, u8, usize,
};
use winapi::ctypes::{c_int, wchar_t};
use winapi::shared::minwindef::{BYTE, DWORD, LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::ntdef::{LANG_NEUTRAL, LPCWSTR, MAKELANGID, SUBLANG_DEFAULT};
use winapi::shared::windef::{HICON, HWND};
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::libloaderapi;
use winapi::um::winbase::{
    lstrlenW, FormatMessageW, LocalFree, FORMAT_MESSAGE_ALLOCATE_BUFFER,
    FORMAT_MESSAGE_FROM_SYSTEM, FORMAT_MESSAGE_IGNORE_INSERTS,
};
use winapi::um::winuser;

/// A size represented in logical pixels.
///
/// The size is stored as floats, so please be careful. Casting floats to integers truncates the fractional part,
/// which can cause noticable issues. To help with that, an `Into<(u32, u32)>` implementation is provided which
/// does the rounding for you.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LogicalSize {
    pub width: f64,
    pub height: f64,
}

impl LogicalSize {
    #[inline]
    pub fn new(width: f64, height: f64) -> Self {
        LogicalSize { width, height }
    }
}

impl From<(u32, u32)> for LogicalSize {
    #[inline]
    fn from((width, height): (u32, u32)) -> Self {
        Self::new(width as f64, height as f64)
    }
}

#[repr(C)]
#[derive(Debug)]
pub(crate) struct Pixel {
    pub(crate) r: u8,
    pub(crate) g: u8,
    pub(crate) b: u8,
    pub(crate) a: u8,
}

impl Pixel {
    fn to_bgra(&mut self) {
        mem::swap(&mut self.r, &mut self.b);
    }
}

pub(crate) const PIXEL_SIZE: usize = mem::size_of::<Pixel>();

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct WinError(Option<String>);

impl WinError {
    pub fn from_last_error() -> Self {
        WinError(unsafe { get_last_error() })
    }
}

pub fn wchar_to_string(wchar: &[wchar_t]) -> String {
    String::from_utf16_lossy(wchar).to_string()
}

pub unsafe fn get_last_error() -> Option<String> {
    let err = GetLastError();
    if err != 0 {
        let buf_addr: LPCWSTR = {
            let mut buf_addr: LPCWSTR = mem::uninitialized();
            FormatMessageW(
                FORMAT_MESSAGE_ALLOCATE_BUFFER
                    | FORMAT_MESSAGE_FROM_SYSTEM
                    | FORMAT_MESSAGE_IGNORE_INSERTS,
                ptr::null(),
                err,
                MAKELANGID(LANG_NEUTRAL, SUBLANG_DEFAULT) as DWORD,
                // This is a pointer to a pointer
                &mut buf_addr as *mut LPCWSTR as *mut _,
                0,
                ptr::null_mut(),
            );
            buf_addr
        };
        if !buf_addr.is_null() {
            let buf_len = lstrlenW(buf_addr) as usize;
            let buf_slice = std::slice::from_raw_parts(buf_addr, buf_len);
            let string = wchar_to_string(buf_slice);
            LocalFree(buf_addr as *mut _);
            return Some(string);
        }
    }
    None
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// An icon used for the window titlebar, taskbar, etc.
///
/// Enabling the `icon_loading` feature provides you with several convenience methods for creating
/// an `Icon` from any format supported by the [image](https://github.com/PistonDevelopers/image)
/// crate.
pub struct Icon {
    pub(crate) rgba: Vec<u8>,
    pub(crate) width: u32,
    pub(crate) height: u32,
}

#[derive(Clone, Debug)]
pub struct WinIcon {
    pub handle: HICON,
}

/// A simple non-owning wrapper around a window.
#[doc(hidden)]
#[derive(Clone)]
pub struct WindowWrapper(HWND);

/// w_attr to use when creating a window.
#[derive(Debug, Clone)]
pub struct WindowAttributes {
    /// The dimensions of the window. If this is `None`, some platform-specific dimensions will be
    /// used.
    ///
    /// The default is `None`.
    pub dimensions: Option<LogicalSize>,
    /// The minimum dimensions a window can be, If this is `None`, the window will have no minimum dimensions (aside from reserved).
    ///
    /// The default is `None`.
    pub min_dimensions: Option<LogicalSize>,
    /// The maximum dimensions a window can be, If this is `None`, the maximum will have no maximum or will be set to the primary monitor's dimensions by the platform.
    ///
    /// The default is `None`.
    pub max_dimensions: Option<LogicalSize>,
    /// Whether the window is resizable or not.
    ///
    /// The default is `true`.
    pub resizable: bool,
    /// The title of the window in the title bar.
    ///
    /// The default is `"winit window"`.
    pub title: String,
    /// Whether the window should be maximized upon creation.
    ///
    /// The default is `false`.
    pub maximized: bool,
    /// Whether the window should be immediately visible upon creation.
    ///
    /// The default is `true`.
    pub visible: bool,
    /// Whether the the window should be transparent. If this is true, writing colors
    /// with alpha values different than `1.0` will produce a transparent window.
    ///
    /// The default is `false`.
    pub transparent: bool,
    /// Whether the window should have borders and bars.
    ///
    /// The default is `true`.
    pub decorations: bool,
    /// Whether the window should always be on top of other windows.
    ///
    /// The default is `false`.
    pub always_on_top: bool,
    /// The window icon.
    ///
    /// The default is `None`.
    pub window_icon: Option<Icon>,
}

impl Default for WindowAttributes {
    #[inline]
    fn default() -> WindowAttributes {
        WindowAttributes {
            dimensions: None,
            min_dimensions: None,
            max_dimensions: None,
            resizable: true,
            title: "winit window".to_owned(),
            maximized: false,
            visible: true,
            transparent: false,
            decorations: true,
            always_on_top: false,
            window_icon: None,
        }
    }
}

#[derive(Clone, Default)]
pub struct PlatformSpecificWindowBuilderAttributes {
    pub parent: Option<HWND>,
    pub taskbar_icon: Option<Icon>,
    pub no_redirection_bitmap: bool,
}

/// The Win32 implementation of the main `Window` object.
pub struct Window {
    /// Main handle for the window.
    window: WindowWrapper,
}

impl Window {
    /// Returns the `hwnd` of this window.
    #[inline]
    pub fn hwnd(&self) -> HWND {
        self.window.0
    }

    #[inline]
    pub fn show(&self) {
        unsafe {
            winuser::ShowWindow(self.window.0, winuser::SW_SHOW);
        }
    }
}

/// Additional methods on `Window` that are specific to Windows.
pub trait WindowExt {
    /// Returns the native handle that is used by this window.
    ///
    /// The pointer will become invalid when the native window was destroyed.
    fn get_hwnd(&self) -> *mut libc::c_void;
}

impl WindowExt for Window {
    #[inline]
    fn get_hwnd(&self) -> *mut libc::c_void {
        self.hwnd() as *mut _
    }
}

impl WinIcon {
    pub fn from_icon(icon: Icon) -> Result<Self, WinError> {
        Self::from_rgba(icon.rgba, icon.width, icon.height)
    }

    pub fn from_rgba(mut rgba: Vec<u8>, width: u32, height: u32) -> Result<Self, WinError> {
        assert_eq!(rgba.len() % PIXEL_SIZE, 0);
        let pixel_count = rgba.len() / PIXEL_SIZE;
        assert_eq!(pixel_count, (width * height) as usize);
        let mut and_mask = Vec::with_capacity(pixel_count);
        let pixels = rgba.as_mut_ptr() as *mut Pixel; // how not to write idiomatic Rust
        for pixel_index in 0..pixel_count {
            let pixel = unsafe { &mut *pixels.offset(pixel_index as isize) };
            and_mask.push(pixel.a.wrapping_sub(std::u8::MAX)); // invert alpha channel
            pixel.to_bgra();
        }
        assert_eq!(and_mask.len(), pixel_count);
        let handle = unsafe {
            winuser::CreateIcon(
                ptr::null_mut(),
                width as c_int,
                height as c_int,
                1,
                (PIXEL_SIZE * 8) as BYTE,
                and_mask.as_ptr() as *const BYTE,
                rgba.as_ptr() as *const BYTE,
            ) as HICON
        };
        if !handle.is_null() {
            Ok(WinIcon { handle })
        } else {
            Err(WinError::from_last_error())
        }
    }
}

bitflags! {
    pub struct WindowFlags: u32 {
        const RESIZABLE      = 1 << 0;
        const DECORATIONS    = 1 << 1;
        const VISIBLE        = 1 << 2;
        const ON_TASKBAR     = 1 << 3;
        const ALWAYS_ON_TOP  = 1 << 4;
        const NO_BACK_BUFFER = 1 << 5;
        const TRANSPARENT    = 1 << 6;
        const CHILD          = 1 << 7;
        const MAXIMIZED      = 1 << 8;

        /// Marker flag for fullscreen. Should always match `WindowState::fullscreen`, but is
        /// included here to make masking easier.
        const MARKER_FULLSCREEN = 1 << 9;

        /// The `WM_SIZE` event contains some parameters that can effect the state of `WindowFlags`.
        /// In most cases, it's okay to let those parameters change the state. However, when we're
        /// running the `WindowFlags::apply_diff` function, we *don't* want those parameters to
        /// effect our stored state, because the purpose of `apply_diff` is to update the actual
        /// window's state to match our stored state. This controls whether to accept those changes.
        const MARKER_RETAIN_STATE_ON_SIZE = 1 << 10;

        const FULLSCREEN_AND_MASK = !(
            WindowFlags::DECORATIONS.bits |
            WindowFlags::RESIZABLE.bits |
            WindowFlags::MAXIMIZED.bits
        );
        const NO_DECORATIONS_AND_MASK = !WindowFlags::RESIZABLE.bits;
        const INVISIBLE_AND_MASK = !WindowFlags::MAXIMIZED.bits;
    }
}

impl WindowFlags {
    pub fn to_window_styles(self) -> (DWORD, DWORD) {
        use winapi::um::winuser::*;

        let (mut style, mut style_ex) = (0, 0);

        if self.contains(WindowFlags::RESIZABLE) {
            style |= WS_SIZEBOX | WS_MAXIMIZEBOX;
        }
        if self.contains(WindowFlags::DECORATIONS) {
            style |= WS_CAPTION | WS_MINIMIZEBOX | WS_BORDER;
            style_ex = WS_EX_WINDOWEDGE;
        }
        if self.contains(WindowFlags::VISIBLE) {
            style |= WS_VISIBLE;
        }
        if self.contains(WindowFlags::ON_TASKBAR) {
            style_ex |= WS_EX_APPWINDOW;
        }
        if self.contains(WindowFlags::ALWAYS_ON_TOP) {
            style_ex |= WS_EX_TOPMOST;
        }
        if self.contains(WindowFlags::NO_BACK_BUFFER) {
            style_ex |= WS_EX_NOREDIRECTIONBITMAP;
        }
        if self.contains(WindowFlags::TRANSPARENT) {
            // Is this necessary? The docs say that WS_EX_LAYERED requires a windows class without
            // CS_OWNDC, and Winit windows have that flag set.
            style_ex |= WS_EX_LAYERED;
        }
        if self.contains(WindowFlags::CHILD) {
            style |= WS_CHILD; // This is incompatible with WS_POPUP if that gets added eventually.
        }
        if self.contains(WindowFlags::MAXIMIZED) {
            style |= WS_MAXIMIZE;
        }

        style |= WS_CLIPSIBLINGS | WS_CLIPCHILDREN | WS_SYSMENU;
        style_ex |= WS_EX_ACCEPTFILES;

        (style, style_ex)
    }
}

/// Error that can happen while creating a window or a headless renderer.
#[derive(Debug, Clone)]
pub enum CreationError {
    OsError(String),
}

impl Window {
    pub fn new(
        w_attr: WindowAttributes,
        pl_attr: PlatformSpecificWindowBuilderAttributes,
    ) -> Result<Window, CreationError> {
        // registering the window class

        let window_icon = {
            let icon = w_attr
                .window_icon
                // .take()
                .map(WinIcon::from_icon);
            if icon.is_some() {
                Some(icon.unwrap().map_err(|err| {
                    CreationError::OsError(format!("Failed to create `ICON_SMALL`: {:?}", err))
                })?)
            } else {
                None
            }
        };
        let taskbar_icon = {
            let icon = pl_attr
                .taskbar_icon
                // .take()
                .map(WinIcon::from_icon);
            if icon.is_some() {
                Some(icon.unwrap().map_err(|err| {
                    CreationError::OsError(format!("Failed to create `ICON_BIG`: {:?}", err))
                })?)
            } else {
                None
            }
        };
        unsafe {
            let class_name = register_window_class(&window_icon, &taskbar_icon);
            let mut window_flags = WindowFlags::empty();
            window_flags.set(WindowFlags::DECORATIONS, w_attr.decorations);
            window_flags.set(WindowFlags::ALWAYS_ON_TOP, w_attr.always_on_top);
            window_flags.set(WindowFlags::NO_BACK_BUFFER, pl_attr.no_redirection_bitmap);
            window_flags.set(WindowFlags::TRANSPARENT, w_attr.transparent);
            // WindowFlags::VISIBLE and MAXIMIZED are set down below after the window has been configured.
            window_flags.set(WindowFlags::RESIZABLE, w_attr.resizable);
            window_flags.set(WindowFlags::CHILD, pl_attr.parent.is_some());
            window_flags.set(WindowFlags::ON_TASKBAR, true);

            let title = OsStr::new(&w_attr.title)
                .encode_wide()
                .chain(Some(0).into_iter())
                .collect::<Vec<_>>();

            // creating the real window this time, by using the functions in `extra_functions`
            let real_window = {
                let (style, ex_style) = window_flags.to_window_styles();
                let handle = winuser::CreateWindowExW(
                    ex_style,
                    class_name.as_ptr(),
                    title.as_ptr() as LPCWSTR,
                    style,
                    winuser::CW_USEDEFAULT,
                    winuser::CW_USEDEFAULT,
                    winuser::CW_USEDEFAULT,
                    winuser::CW_USEDEFAULT,
                    pl_attr.parent.unwrap_or(ptr::null_mut()),
                    ptr::null_mut(),
                    libloaderapi::GetModuleHandleW(ptr::null()),
                    ptr::null_mut(),
                );

                if handle.is_null() {
                    return Err(CreationError::OsError(format!(
                        "CreateWindowEx function failed: {}",
                        format!("{}", io::Error::last_os_error())
                    )));
                }

                WindowWrapper(handle)
            };

            Ok(Window {
                window: real_window,
            })
        }
    }
}

// init instance
// message loop?

unsafe fn register_window_class(
    window_icon: &Option<WinIcon>,
    taskbar_icon: &Option<WinIcon>,
) -> Vec<u16> {
    let class_name: Vec<_> = std::ffi::OsStr::new("Window Class")
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect();

    let h_icon = taskbar_icon
        .as_ref()
        .map(|icon| icon.handle)
        .unwrap_or(ptr::null_mut());

    let h_icon_small = window_icon
        .as_ref()
        .map(|icon| icon.handle)
        .unwrap_or(ptr::null_mut());

    let class = winuser::WNDCLASSEXW {
        cbSize: mem::size_of::<winuser::WNDCLASSEXW>() as UINT,
        style: winuser::CS_HREDRAW | winuser::CS_VREDRAW,
        lpfnWndProc: Some(callback),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: libloaderapi::GetModuleHandleW(ptr::null()),
        hCursor: ptr::null_mut(), // must be null in order for cursor state to work properly
        hbrBackground: ptr::null_mut(),
        lpszMenuName: ptr::null(),
        lpszClassName: class_name.as_ptr(),
        hIcon: h_icon,
        hIconSm: h_icon_small,
    };

    // We ignore errors because registering the same window class twice would trigger
    //  an error, and because errors here are detected during CreateWindowEx anyway.
    // Also since there is no weird element in the struct, there is no reason for this
    //  call to fail.
    winuser::RegisterClassExW(&class);
    class_name
}

pub fn run_events_loop() {
    unsafe {
        winuser::IsGUIThread(1);

        let mut msg = mem::uninitialized();

        loop {
            if winuser::GetMessageW(&mut msg, ptr::null_mut(), 0, 0) == 0 {
                // Only happens if the message is `WM_QUIT`.
                debug_assert_eq!(msg.message, winuser::WM_QUIT);
                break;
            }

            // Calls `callback` below.
            winuser::TranslateMessage(&msg);
            winuser::DispatchMessageW(&msg);
        }
    }
}

pub unsafe fn run_catch_panic<F, R>(error: R, f: F) -> R
where
    F: panic::UnwindSafe + FnOnce() -> R,
{
    let callback_result = panic::catch_unwind(f);
    match callback_result {
        Ok(lresult) => lresult,
        Err(_) => {
            winuser::PostQuitMessage(-1);
            error
        }
    }
}

/// Any window whose callback is configured to this function will have its events propagated
/// through the events loop of the thread the window was created in.
//
// This is the callback that is called by `DispatchMessage` in the events loop.
//
// Returning 0 tells the Win32 API that the message has been processed.
// FIXME: detect WM_DWMCOMPOSITIONCHANGED and call DwmEnableBlurBehindWindow if necessary
pub unsafe extern "system" fn callback(
    window: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    // Unwinding into foreign code is undefined behavior. So we catch any panics that occur in our
    // code, and if a panic happens we cancel any future operations.
    run_catch_panic(-1, || callback_inner(window, msg, wparam, lparam))
}

unsafe fn callback_inner(window: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        _ => winuser::DefWindowProcW(window, msg, wparam, lparam),
    }
}
