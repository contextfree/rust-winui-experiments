fn main() {
    windows::build!(
        windows::foundation::numerics::{Vector2, Vector3},
        windows::ui::composition::{
            Compositor, ContainerVisual, ICompositionTarget, IVisual, IVisual2, Visual,
            CompositionBrush, CompositionColorBrush,  SpriteVisual
        },
        windows::ui::composition::desktop::DesktopWindowTarget,
        windows::ui::Color,
        windows::system::DispatcherQueueController,
        windows::win32::winrt::ICompositorDesktopInterop,
        windows::win32::gdi::{HICON, HCURSOR, HBRUSH},
        windows::win32::windows_and_messaging::*,
        windows::win32::debug::{GetLastError, FormatMessageW},
        windows::win32::menus_and_resources::{HMENU, CreateIcon, lstrlenW},
        windows::win32::system_services::*,
        windows::ui::xaml::hosting::ElementCompositionPreview,
        microsoft::graphics::canvas::* 
    );
}