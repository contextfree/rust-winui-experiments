use bindings::{
    microsoft::graphics::canvas::geometry::{CanvasPathBuilder, CanvasFigureLoop},
    windows::foundation::numerics::Vector2
};

pub trait CanvasPathBuilderExtensions {
    fn build_path_with_lines(&self, vectors: &[Vector2], canvas_figure_loop: CanvasFigureLoop) -> &Self;
}

impl CanvasPathBuilderExtensions for CanvasPathBuilder {
    fn build_path_with_lines(&self, vectors: &[Vector2], canvas_figure_loop: CanvasFigureLoop) -> &Self {        
        if let Some((first, rest)) = vectors.split_first() {           
            self.begin_figure(first);
            for vector in rest {
                self.add_line(vector);
            }
            self.end_figure(canvas_figure_loop);
        }
        return self;
    }
}