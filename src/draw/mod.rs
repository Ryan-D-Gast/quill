// Drawing helper functions for Plot SVG rendering
use svg::node::element::{Text, Line as SvgLine, Rectangle, Path, Group};
use svg::node::Text as SvgNodeText;
use svg::Document;
use pigment::{color, Color};
use crate::series::Series;
use crate::style::*;
use crate::elements::{Axis, Grid, Line, Marker, Tick};

fn to_svg_color_string(color: &Color) -> String {
    let (r, g, b) = color.rgb();
    format!("#{:02x}{:02x}{:02x}", r, g, b)
}

pub fn draw_title(document: Document, title: &str, font: &str, title_config: &TitleConfig, plot_area_x_start: f32, plot_area_width: f32, current_effective_margin_top: f32) -> Document {
    if !title.is_empty() {
        let title_text_x = plot_area_x_start + plot_area_width / 2.0;
        let title_text_y = current_effective_margin_top * 0.5;
        let title_svg = Text::new()
            .set("x", title_text_x)
            .set("y", title_text_y)
            .set("font-family", font)
            .set("font-size", title_config.font_size)
            .set("fill", to_svg_color_string(&title_config.color))
            .set("text-anchor", "middle")
            .set("dominant-baseline", "middle")
            .add(SvgNodeText::new(title));
        return document.add(title_svg);
    }
    document
}

pub fn draw_x_label(document: Document, x_label: &str, font: &str, x_label_config: &LabelConfig, plot_area_x_start: f32, plot_area_width: f32, plot_area_y_start: f32, plot_area_height: f32, current_effective_margin_bottom: f32) -> Document {
    if !x_label.is_empty() {
        let x_label_text_x = plot_area_x_start + plot_area_width / 2.0;
        let x_label_text_y = plot_area_y_start + plot_area_height + current_effective_margin_bottom * 0.5;
        let x_label_svg = Text::new()
            .set("x", x_label_text_x)
            .set("y", x_label_text_y)
            .set("font-family", font)
            .set("font-size", x_label_config.font_size)
            .set("fill", to_svg_color_string(&x_label_config.color))
            .set("text-anchor", "middle")
            .set("dominant-baseline", "middle")
            .add(SvgNodeText::new(x_label));
        return document.add(x_label_svg);
    }
    document
}

pub fn draw_y_label(document: Document, y_label: &str, font: &str, y_label_config: &LabelConfig, current_effective_margin_left: f32, plot_area_y_start: f32, plot_area_height: f32) -> Document {
    if !y_label.is_empty() {
        let y_label_text_x = current_effective_margin_left * 0.3;
        let y_label_text_y = plot_area_y_start + plot_area_height / 2.0;
        let y_label_svg = Text::new()
            .set("x", y_label_text_x)
            .set("y", y_label_text_y)
            .set("font-family", font)
            .set("font-size", y_label_config.font_size)
            .set("fill", to_svg_color_string(&y_label_config.color))
            .set("text-anchor", "middle")
            .set("dominant-baseline", "middle")
            .set("transform", format!("rotate(-90, {}, {})", y_label_text_x, y_label_text_y))
            .add(SvgNodeText::new(y_label));
        return document.add(y_label_svg);
    }
    document
}

pub fn draw_axis_lines(document: Document, axis: Axis, axis_config: &AxisConfig, plot_area_x_start: f32, plot_area_y_start: f32, plot_area_width: f32, plot_area_height: f32) -> Document {
    let axis_color = to_svg_color_string(&axis_config.color);
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
    for &tick_val in x_ticks {
        let screen_x = map_x(tick_val);
        if screen_x >= plot_area_x_start - 0.1 && screen_x <= plot_area_x_start + plot_area_width + 0.1 {
            match grid {
                Grid::None => {}
                Grid::Solid | Grid::Dashed | Grid::Dotted => {
                    let mut skip_grid_line = false;
                    if (screen_x - plot_area_x_start).abs() < 0.1 {
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
                        if let Grid::Dashed = grid {
                            grid_line = grid_line.set("stroke-dasharray", "4 4");
                        }
                        document = document.add(grid_line);
                    }
                }
                // Add new grid options here
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
    for &tick_val in y_ticks {
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
                // Add new grid options here
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

pub fn draw_data_series<Fx, Fy>(data: &[Series], color_fn: fn(&str) -> Option<Color>, map_x: Fx, map_y: Fy) -> Group
where
    Fx: Fn(f32) -> f32,
    Fy: Fn(f32) -> f32,
{
    let mut data_group = Group::new().set("clip-path", "url(#plotAreaClip)");
    for series in data {
        let color_val = match color_fn(&series.color) {
            Some(c) => c,
            None => color("Black").unwrap(),
        };
        let series_color_svg = to_svg_color_string(&color_val);
        if series.line != Line::None && series.data.len() > 1 {
            let mut line_data = svg::node::element::path::Data::new();
            if let Some((first_x, first_y)) = series.data.first() {
                line_data = line_data.move_to((map_x(*first_x), map_y(*first_y)));
                for (x, y) in series.data.iter().skip(1) {
                    line_data = line_data.line_to((map_x(*x), map_y(*y)));
                }
            }
            let mut line_path = Path::new()
                .set("d", line_data)
                .set("fill", "none")
                .set("stroke", series_color_svg.clone())
                .set("stroke-width", series.line_width);
            if series.line == Line::Dashed {
                line_path = line_path.set("stroke-dasharray", "5 5");
            }
            data_group = data_group.add(line_path);
        }
        if series.marker != Marker::None {
            let marker_size = series.marker_size;
            for &(data_x, data_y) in &series.data {
                let screen_x = map_x(data_x);
                let screen_y = map_y(data_y);
                match series.marker {
                    Marker::Circle => {
                        let circle = svg::node::element::Circle::new()
                            .set("cx", screen_x)
                            .set("cy", screen_y)
                            .set("r", marker_size / 2.0)
                            .set("fill", series_color_svg.clone());
                        data_group = data_group.add(circle);
                    }
                    Marker::Square => {
                        let square = Rectangle::new()
                            .set("x", screen_x - marker_size / 2.0)
                            .set("y", screen_y - marker_size / 2.0)
                            .set("width", marker_size)
                            .set("height", marker_size)
                            .set("fill", series_color_svg.clone());
                        data_group = data_group.add(square);
                    }
                    Marker::Cross => {
                        let d = marker_size / 2.0;
                        let cross_data = svg::node::element::path::Data::new()
                            .move_to((screen_x - d, screen_y - d))
                            .line_to((screen_x + d, screen_y + d))
                            .move_to((screen_x - d, screen_y + d))
                            .line_to((screen_x + d, screen_y - d));
                        let cross_path = Path::new()
                            .set("d", cross_data)
                            .set("stroke", series_color_svg.clone())
                            .set("stroke-width", 1.0)
                            .set("fill", "none");
                        data_group = data_group.add(cross_path);
                    }
                    Marker::None => {}
                }
            }
        }
    }
    data_group
}

pub fn draw_legend(
    document: Document,
    data: &[Series],
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
        let item_base_y = legend_y_base + legend_config.padding + i as f32 * legend_config.item_height;
        let swatch_x = legend_x_base + legend_config.padding;
        let swatch_y = item_base_y + (legend_config.item_height - legend_config.item_height * 0.8) / 2.0;
        let color_val = match color_fn(&series.color) {
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
            .add(SvgNodeText::new(series.name.clone()));
        document = document.add(legend_text_svg);
    }
    document
}