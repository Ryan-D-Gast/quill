use crate::elements::Axis;
use crate::style::*;
use svg::Document;
use svg::node::element::{Line as SvgLine, Rectangle};

pub fn draw_axis_lines(
    document: Document,
    axis: Axis,
    axis_config: &AxisConfig,
    plot_area_x_start: f32,
    plot_area_y_start: f32,
    plot_area_width: f32,
    plot_area_height: f32,
) -> Document {
    let axis_color = axis_config.color.to_hex_string();
    let axis_stroke_width = axis_config.line_width;
    match axis {
        Axis::BottomLeft => {
            let x_axis_line = SvgLine::new()
                .set("x1", plot_area_x_start)
                .set("y1", plot_area_y_start + plot_area_height)
                .set("x2", plot_area_x_start + plot_area_width)
                .set("y2", plot_area_y_start + plot_area_height)
                .set("stroke", axis_color.clone())
                .set("stroke-width", axis_stroke_width);
            let document = document.add(x_axis_line);
            let y_axis_line = SvgLine::new()
                .set("x1", plot_area_x_start)
                .set("y1", plot_area_y_start)
                .set("x2", plot_area_x_start)
                .set("y2", plot_area_y_start + plot_area_height)
                .set("stroke", axis_color)
                .set("stroke-width", axis_stroke_width);
            document.add(y_axis_line)
        }
        Axis::Box => {
            let box_rect = Rectangle::new()
                .set("x", plot_area_x_start)
                .set("y", plot_area_y_start)
                .set("width", plot_area_width)
                .set("height", plot_area_height)
                .set("stroke", axis_color)
                .set("stroke-width", axis_stroke_width)
                .set("fill", "none");
            document.add(box_rect)
        }
    }
}
