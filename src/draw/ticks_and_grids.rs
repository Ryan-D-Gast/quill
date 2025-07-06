use super::to_svg_color_string;
use crate::elements::{Axis, Grid, Scale, Tick};
use crate::style::*;
use svg::Document;
use svg::node::Text as SvgNodeText;
use svg::node::element::{Line as SvgLine, Text};

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

    // Determine Y-axis scaling factor
    let mut y_scale_factor = 1.0;
    let mut y_scale_exponent = 0;

    match tick_config.y_scale_type {
        Scale::None => {
            // No scaling, factor remains 1.0, exponent 0
        }
        Scale::Log => {
            // For logarithmic scale, we don't use a simple scaling factor
            // The tick values should already be in log space if using log scale
            // This will need more comprehensive implementation in the future
        }
        Scale::Scientific | Scale::Engineering => {
            let mut max_tick_abs = 0.0;
            for &tick_val in y_ticks.iter() {
                if tick_val.abs() > max_tick_abs {
                    max_tick_abs = tick_val.abs();
                }
            }

            if max_tick_abs > 0.0 {
                let mut current_power = 0;
                let temp_max = max_tick_abs;

                if tick_config.y_scale_type == Scale::Scientific {
                    // Scientific: normalize to 1.xxxx
                    if temp_max >= 10.0 || (temp_max < 1.0 && temp_max > 0.0) {
                        current_power = temp_max.log10().floor() as i32;
                    }
                } else {
                    // Engineering: normalize to xxx.xxxx with exponent multiple of 3
                    let exp = temp_max.log10().floor() as i32;
                    current_power = (exp / 3) * 3;
                    // Adjust temp_max to reflect the engineering scaling for the following format call, if needed
                    // This ensures that the number of digits before decimal is between 1 and 3.
                    let scaled_val_check = temp_max / 10.0_f32.powi(current_power);
                    if scaled_val_check >= 1000.0 {
                        current_power += 3;
                    } else if scaled_val_check < 1.0 && scaled_val_check > 0.0 {
                        // this case should ideally not be hit if numbers are typical positives
                        // but if max_tick_abs was < 1 initially, engineering might make it 0.xxx * 10^0
                        // or for very small numbers like 0.000123 -> 123 * 10^-6
                        // if current_power is 0 and val is < 1, we might want to shift to e.g. 123u * 10^-3
                        if current_power == 0 && scaled_val_check < 1.0 {
                            let sub_exp = scaled_val_check.log10().floor() as i32;
                            current_power = ((sub_exp - 2) / 3) * 3; // Aim for xxx.yyy, so shift by 2 more than usual
                        }
                    }
                }

                if current_power != 0 {
                    y_scale_factor = 10.0_f32.powi(current_power);
                    y_scale_exponent = current_power;
                }
            }
        }
    }

    // Determine X-axis scaling factor
    let mut x_scale_factor = 1.0;
    let mut x_scale_exponent = 0;

    match tick_config.x_scale_type {
        Scale::None => {
            // No scaling, factor remains 1.0, exponent 0
        }
        Scale::Log => {
            // For logarithmic scale, we don't use a simple scaling factor
            // The tick values should already be in log space if using log scale
            // This will need more comprehensive implementation in the future
        }
        Scale::Scientific | Scale::Engineering => {
            let mut max_tick_abs = 0.0;
            for &tick_val in x_ticks.iter() {
                if tick_val.abs() > max_tick_abs {
                    max_tick_abs = tick_val.abs();
                }
            }

            if max_tick_abs > 0.0 {
                let mut current_power = 0;
                let temp_max = max_tick_abs;

                if tick_config.x_scale_type == Scale::Scientific {
                    // Scientific: normalize to 1.xxxx
                    if temp_max >= 10.0 || (temp_max < 1.0 && temp_max > 0.0) {
                        current_power = temp_max.log10().floor() as i32;
                    }
                } else {
                    // Engineering: normalize to xxx.xxxx with exponent multiple of 3
                    let exp = temp_max.log10().floor() as i32;
                    current_power = (exp / 3) * 3;
                    let scaled_val_check = temp_max / 10.0_f32.powi(current_power);
                    if scaled_val_check >= 1000.0 {
                        current_power += 3;
                    } else if scaled_val_check < 1.0 && scaled_val_check > 0.0 {
                        if current_power == 0 && scaled_val_check < 1.0 {
                            let sub_exp = scaled_val_check.log10().floor() as i32;
                            current_power = ((sub_exp - 2) / 3) * 3;
                        }
                    }
                }

                if current_power != 0 {
                    x_scale_factor = 10.0_f32.powi(current_power);
                    x_scale_exponent = current_power;
                }
            }
        }
    }

    for &tick_val in x_ticks.iter() {
        let screen_x = map_x(tick_val);
        let is_origin = (screen_x - plot_area_x_start).abs() < 0.1;
        if screen_x >= plot_area_x_start - 0.1
            && screen_x <= plot_area_x_start + plot_area_width + 0.1
        {
            match grid {
                Grid::None => {}
                Grid::Solid | Grid::Dashed | Grid::Dotted => {
                    let mut skip_grid_line = false;
                    if is_origin {
                        skip_grid_line = true;
                    }
                    if axis == Axis::Box
                        && (screen_x - (plot_area_x_start + plot_area_width)).abs() < 0.1
                    {
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
                        let tick_label_text_bottom = if tick_config.x_scale_type == Scale::Log {
                            // For log scale, always use scientific notation like "10³"
                            let abs_value = tick_val.abs();
                            if abs_value == 0.0 {
                                "0".to_string()
                            } else {
                                let log_value = abs_value.log10();
                                if (log_value.round() - log_value).abs() < 0.001 {
                                    let exponent = log_value.round() as i32;
                                    // Always use scientific notation: "10^exponent"
                                    format!("10^{}", exponent)
                                } else {
                                    // For intermediate values, use coefficient·10^exponent format
                                    let coefficient = tick_val / 10.0_f32.powi(log_value.floor() as i32);
                                    let exponent = log_value.floor() as i32;
                                    if (coefficient - 1.0).abs() < 0.001 {
                                        format!("10^{}", exponent)
                                    } else {
                                        format!("{:.1}·10^{}", coefficient, exponent)
                                    }
                                }
                            }
                        } else {
                            format!("{:.1}", tick_val / x_scale_factor)
                        };

                        // Handle logarithmic labels with proper superscript formatting for x-axis
                        if tick_config.x_scale_type == Scale::Log && (tick_label_text_bottom.contains("10^") || tick_label_text_bottom.contains("·10^")) {
                            // Handle both "10^exponent" and "coefficient·10^exponent" formats
                            if let Some(cap) = tick_label_text_bottom.strip_prefix("10^") {
                                // Simple "10^exponent" format
                                let exponent = cap.parse::<i32>().unwrap_or(0);
                                
                                let base_text_node = SvgNodeText::new("10");
                                let exponent_tspan = svg::node::element::TSpan::new()
                                    .set("dy", "-0.4em") // Shift exponent upwards
                                    .set("dx", "-0.2em") // Shift left to align with base
                                    .add(SvgNodeText::new(exponent.to_string()));
                                
                                let tick_label_svg_bottom = Text::new()
                                    .set("x", screen_x)
                                    .set("y", tick_y_bottom + tick_label_offset)
                                    .set("font-family", font)
                                    .set("font-size", tick_config.font_size)
                                    .set("fill", tick_label_color_svg.clone())
                                    .set("text-anchor", "middle")
                                    .set("dominant-baseline", "hanging")
                                    .add(base_text_node)
                                    .add(exponent_tspan);
                                document = document.add(tick_label_svg_bottom);
                            } else if let Some(pos) = tick_label_text_bottom.find("·10^") {
                                // "coefficient·10^exponent" format
                                let coefficient = &tick_label_text_bottom[..pos];
                                let exponent_str = &tick_label_text_bottom[pos + 4..]; // Skip "·10^"
                                let exponent = exponent_str.parse::<i32>().unwrap_or(0);
                                
                                let base_text_node = SvgNodeText::new(&format!("{}·10", coefficient));
                                let exponent_tspan = svg::node::element::TSpan::new()
                                    .set("dy", "-0.4em") // Shift exponent upwards
                                    .set("dx", "-0.2em") // Shift left to align with base
                                    .add(SvgNodeText::new(exponent.to_string()));
                                
                                let tick_label_svg_bottom = Text::new()
                                    .set("x", screen_x)
                                    .set("y", tick_y_bottom + tick_label_offset)
                                    .set("font-family", font)
                                    .set("font-size", tick_config.font_size)
                                    .set("fill", tick_label_color_svg.clone())
                                    .set("text-anchor", "middle")
                                    .set("dominant-baseline", "hanging")
                                    .add(base_text_node)
                                    .add(exponent_tspan);
                                document = document.add(tick_label_svg_bottom);
                            } else {
                                // Fallback to simple text
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
                        } else {
                            // Normal text formatting for non-logarithmic or simple logarithmic labels
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
    // Draw Y-axis scale factor label if needed
    if y_scale_factor != 1.0 && tick_config.y_scale_type != Scale::None && tick_config.y_scale_type != Scale::Log {
        let exponent_str = y_scale_exponent;
        let base_text_node = SvgNodeText::new("·10");

        let exponent_tspan = svg::node::element::TSpan::new()
            .set("dy", "-0.4em") // Shift exponent upwards. Adjust value if needed.
            .set("dx", "-0.2em") // Shift left to align with base
            .add(SvgNodeText::new(exponent_str.to_string()));

        let scale_label_svg = Text::new()
            .set("x", plot_area_x_start + tick_config.text_padding)
            .set("y", plot_area_y_start - tick_config.text_padding)
            .set("font-family", font)
            .set("font-size", tick_config.font_size)
            .set("fill", tick_label_color_svg.clone())
            .set("text-anchor", "start")
            .set("dominant-baseline", "text-after-edge")
            .add(base_text_node)
            .add(exponent_tspan);
        document = document.add(scale_label_svg);
    }

    // Draw X-axis scale factor label if needed
    if x_scale_factor != 1.0 && tick_config.x_scale_type != Scale::None && tick_config.x_scale_type != Scale::Log {
        let exponent_str = x_scale_exponent;
        let base_text_node = SvgNodeText::new("·10");

        let exponent_tspan = svg::node::element::TSpan::new()
            .set("dy", "-0.4em") // Shift exponent upwards
            .set("dx", "-0.2em") // Shift left to align with base
            .add(SvgNodeText::new(exponent_str.to_string()));

        let scale_label_svg = Text::new()
            .set("x", plot_area_x_start + plot_area_width - tick_config.text_padding)
            .set("y", plot_area_y_start + plot_area_height + tick_config.font_size + tick_config.text_padding * 2.0)
            .set("font-family", font)
            .set("font-size", tick_config.font_size)
            .set("fill", tick_label_color_svg.clone())
            .set("text-anchor", "end")
            .set("dominant-baseline", "text-before-edge")
            .add(base_text_node)
            .add(exponent_tspan);
        document = document.add(scale_label_svg);
    }
    for &tick_val in y_ticks.iter() {
        let screen_y = map_y(tick_val);
        if screen_y >= plot_area_y_start - 0.1
            && screen_y <= plot_area_y_start + plot_area_height + 0.1
        {
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
                let display_val = tick_val / y_scale_factor;
                let tick_label_text = if tick_config.y_scale_type == Scale::Log {
                    // For log scale, always use scientific notation like "10³"
                    let abs_value = tick_val.abs();
                    if abs_value == 0.0 {
                        "0".to_string()
                    } else {
                        let log_value = abs_value.log10();
                        if (log_value.round() - log_value).abs() < 0.001 {
                            let exponent = log_value.round() as i32;
                            // Always use scientific notation: "10^exponent"
                            format!("10^{}", exponent)
                        } else {
                            // For intermediate values, use coefficient·10^exponent format
                            let coefficient = tick_val / 10.0_f32.powi(log_value.floor() as i32);
                            let exponent = log_value.floor() as i32;
                            if (coefficient - 1.0).abs() < 0.001 {
                                format!("10^{}", exponent)
                            } else {
                                format!("{:.1}·10^{}", coefficient, exponent)
                            }
                        }
                    }
                } else {
                    format!("{:.1}", display_val)
                };

                // Handle logarithmic labels with proper superscript formatting
                if tick_config.y_scale_type == Scale::Log && (tick_label_text.contains("10^") || tick_label_text.contains("·10^")) {
                    // Handle both "10^exponent" and "coefficient·10^exponent" formats
                    if let Some(cap) = tick_label_text.strip_prefix("10^") {
                        // Simple "10^exponent" format
                        let exponent = cap.parse::<i32>().unwrap_or(0);
                        
                        let base_text_node = SvgNodeText::new("10");
                        let exponent_tspan = svg::node::element::TSpan::new()
                            .set("dy", "-0.4em") // Shift exponent upwards
                            .set("dx", "-0.2em") // Shift left to align with base
                            .add(SvgNodeText::new(exponent.to_string()));
                        
                        let tick_label_svg = Text::new()
                            .set(
                                "x",
                                tick_x_left
                                    - tick_config.text_padding
                                    - (if tick_direction > 0.0 {
                                        tick_config.length
                                    } else {
                                        tick_config.length
                                    }),
                            )
                            .set("y", screen_y)
                            .set("font-family", font)
                            .set("font-size", tick_config.font_size)
                            .set("fill", tick_label_color_svg.clone())
                            .set("text-anchor", "end")
                            .set("dominant-baseline", "middle")
                            .add(base_text_node)
                            .add(exponent_tspan);
                        document = document.add(tick_label_svg);
                    } else if let Some(pos) = tick_label_text.find("·10^") {
                        // "coefficient·10^exponent" format
                        let coefficient = &tick_label_text[..pos];
                        let exponent_str = &tick_label_text[pos + 4..]; // Skip "·10^"
                        let exponent = exponent_str.parse::<i32>().unwrap_or(0);
                        
                        let base_text_node = SvgNodeText::new(&format!("{}·10", coefficient));
                        let exponent_tspan = svg::node::element::TSpan::new()
                            .set("dy", "-0.4em") // Shift exponent upwards
                            .set("dx", "-0.2em") // Shift left to align with base
                            .add(SvgNodeText::new(exponent.to_string()));
                        
                        let tick_label_svg = Text::new()
                            .set(
                                "x",
                                tick_x_left
                                    - tick_config.text_padding
                                    - (if tick_direction > 0.0 {
                                        tick_config.length
                                    } else {
                                        tick_config.length
                                    }),
                            )
                            .set("y", screen_y)
                            .set("font-family", font)
                            .set("font-size", tick_config.font_size)
                            .set("fill", tick_label_color_svg.clone())
                            .set("text-anchor", "end")
                            .set("dominant-baseline", "middle")
                            .add(base_text_node)
                            .add(exponent_tspan);
                        document = document.add(tick_label_svg);
                    } else {
                        // Fallback to simple text
                        let tick_label_svg = Text::new()
                            .set(
                                "x",
                                tick_x_left
                                    - tick_config.text_padding
                                    - (if tick_direction > 0.0 {
                                        tick_config.length
                                    } else {
                                        tick_config.length
                                    }),
                            )
                            .set("y", screen_y)
                            .set("font-family", font)
                            .set("font-size", tick_config.font_size)
                            .set("fill", tick_label_color_svg.clone())
                            .set("text-anchor", "end")
                            .set("dominant-baseline", "middle")
                            .add(SvgNodeText::new(tick_label_text));
                        document = document.add(tick_label_svg);
                    }
                } else {
                    // Normal text formatting for non-logarithmic or simple logarithmic labels
                    let tick_label_svg = Text::new()
                        .set(
                            "x",
                            tick_x_left
                                - tick_config.text_padding
                                - (if tick_direction > 0.0 {
                                    tick_config.length
                                } else {
                                    tick_config.length
                                }),
                        )
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
    }
    document
}
