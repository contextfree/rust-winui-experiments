#![allow(non_snake_case, non_upper_case_globals)]
use winapi::shared::minwindef::{BOOL, DWORD};
use winapi::shared::ntdef::HRESULT;
use winapi::shared::windef::HWND;
use winrt::windows::ui::composition::desktop::IDesktopWindowTarget;
use winrt::IUnknown;

winrt::DEFINE_IID!(
    IID_ICompositorDesktopInterop,
    0x29e691fa,
    0x4567,
    0x4dca,
    0xb3,
    0x19,
    0xd0,
    0xf2,
    0x07,
    0xeb,
    0x68,
    0x07
);
winrt::COM_INTERFACE! {interface ICompositorDesktopInterop(ICompositorDesktopInteropVtbl): IUnknown [IID_ICompositorDesktopInterop] {
    fn CreateDesktopWindowTarget(
        &mut self,
        hwndTarget: HWND,
        isTopmost: BOOL,
        result: *mut IDesktopWindowTarget
    ) -> HRESULT,
    fn EnsureOnThread(
        &mut self,
        threadId: DWORD
    ) -> HRESULT
}}
