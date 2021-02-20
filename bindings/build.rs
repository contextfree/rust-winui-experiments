fn main() {
    windows::build!(
        // windows::system::*,
        // windows::ui::*,
        // windows::ui::composition::*,
        // windows::ui::composition::desktop::*,
        // windows::win32::winrt::*,
        // windows::win32::system_services::*,
        // windows::win32::windows_and_messaging::*,
        // windows::win32::gdi::*,
        // windows::win32::debug::*,
        // windows::win32::menus_and_resources::*
        windows::foundation::numerics::{Vector2, Vector3},
        windows::ui::composition::{
            Compositor, ContainerVisual, ICompositionTarget, IVisual, IVisual2, Visual,
            CompositionBrush, CompositionColorBrush,  SpriteVisual
        },
        windows::ui::composition::desktop::DesktopWindowTarget,
        // windows::{Interface, Guid, Abi, BOOL},
        windows::ui::Color,
        windows::system::DispatcherQueueController,
        windows::win32::winrt::ICompositorDesktopInterop,
        windows::win32::gdi::{HICON, HCURSOR, HBRUSH},
        // windows::win32::windows_and_messaging::{HWND, LPARAM, WPARAM, ShowWindow, CreateWindowExW, RegisterClassExW, IsGUIThread, GetMessageW, TranslateMessage, DispatchMessageW, PostQuitMessage, DefWindowProcW, WNDCLASSEXW},
        windows::win32::windows_and_messaging::*,
        windows::win32::debug::{GetLastError, FormatMessageW},
        windows::win32::menus_and_resources::{HMENU, CreateIcon, lstrlenW},
        // windows::win32::system_services::{LocalFree, GetModuleHandleW, CW_USEDEFAULT, SW_SHOW, WM_QUIT, CS_VREDRAW, CS_HREDRAW}
        windows::win32::system_services::*   
    );
    // println!("cargo:rustc-link-lib=CoreMessaging");
}