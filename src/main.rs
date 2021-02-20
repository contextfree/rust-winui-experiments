#![feature(try_trait)]

// extern crate winapi;
// extern crate winrt;

#[macro_use]
extern crate bitflags;
extern crate libc;
mod DispatcherQueue;
mod nresult;
mod win32_composition;
mod window;
mod windows_ui_composition_interop;

use bindings::{
    windows::foundation::numerics::{Vector2, Vector3},
    windows::ui::composition::{
        CompositionBrush, CompositionColorBrush, IVisual, SpriteVisual, Visual,
    },
    windows::{Interface, Guid},
    windows::ui::Color
};

use nresult::NResult;
use std::mem::transmute;


fn main() {
    match window::Window::new(Default::default(), Default::default()) {
        Ok(window) => match run(&window) {
            Ok(()) => {
                return;
            }
            Err(err) => {
                println!("Error: {:?}", err);
            }
        },
        Err(err) => {
            println!("Error creating window: {:?}", err);
        }
    }
}

fn run(window: &window::Window) -> NResult<()> {
    window.show();
    let comp_host = window.create_composition_host()?;
    let comp = comp_host.compositor;
    let children = comp_host.root_visual.children()?;
    for x in 0..5 {
        for y in 0..5 {
            let child_visual = comp.create_sprite_visual()?;
            let color = Color {
                r: 0xDC,
                g: 0x58,
                b: 0x9B,
                a: 0xD5,
            };
            let brush = comp.create_color_brush_with_color(color)?;
            unsafe {
                child_visual
                    .set_brush(&transmute::<CompositionColorBrush, CompositionBrush>(brush))?;
            }
            let child_visual_ivisual = child_visual.cast::<IVisual>()?;
            child_visual_ivisual.set_size(Vector2 { x: 100.0, y: 100.0 })?;
            child_visual_ivisual.set_offset(Vector3 {
                x: 150.0 * x as f32,
                y: 150.0 * y as f32,
                z: 0.0,
            })?;
            child_visual_ivisual.set_is_visible(true)?;
            unsafe {
                children.insert_at_top(&transmute::<SpriteVisual, Visual>(child_visual))?;
            }
        }
    }
    window::run_events_loop();
    return Ok(());
}
