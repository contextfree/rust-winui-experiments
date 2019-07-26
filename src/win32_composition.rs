use std::mem::{size_of, transmute};
use std::ptr;
use winapi::shared::minwindef::BOOL;
use winapi::shared::windef::HWND;
use winrt::windows::foundation::numerics::{Vector2, Vector3};
use winrt::windows::system::IDispatcherQueueController;
use winrt::windows::ui::composition::desktop::IDesktopWindowTarget;
use winrt::windows::ui::composition::{
  Compositor, ContainerVisual, ICompositionTarget, IVisual, IVisual2, Visual,
};
use winrt::{ComInterface, RtDefaultConstructible};

use nresult::NResult;
use window::{Window, WindowExt};
use windows_ui_composition_interop::ICompositorDesktopInterop;
use DispatcherQueue::{
  CreateDispatcherQueueController, DispatcherQueueOptions, DQTAT_COM_ASTA, DQTYPE_THREAD_CURRENT,
};

pub struct Win32CompositionHost {
  pub compositor: Compositor,
  pub root_visual: ContainerVisual,
  // root_visual depends on the underlying composition target and
  // dispatcher queue controller being kept alive, so we retain them here
  #[allow(dead_code)]
  target: ICompositionTarget,
  #[allow(dead_code)]
  dispatcher_queue_controller: IDispatcherQueueController,
}

impl Window {
  pub fn create_composition_host(&self) -> NResult<Win32CompositionHost> {
    let queue: IDispatcherQueueController = init_dispatcher_queue();
    let comp = winrt::windows::ui::composition::Compositor::new();
    let target = create_desktop_window_target(&self, &comp)?;
    let comp_root = create_composition_root(&comp, &target)?;
    let composition_target = target.query_interface::<ICompositionTarget>()?;
    return Ok(Win32CompositionHost {
      compositor: comp,
      root_visual: comp_root,
      target: composition_target,
      dispatcher_queue_controller: queue,
    });
  }
}

pub fn init_dispatcher_queue() -> IDispatcherQueueController {
  let options = DispatcherQueueOptions {
    dwSize: size_of::<DispatcherQueueOptions>() as u32,
    threadType: DQTYPE_THREAD_CURRENT,
    apartmentType: DQTAT_COM_ASTA,
  };
  unsafe {
    let mut p_controller: *mut <IDispatcherQueueController as ComInterface>::TAbi = ptr::null_mut();
    CreateDispatcherQueueController(
      options,
      (&mut p_controller) as *mut *mut <IDispatcherQueueController as ComInterface>::TAbi,
    );
    return IDispatcherQueueController::wrap_com(p_controller);
  }
}

pub fn create_composition_root(
  compositor: &Compositor,
  target: &IDesktopWindowTarget,
) -> NResult<ContainerVisual> {
  let container_visual = compositor.create_container_visual()??;
  let visual2 = container_visual.query_interface::<IVisual2>()?;
  visual2.set_relative_size_adjustment(Vector2 { X: 1.0, Y: 1.0 })?;
  let visual1 = container_visual.query_interface::<IVisual>()?;
  visual1.set_offset(Vector3 {
    X: 24.0,
    Y: 24.0,
    Z: 0.0,
  })?;
  let composition_target = target.query_interface::<ICompositionTarget>()?;
  unsafe {
    let visual = transmute::<ContainerVisual, Visual>(container_visual.clone());
    composition_target.set_root(&visual)?;
    return Ok(container_visual);
  }
}

pub fn create_desktop_window_target(
  window: &Window,
  compositor: &Compositor,
) -> NResult<IDesktopWindowTarget> {
  let hwnd = window.get_hwnd() as HWND;
  let mut interop = compositor.query_interface::<ICompositorDesktopInterop>()?;
  unsafe {
    let mut ret: *mut <IDesktopWindowTarget as ComInterface>::TAbi = ptr::null_mut();
    interop.CreateDesktopWindowTarget(
      hwnd,
      true as BOOL,
      (&mut ret) as *mut *mut _ as *mut IDesktopWindowTarget,
    );
    return Ok(IDesktopWindowTarget::wrap_com(ret));
  }
}
