use super::to_svg_color_string;
use crate::PlotValue;
use crate::series::Series;
use crate::style::*;
use pigment::{Color, color};
use svg::Document;
use svg::node::Text as SvgNodeText;
use svg::node::element::{Rectangle, Text};

pub fn draw_legend<T: PlotValue>(
    document: Document,
    data: &[Series<T>],
    font: &str,
    legend_config: &LegendConfig,
    legend_x_base: f32,
    legend_y_base: f32,
    color_fn: fn(&str) -> Option<Color>,
    legend_actual_box_width: f32,
    legend_height: f32,
) -> Document {
    let legend_box_svg = Rectangle::new()
        .set("x", legend_x_base)
        .set("y", legend_y_base)
        .set("width", legend_actual_box_width)
        .set("height", legend_height)
        .set("fill", "white")
        .set("stroke", to_svg_color_string(&legend_config.border_color))
        .set("stroke-width", 1.0);
    let mut document = document.add(legend_box_svg);
    for (i, series) in data.iter().enumerate() {
        let item_base_y =
            legend_y_base + legend_config.padding + i as f32 * legend_config.item_height;
        let swatch_x = legend_x_base + legend_config.padding;
        let swatch_y =
            item_base_y + (legend_config.item_height - legend_config.item_height * 0.8) / 2.0;
        let color_val = match color_fn(series.color) {
            Some(c) => c,
            None => color("Black").unwrap(),
        };
        let swatch_svg = Rectangle::new()
            .set("x", swatch_x)
            .set("y", swatch_y)
            .set("width", legend_config.color_swatch_width)
            .set("height", legend_config.item_height * 0.8)
            .set("fill", to_svg_color_string(&color_val));
        document = document.add(swatch_svg);
        let text_x = swatch_x + legend_config.color_swatch_width + legend_config.text_offset;
        let text_y = item_base_y + legend_config.item_height / 2.0;
        let legend_text_svg = Text::new()
            .set("x", text_x)
            .set("y", text_y)
            .set("font-family", font)
            .set("font-size", legend_config.font_size)
            .set("fill", to_svg_color_string(&legend_config.text_color))
            .set("text-anchor", "start")
            .set("dominant-baseline", "middle")
            .add(SvgNodeText::new(series.name));
        document = document.add(legend_text_svg);
    }
    document
}
