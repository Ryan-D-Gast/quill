use svg::node::element::{Text, Line as SvgLine};
use svg::node::Text as SvgNodeText;
use svg::Document;
use crate::style::*;
use crate::elements::{Axis, Grid, Tick};
use super::to_svg_color_string;

pub fn draw_ticks_and_grids<FX, FY>(
    document: Document,
    axis: Axis,
    tick: Tick,
    grid: Grid,
    tick_config: &TickConfig,
    grid_config: &GridConfig,
    font: &str,
    plot_area_x_start: f32,
    plot_area_y_start: f32,
    plot_area_width: f32,
    plot_area_height: f32,
    x_ticks: &[f32],
    y_ticks: &[f32],
    map_x: FX,
    map_y: FY,
) -> Document
where
    FX: Fn(f32) -> f32,
    FY: Fn(f32) -> f32,
{
    let tick_label_color_svg = to_svg_color_string(&tick_config.label_color);
    let tick_line_color_svg = to_svg_color_string(&tick_config.line_color);
    let grid_line_color_svg = to_svg_color_string(&grid_config.color);
    let mut document = document;
    for &tick_val in x_ticks.iter() {
        let screen_x = map_x(tick_val);
        let is_origin = (screen_x - plot_area_x_start).abs() < 0.1;
        if screen_x >= plot_area_x_start - 0.1 && screen_x <= plot_area_x_start + plot_area_width + 0.1 {
            match grid {
                Grid::None => {}
                Grid::Solid | Grid::Dashed | Grid::Dotted => {
                    let mut skip_grid_line = false;
                    if is_origin {
                        skip_grid_line = true;
                    }
                    if axis == Axis::Box && (screen_x - (plot_area_x_start + plot_area_width)).abs() < 0.1 {
                        skip_grid_line = true;
                    }
                    if !skip_grid_line {
                        let mut grid_line = SvgLine::new()
                            .set("x1", screen_x)
                            .set("y1", plot_area_y_start)
                            .set("x2", screen_x)
                            .set("y2", plot_area_y_start + plot_area_height)
                            .set("stroke", grid_line_color_svg.clone())
                            .set("stroke-width", grid_config.line_width);
                        match grid {
                            Grid::Dotted => {
                                grid_line = grid_line.set("stroke-dasharray", "1 2");
                            }
                            Grid::Dashed => {
                                grid_line = grid_line.set("stroke-dasharray", "4 4");
                            }
                            Grid::Solid | Grid::None => {}
                        }
                        document = document.add(grid_line);
                    }
                }
            }
            if tick != Tick::None {
                let tick_direction = if tick == Tick::Inward { -1.0 } else { 1.0 };
                let tick_y_bottom = plot_area_y_start + plot_area_height;
                let tick_y_top = plot_area_y_start;
                let tick_label_offset = tick_config.font_size * 0.4 + 5.0;
                match axis {
                    Axis::BottomLeft | Axis::Box => {
                        let tick_line_bottom = SvgLine::new()
                            .set("x1", screen_x)
                            .set("y1", tick_y_bottom)
                            .set("x2", screen_x)
                            .set("y2", tick_y_bottom + tick_config.length * tick_direction)
                            .set("stroke", tick_line_color_svg.clone())
                            .set("stroke-width", 1.0);
                        document = document.add(tick_line_bottom);
                        let tick_label_text_bottom = format!("{:.1}", tick_val);
                        let tick_label_svg_bottom = Text::new()
                            .set("x", screen_x)
                            .set("y", tick_y_bottom + tick_label_offset)
                            .set("font-family", font)
                            .set("font-size", tick_config.font_size)
                            .set("fill", tick_label_color_svg.clone())
                            .set("text-anchor", "middle")
                            .set("dominant-baseline", "hanging")
                            .add(SvgNodeText::new(tick_label_text_bottom));
                        document = document.add(tick_label_svg_bottom);
                    }
                }
                if axis == Axis::Box {
                    let tick_line_top = SvgLine::new()
                        .set("x1", screen_x)
                        .set("y1", tick_y_top)
                        .set("x2", screen_x)
                        .set("y2", tick_y_top - tick_config.length * tick_direction)
                        .set("stroke", tick_line_color_svg.clone())
                        .set("stroke-width", 1.0);
                    document = document.add(tick_line_top);
                }
            }
        }
    }
    for &tick_val in y_ticks.iter() {
        let screen_y = map_y(tick_val);
        if screen_y >= plot_area_y_start - 0.1 && screen_y <= plot_area_y_start + plot_area_height + 0.1 {
            match grid {
                Grid::None => {}
                Grid::Solid | Grid::Dashed | Grid::Dotted => {
                    let mut skip_grid_line = false;
                    if (screen_y - (plot_area_y_start + plot_area_height)).abs() < 0.1 {
                        skip_grid_line = true;
                    }
                    if axis == Axis::Box && (screen_y - plot_area_y_start).abs() < 0.1 {
                        skip_grid_line = true;
                    }
                    if !skip_grid_line {
                        let mut grid_line = SvgLine::new()
                            .set("x1", plot_area_x_start)
                            .set("y1", screen_y)
                            .set("x2", plot_area_x_start + plot_area_width)
                            .set("y2", screen_y)
                            .set("stroke", grid_line_color_svg.clone())
                            .set("stroke-width", grid_config.line_width);
                        match grid {
                            Grid::Dotted => {
                                grid_line = grid_line.set("stroke-dasharray", "1 2");
                            }
                            Grid::Dashed => {
                                grid_line = grid_line.set("stroke-dasharray", "4 4");
                            }
                            Grid::Solid | Grid::None => {}
                        }
                        document = document.add(grid_line);
                    }
                }
            }
            if tick != Tick::None {
                let tick_direction = if tick == Tick::Inward { -1.0 } else { 1.0 };
                let tick_x_left = plot_area_x_start;
                let tick_x_right = plot_area_x_start + plot_area_width;
                match axis {
                    Axis::BottomLeft | Axis::Box => {
                        let tick_line_left = SvgLine::new()
                            .set("x1", tick_x_left)
                            .set("y1", screen_y)
                            .set("x2", tick_x_left - tick_config.length * tick_direction)
                            .set("y2", screen_y)
                            .set("stroke", tick_line_color_svg.clone())
                            .set("stroke-width", 1.0);
                        document = document.add(tick_line_left);
                    }
                }
                if axis == Axis::Box {
                    let tick_line_right = SvgLine::new()
                        .set("x1", tick_x_right)
                        .set("y1", screen_y)
                        .set("x2", tick_x_right + tick_config.length * tick_direction)
                        .set("y2", screen_y)
                        .set("stroke", tick_line_color_svg.clone())
                        .set("stroke-width", 1.0);
                    document = document.add(tick_line_right);
                }
                let tick_label_text = format!("{:.1}", tick_val);
                let tick_label_svg = Text::new()
                    .set("x", tick_x_left - tick_config.text_padding - (if tick_direction > 0.0 { 0.0 } else { tick_config.length }))
                    .set("y", screen_y)
                    .set("font-family", font)
                    .set("font-size", tick_config.font_size)
                    .set("fill", tick_label_color_svg.clone())
                    .set("text-anchor", "end")
                    .set("dominant-baseline", "middle")
                    .add(SvgNodeText::new(tick_label_text));
                document = document.add(tick_label_svg);
            }
        }
    }
    document
}