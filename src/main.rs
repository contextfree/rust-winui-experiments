#![feature(try_trait)]
#![feature(maybe_uninit_ref)]

#[macro_use]
extern crate bitflags;
extern crate libc;
mod nresult;
mod win32_composition;
mod window;
mod path_builder_extensions;

use bindings::{
    windows::foundation::numerics::{Vector2, Vector3},
    windows::ui::Color,
    windows::ui::composition::{CompositionPath, CompositionPathGeometry, PathKeyFrameAnimation, AnimationIterationBehavior, AnimationDelayBehavior},
    windows::ui::xaml::hosting::ElementCompositionPreview,
    microsoft::graphics::canvas::geometry::{CanvasPathBuilder, CanvasFigureLoop, CanvasGeometry}
};

use win32_composition::Win32CompositionHost;
use std::f64::consts::PI;
use nresult::NResult;
use core::time::Duration;

use crate::path_builder_extensions::CanvasPathBuilderExtensions;

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
    let comp = &comp_host.compositor;
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
            child_visual.set_brush(brush)?;
            child_visual.set_size(Vector2 { x: 100.0, y: 100.0 })?;
            child_visual.set_offset(Vector3 {
                x: 150.0 * x as f32,
                y: 150.0 * y as f32,
                z: 0.0,
            })?;
            child_visual.set_is_visible(true)?;
            children.insert_at_top(child_visual)?;
        }
    }

    let shape_container = comp.create_container_shape()?;
    let rotate_anim = comp.create_scalar_key_frame_animation()?;
    rotate_anim.insert_key_frame(0.0, 0.0);
    rotate_anim.insert_key_frame(1.0, (2.0 * PI) as f32);
    rotate_anim.set_duration(Duration::from_secs(2));
    rotate_anim.set_iteration_behavior(AnimationIterationBehavior::Forever);

    fn get_color(x: u8, y: u8) -> Color {
        return Color { 
            a: 64 + x * y * 3 / 4, 
            r: x * 16, g: y * 16, b: 255-(x * y)
        };
    }

    let step_easing_function = comp.create_step_easing_function_with_step_count(1)?;
    for x in 0..7u8 {
        let (morph_geom, morph_anim) = get_regular_polygon_morph(&comp_host, (x + 3).into())?;
        for y in 0..12u8 {
            let shape = comp.create_sprite_shape_with_geometry(morph_geom.clone())?;
            shape.set_scale(Vector2 { x: 50.0, y: 50.0 });
            shape.set_offset(Vector2 { x: ((x as i32 - 1) * 64 + 10) as f32, y : (y as u32 * 64) as f32 });
            shape_container.shapes()?.append(shape.clone());
            let mut color = get_color(x, y);

            let movement_anim = comp.create_vector2_key_frame_animation()?;
            movement_anim.set_duration(Duration::from_secs(1));
            movement_anim.set_delay_behavior(AnimationDelayBehavior::SetInitialValueBeforeDelay);
            movement_anim.insert_key_frame(0.0, Vector2 { x: ((x as i32 - 1) * 64 + 10) as f32, y: (y as u32 * 64) as f32 });
            movement_anim.insert_key_frame(1.0, Vector2 { x: (x as u32 * 64 + 10) as f32, y: (y as u32 * 64) as f32});

            let visibility_anim = comp.create_color_key_frame_animation()?;
            visibility_anim.set_duration(Duration::from_secs((2*(10 + x + 1)).into()));
            let alpha = color.a;
            color.a = 0;
            shape.set_fill_brush(comp.create_color_brush_with_color(color.clone())?);
            visibility_anim.insert_key_frame(0.0, color.clone());
            color.a = alpha;
            visibility_anim.insert_key_frame_with_easing_function(1.0, color, step_easing_function.clone());

            shape.fill_brush()?.start_animation("Color", visibility_anim);
            shape.start_animation("Offset", movement_anim);
            morph_anim.set_delay_time(Duration::from_secs((10 + x + 1).into()));
            morph_geom.start_animation("Path", morph_anim.clone());
            shape.start_animation("RotationAngle", rotate_anim.clone());
        }
    }

    let shape_visual = comp.create_shape_visual()?;
    shape_visual.shapes()?.append(shape_container);
    shape_visual.set_size(Vector2 { x: 1000.0, y: 1000.0 });
    children.insert_at_top(shape_visual)?;

    // let ptr_pos_set = ElementCompositionPreview::get_pointer_position_property_set(&comp_host.root_visual);
    // ElementCompositionPreview::set_element_child_visual(&comp_host.root_visual, shape_visual.clone());
    window::run_events_loop();
    return Ok(());
}

fn create_regular_polygon_geometry(comp_host: &Win32CompositionHost, numSides: u16) -> NResult<CompositionPathGeometry> {
    std::debug_assert!(numSides >= 3);
    let mut start_path_builder = CanvasPathBuilder::create(&comp_host.canvas)?;
    let vertices : Vec<Vector2> = (0..numSides).map(|i| Vector2 {
        x: (PI * 2.0 * (i as f64) / (numSides as f64)).cos() as f32, 
        y: (PI * 2.0 * (i as f64) / (numSides as f64)).sin() as f32
    }).collect();
    start_path_builder.build_path_with_lines(vertices.as_slice(), CanvasFigureLoop::Closed);
    let comp_path = CompositionPath::create(CanvasGeometry::create_path(start_path_builder)?)?;
    let geom = comp_host.compositor.create_path_geometry_with_path(comp_path)?;
    return Ok(geom);
}

fn get_regular_polygon_morph(comp_host: &Win32CompositionHost, old_num_sides: u32) -> NResult<(CompositionPathGeometry, PathKeyFrameAnimation)> {
    fn get_vertex(num_sides: u32, side_ix: f32) -> Vector2 {
        return Vector2 {
            x: (PI * 2.0 * (side_ix as f64) / (num_sides as f64)).cos() as f32, 
            y: (PI * 2.0 * (side_ix as f64) / (num_sides as f64)).sin() as f32    
        };
    }

    std::debug_assert!(old_num_sides >= 3);

    let mut start_path_builder = CanvasPathBuilder::create(&comp_host.canvas)?;
    start_path_builder.begin_figure(get_vertex(old_num_sides, 0.0));
    for i in 1..old_num_sides {
        let vertex = get_vertex(old_num_sides, i as f32);
        start_path_builder.add_line(&vertex);
        start_path_builder.add_line(&vertex);
    }
    start_path_builder.end_figure(CanvasFigureLoop::Closed);

    let start_path = CompositionPath::create(CanvasGeometry::create_path(start_path_builder)?)?;
    let ret_geo = comp_host.compositor.create_path_geometry_with_path(start_path.clone())?;

    let mut end_path_builder = CanvasPathBuilder::create(&comp_host.canvas)?;
    end_path_builder.begin_figure(get_vertex(old_num_sides + 1, 0.0));
    for i in 1..old_num_sides {
        end_path_builder.add_line(get_vertex(old_num_sides + 1, i as f32));
        end_path_builder.add_line(get_vertex(old_num_sides + 1, (i+1) as f32));
    }
    end_path_builder.end_figure(CanvasFigureLoop::Closed);
    let end_path = CompositionPath::create(CanvasGeometry::create_path(end_path_builder)?)?;
  
    let animation = comp_host.compositor.create_path_key_frame_animation()?;
    animation.set_target("Geometry.Path");
    animation.set_duration(Duration::from_secs(2));
    animation.insert_key_frame(0.0, start_path);
    animation.insert_key_frame(1.0, end_path);
    return Ok((ret_geo, animation));
}
