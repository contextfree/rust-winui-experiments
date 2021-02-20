use std::mem::{size_of, transmute};

use bindings::{
  windows::foundation::numerics::{Vector2, Vector3},
  windows::system::DispatcherQueueController,
  windows::ui::composition::desktop::DesktopWindowTarget,
  windows::ui::composition::{
    Compositor, ContainerVisual, ICompositionTarget, IVisual, IVisual2, Visual,
  },
  windows::Interface,
  windows::win32::winrt::ICompositorDesktopInterop,
  windows::win32::system_services::{
     CreateDispatcherQueueController, DispatcherQueueOptions, DISPATCHERQUEUE_THREAD_APARTMENTTYPE, DISPATCHERQUEUE_THREAD_TYPE
  }
};

use crate::nresult::NResult;
use crate::window::Window;

pub struct Win32CompositionHost {
  pub compositor: Compositor,
  pub root_visual: ContainerVisual,
  // root_visual depends on the underlying composition target and
  // dispatcher queue controller being kept alive, so we retain them here
  #[allow(dead_code)]
  target: ICompositionTarget,
  #[allow(dead_code)]
  dispatcher_queue_controller: DispatcherQueueController,
}

impl Window {
  pub fn create_composition_host(&self) -> NResult<Win32CompositionHost> {
    let queue: DispatcherQueueController = init_dispatcher_queue()?;
    let comp = Compositor::new()?;
    let target = create_desktop_window_target(&self, &comp)?;
    let comp_root = create_composition_root(&comp, &target)?;
    let composition_target = target.cast::<ICompositionTarget>()?;
    return Ok(Win32CompositionHost {
      compositor: comp,
      root_visual: comp_root,
      target: composition_target,
      dispatcher_queue_controller: queue,
    });
  }
}

pub fn init_dispatcher_queue() -> NResult<DispatcherQueueController> {
  let options = DispatcherQueueOptions {
    dw_size: size_of::<DispatcherQueueOptions>() as u32,
    thread_type: DISPATCHERQUEUE_THREAD_TYPE::DQTYPE_THREAD_CURRENT,
    apartment_type: DISPATCHERQUEUE_THREAD_APARTMENTTYPE::DQTAT_COM_ASTA,
  };
  unsafe {
    let mut controller: Option<DispatcherQueueController> = None;
    let _ = CreateDispatcherQueueController(
      options, &mut controller as *mut _
    );
    return Ok(controller?);
  }
}

pub fn create_composition_root(
  compositor: &Compositor,
  target: &DesktopWindowTarget,
) -> NResult<ContainerVisual> {
  let container_visual = compositor.create_container_visual()?;
  let visual2 = container_visual.cast::<IVisual2>()?;
  visual2.set_relative_size_adjustment(Vector2 { x: 1.0, y: 1.0 })?;
  let visual1 = container_visual.cast::<IVisual>()?;
  visual1.set_offset(Vector3 {
    x: 24.0,
    y: 24.0,
    z: 0.0,
  })?;
  let composition_target = target.cast::<ICompositionTarget>()?;
  unsafe {
    let visual = transmute::<ContainerVisual, Visual>(container_visual.clone());
    composition_target.set_root(&visual)?;
    return Ok(container_visual);
  }
}

pub fn create_desktop_window_target(
  window: &Window,
  compositor: &Compositor,
) -> NResult<DesktopWindowTarget> {
  let hwnd = window.hwnd();
  let interop = compositor.cast::<ICompositorDesktopInterop>()?;
  unsafe {
    let mut ret: Option<DesktopWindowTarget> = None;
    let _ = interop.CreateDesktopWindowTarget(
      hwnd, windows::BOOL::from(true), &mut ret as *mut _
    );
    return Ok(ret?);
  }
}
