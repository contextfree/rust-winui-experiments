#![feature(try_trait)]

extern crate winapi;
extern crate winrt;
#[macro_use]
extern crate bitflags;
extern crate libc;
mod DispatcherQueue;
mod nresult;
mod win32_composition;
mod window;
mod windows_ui_composition_interop;

use nresult::NResult;
use std::mem::transmute;
use winrt::windows::foundation::numerics::{Vector2, Vector3};
use winrt::windows::ui::composition::{
    CompositionBrush, CompositionColorBrush, IVisual, SpriteVisual, Visual,
};
use winrt::Guid;

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
    let children = comp_host.root_visual.get_children()??;
    for x in 0..5 {
        for y in 0..5 {
            let child_visual = comp.create_sprite_visual()??;
            let color = winrt::windows::ui::Color {
                R: 0xDC,
                G: 0x58,
                B: 0x9B,
                A: 0xD5,
            };
            let brush = comp.create_color_brush_with_color(color)??;
            unsafe {
                child_visual
                    .set_brush(&transmute::<CompositionColorBrush, CompositionBrush>(brush))?;
            }
            let child_visual_ivisual = child_visual.query_interface::<IVisual>()?;
            child_visual_ivisual.set_size(Vector2 { X: 100.0, Y: 100.0 })?;
            child_visual_ivisual.set_offset(Vector3 {
                X: 150.0 * x as f32,
                Y: 150.0 * y as f32,
                Z: 0.0,
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
