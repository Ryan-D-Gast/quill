use bon::Builder;
use pigment::{color, Color};
use crate::{
    style::*,
    elements::*,
    series::Series,
};
use svg::node::element::{
    Path, Rectangle, Text as SvgText, Line as SvgLine, Circle, Definitions, ClipPath, Group,
    path::Data,
};
use svg::node::Text as SvgNodeText;
use svg::Document;
use std::io;

fn to_svg_color_string(color: &Color) -> String {
    let (r, g, b) = color.rgb();
    format!("#{:02x}{:02x}{:02x}", r, g, b)
}

#[derive(Builder)]
pub struct Plot {
    // --- Plot Settings ---
    #[builder(default = (800, 600))]
    pub dimensions: (i32, i32),
    #[builder(default = "".to_string())]
    pub title: String,
    #[builder(default = "".to_string())]
    pub x_label: String,
    #[builder(default = "".to_string())]
    pub y_label: String,
    #[builder(default = Range::Auto)]
    pub x_range: Range,
    #[builder(default = Range::Auto)]
    pub y_range: Range,
    #[builder(default = Legend::None)]
    pub legend: Legend,
    #[builder(default = Axis::Box)]
    pub axis: Axis,
    #[builder(default = Tick::Inward)]
    pub tick: Tick,
    #[builder(default = Grid::Solid)]
    pub grid: Grid,
    #[builder(default = "Times New Roman".to_string())]
    pub font: String, // Font name can remain for an abstract renderer

    // --- Style Configurations ---
    #[builder(default = Margin::default())]
    pub margin: Margin,
    #[builder(default = TitleConfig::default())]
    pub title_config: TitleConfig,
    #[builder(default = LabelConfig::default())]
    pub x_label_config: LabelConfig,
    #[builder(default = LabelConfig::default())]
    pub y_label_config: LabelConfig,
    #[builder(default = TickConfig::default())]
    pub tick_config: TickConfig,
    #[builder(default = LegendConfig::default())]
    pub legend_config: LegendConfig,
    #[builder(default = AxisConfig::default())]
    pub axis_config: AxisConfig,
    #[builder(default = GridConfig::default())]
    pub grid_config: GridConfig,

    // --- Data ---
    pub data: Vec<Series>,
}

impl Plot {
    /// Generates an SVG document representing the plot and saves it to a file.
    pub fn plot(&self, filename: &str) -> Result<(), io::Error> {
        let (total_width, total_height) = self.dimensions;
        let mut document = Document::new()
            .set("width", total_width)
            .set("height", total_height)
            .set("viewBox", (0, 0, total_width, total_height));

        // Background
        let background = Rectangle::new()
            .set("x", 0)
            .set("y", 0)
            .set("width", total_width)
            .set("height", total_height)
            .set("fill", "white");
        document = document.add(background);

        // Determine x_min, x_max, y_min, y_max based on Range
        let (actual_x_min, actual_x_max) = match self.x_range {
            Range::Auto => {
                if self.data.is_empty() || self.data.iter().all(|s| s.data.is_empty()) {
                    (0.0, 1.0)
                } else {
                    let mut min_x = f32::MAX;
                    let mut max_x = f32::MIN;
                    for series in &self.data {
                        for (x, _) in &series.data {
                            min_x = min_x.min(*x);
                            max_x = max_x.max(*x);
                        }
                    }
                    if (max_x - min_x).abs() < f32::EPSILON {
                        (min_x - 0.5, max_x + 0.5)
                    } else {
                        (min_x, max_x)
                    }
                }
            }
            Range::Manual { min, max } => (min, max),
        };

        let (actual_y_min, actual_y_max) = match self.y_range {
            Range::Auto => {
                if self.data.is_empty() || self.data.iter().all(|s| s.data.is_empty()) {
                    (0.0, 1.0)
                } else {
                    let mut min_y = f32::MAX;
                    let mut max_y = f32::MIN;
                    for series in &self.data {
                        for (_, y) in &series.data {
                            min_y = min_y.min(*y);
                            max_y = max_y.max(*y);
                        }
                    }
                    if (max_y - min_y).abs() < f32::EPSILON {
                        (min_y - 0.5, max_y + 0.5)
                    } else {
                        (min_y, max_y)
                    }
                }
            }
            Range::Manual { min, max } => (min, max),
        };

        // Calculate legend dimensions
        let mut calculated_max_series_name_width = 0.0f32;
        if self.legend != Legend::None && !self.data.is_empty() {
            calculated_max_series_name_width = self.data.iter()
                .map(|s| s.name.len() as f32 * self.legend_config.font_size * 0.6)
                .fold(0.0f32, |a, b| a.max(b));
        }

        let legend_actual_box_width = if self.legend != Legend::None && !self.data.is_empty() {
            self.legend_config.color_swatch_width + self.legend_config.text_offset + calculated_max_series_name_width + self.legend_config.padding * 2.0
        } else {
            0.0
        };
        let legend_height = if self.legend != Legend::None && !self.data.is_empty() {
            self.data.len()  as f32 * self.legend_config.item_height + self.legend_config.padding * 2.0
        } else {
            0.0
        };

        // Adjust margins based on legend position
        let mut current_effective_margin_left = self.margin.left;
        let mut current_effective_margin_right = self.margin.right;
        let current_effective_margin_top = self.margin.top;
        let current_effective_margin_bottom = self.margin.bottom;

        if self.legend != Legend::None && !self.data.is_empty() {
            match self.legend {
                Legend::TopRightOutside | Legend::RightCenterOutside | Legend::BottomRightOutside => {
                    current_effective_margin_right += legend_actual_box_width + self.legend_config.padding;
                }
                Legend::TopLeftOutside | Legend::LeftCenterOutside | Legend::BottomLeftOutside => {
                    current_effective_margin_left += legend_actual_box_width + self.legend_config.padding;
                }
                _ => {}
            }
        }

        // Calculate plot area dimensions
        let plot_area_x_start = current_effective_margin_left;
        let plot_area_y_start = current_effective_margin_top;
        let plot_area_width = total_width as f32 - current_effective_margin_left - current_effective_margin_right;
        let plot_area_height = total_height as f32 - current_effective_margin_top - current_effective_margin_bottom;

        if plot_area_width <= 0.0 || plot_area_height <= 0.0 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Plot area is too small (width: {}, height: {}). Check dimensions and margins.", plot_area_width, plot_area_height)));
        }

        // Helper closures to map data coordinates to screen coordinates
        let map_x = |data_x: f32| -> f32 {
            if (actual_x_max - actual_x_min).abs() < f32::EPSILON {
                plot_area_x_start + plot_area_width / 2.0
            } else {
                plot_area_x_start + ((data_x - actual_x_min) / (actual_x_max - actual_x_min) * plot_area_width)
            }
        };
        let map_y = |data_y: f32| -> f32 {
            if (actual_y_max - actual_y_min).abs() < f32::EPSILON {
                plot_area_y_start + plot_area_height / 2.0
            } else {
                plot_area_y_start + plot_area_height - ((data_y - actual_y_min) / (actual_y_max - actual_y_min) * plot_area_height)
            }
        };

        // --- Draw Title ---
        if !self.title.is_empty() {
            let title_text_x = plot_area_x_start + plot_area_width / 2.0;
            let title_text_y = current_effective_margin_top * 0.5;
            let title_svg = SvgText::new()
                .set("x", title_text_x)
                .set("y", title_text_y)
                .set("font-family", self.font.clone())
                .set("font-size", self.title_config.font_size)
                .set("fill", to_svg_color_string(&self.title_config.color))
                .set("text-anchor", "middle")
                .set("dominant-baseline", "middle")
                .add(SvgNodeText::new(self.title.clone()));
            document = document.add(title_svg);
        }

        // --- Draw X-axis Label ---
        if !self.x_label.is_empty() {
            let x_label_text_x = plot_area_x_start + plot_area_width / 2.0;
            let x_label_text_y = plot_area_y_start + plot_area_height + current_effective_margin_bottom * 0.5;
            let x_label_svg = SvgText::new()
                .set("x", x_label_text_x)
                .set("y", x_label_text_y)
                .set("font-family", self.font.clone())
                .set("font-size", self.x_label_config.font_size)
                .set("fill", to_svg_color_string(&self.x_label_config.color))
                .set("text-anchor", "middle")
                .set("dominant-baseline", "middle")
                .add(SvgNodeText::new(self.x_label.clone()));
            document = document.add(x_label_svg);
        }

        // --- Draw Y-axis Label ---
        if !self.y_label.is_empty() {
            let y_label_text_x = current_effective_margin_left * 0.3;
            let y_label_text_y = plot_area_y_start + plot_area_height / 2.0;
            let y_label_svg = SvgText::new()
                .set("x", y_label_text_x)
                .set("y", y_label_text_y)
                .set("font-family", self.font.clone())
                .set("font-size", self.y_label_config.font_size)
                .set("fill", to_svg_color_string(&self.y_label_config.color))
                .set("text-anchor", "middle")
                .set("dominant-baseline", "middle")
                .set("transform", format!("rotate(-90, {}, {})", y_label_text_x, y_label_text_y))
                .add(SvgNodeText::new(self.y_label.clone()));
            document = document.add(y_label_svg);
        }

        // --- Draw Axis Lines ---
        let axis_color = to_svg_color_string(&self.axis_config.color);
        let axis_stroke_width = self.axis_config.line_width;

        match self.axis {
            Axis::BottomLeft => {
                let x_axis_line = SvgLine::new()
                    .set("x1", plot_area_x_start)
                    .set("y1", plot_area_y_start + plot_area_height)
                    .set("x2", plot_area_x_start + plot_area_width)
                    .set("y2", plot_area_y_start + plot_area_height)
                    .set("stroke", axis_color.clone())
                    .set("stroke-width", axis_stroke_width);
                document = document.add(x_axis_line);

                let y_axis_line = SvgLine::new()
                    .set("x1", plot_area_x_start)
                    .set("y1", plot_area_y_start)
                    .set("x2", plot_area_x_start)
                    .set("y2", plot_area_y_start + plot_area_height)
                    .set("stroke", axis_color)
                    .set("stroke-width", axis_stroke_width);
                document = document.add(y_axis_line);
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
                document = document.add(box_rect);
            }
        }

        // --- Tick Marks, Grid Lines, and Tick Labels ---
        let num_x_ticks = (plot_area_width / self.tick_config.density_x).max(2.0) as usize;
        let num_y_ticks = (plot_area_height / self.tick_config.density_y).max(2.0) as usize;

        let calculate_ticks = |min_val: f32, max_val: f32, max_ticks: usize| -> Vec<f32> {
            if (max_val - min_val).abs() < f32::EPSILON { return vec![min_val]; }
            let range = max_val - min_val;
            let rough_step = range / (max_ticks.saturating_sub(1) as f32).max(1.0);
            if rough_step == 0.0 { return vec![min_val]; }
            let exponent = rough_step.log10().floor();
            let fraction = rough_step / 10f32.powf(exponent);
            let nice_fraction = if fraction < 1.5 { 1.0 }
            else if fraction < 3.5 { 2.0 }
            else if fraction < 7.5 { 5.0 }
            else { 10.0 };
            let step = nice_fraction * 10f32.powf(exponent);
            if step == 0.0 { return vec![min_val, max_val].into_iter().collect() }

            let start_tick = (min_val / step).ceil() * step;
            let mut ticks = Vec::new();
            let mut current_tick = start_tick;
            while current_tick <= max_val + step * 0.5 {
                ticks.push(current_tick);
                current_tick += step;
                if ticks.len() > max_ticks * 2 { break; }
            }
             if ticks.is_empty() {
                if min_val == max_val { ticks.push(min_val); }
                else { ticks.extend_from_slice(&[min_val, max_val]); }
            } else if ticks.len() == 1 && min_val != max_val {
                 ticks.push(max_val);
            }
            ticks
        };

        let x_ticks = calculate_ticks(actual_x_min, actual_x_max, num_x_ticks);
        let y_ticks = calculate_ticks(actual_y_min, actual_y_max, num_y_ticks);
        
        let tick_label_color_svg = to_svg_color_string(&self.tick_config.label_color);
        let tick_line_color_svg = to_svg_color_string(&self.tick_config.line_color);
        let grid_line_color_svg = to_svg_color_string(&self.grid_config.color);

        // X Ticks and Grid Lines
        for &tick_val in &x_ticks {
            let screen_x = map_x(tick_val);
            if screen_x >= plot_area_x_start - 0.1 && screen_x <= plot_area_x_start + plot_area_width + 0.1 {
                if self.grid != Grid::None {
                    let mut skip_grid_line = false;
                    if (screen_x - plot_area_x_start).abs() < 0.1 {
                        skip_grid_line = true;
                    }
                    if self.axis == Axis::Box && (screen_x - (plot_area_x_start + plot_area_width)).abs() < 0.1 {
                        skip_grid_line = true;
                    }

                    if !skip_grid_line {
                        let mut grid_line = SvgLine::new()
                            .set("x1", screen_x)
                            .set("y1", plot_area_y_start)
                            .set("x2", screen_x)
                            .set("y2", plot_area_y_start + plot_area_height)
                            .set("stroke", grid_line_color_svg.clone())
                            .set("stroke-width", self.grid_config.line_width);
                        if self.grid == Grid::Dashed {
                            grid_line = grid_line.set("stroke-dasharray", "4 4");
                        }
                        document = document.add(grid_line);
                    }
                }

                if self.tick != Tick::None {
                    let tick_direction = if self.tick == Tick::Inward { -1.0 } else { 1.0 };
                    let tick_y_bottom = plot_area_y_start + plot_area_height;
                    let tick_y_top = plot_area_y_start;
                    let tick_label_offset = self.tick_config.font_size * 0.4 + 5.0;

                    match self.axis {
                        Axis::BottomLeft | Axis::Box => {
                            let tick_line_bottom = SvgLine::new()
                                .set("x1", screen_x)
                                .set("y1", tick_y_bottom)
                                .set("x2", screen_x)
                                .set("y2", tick_y_bottom + self.tick_config.length * tick_direction)
                                .set("stroke", tick_line_color_svg.clone())
                                .set("stroke-width", 1.0);
                            document = document.add(tick_line_bottom);

                            let tick_label_text_bottom = format!("{:.1}", tick_val);
                            let tick_label_svg_bottom = SvgText::new()
                                .set("x", screen_x)
                                .set("y", tick_y_bottom + tick_label_offset)
                                .set("font-family", self.font.clone())
                                .set("font-size", self.tick_config.font_size)
                                .set("fill", tick_label_color_svg.clone())
                                .set("text-anchor", "middle")
                                .set("dominant-baseline", "hanging")
                                .add(SvgNodeText::new(tick_label_text_bottom));
                            document = document.add(tick_label_svg_bottom);
                        }
                    }
                    if self.axis == Axis::Box {
                         let tick_line_top = SvgLine::new()
                            .set("x1", screen_x)
                            .set("y1", tick_y_top)
                            .set("x2", screen_x)
                            .set("y2", tick_y_top - self.tick_config.length * tick_direction)
                            .set("stroke", tick_line_color_svg.clone())
                            .set("stroke-width", 1.0);
                        document = document.add(tick_line_top);
                    }
                }
            }
        }

        // Y Ticks and Grid Lines
        for &tick_val in &y_ticks {
            let screen_y = map_y(tick_val);
             if screen_y >= plot_area_y_start - 0.1 && screen_y <= plot_area_y_start + plot_area_height + 0.1 {
                if self.grid != Grid::None {
                    let mut skip_grid_line = false;
                    if (screen_y - (plot_area_y_start + plot_area_height)).abs() < 0.1 {
                        skip_grid_line = true;
                    }
                    if self.axis == Axis::Box && (screen_y - plot_area_y_start).abs() < 0.1 {
                        skip_grid_line = true;
                    }

                    if !skip_grid_line {
                        let mut grid_line = SvgLine::new()
                            .set("x1", plot_area_x_start)
                            .set("y1", screen_y)
                            .set("x2", plot_area_x_start + plot_area_width)
                            .set("y2", screen_y)
                            .set("stroke", grid_line_color_svg.clone())
                            .set("stroke-width", self.grid_config.line_width);
                        if self.grid == Grid::Dashed {
                            grid_line = grid_line.set("stroke-dasharray", "4 4");
                        }
                        document = document.add(grid_line);
                    }
                }

                if self.tick != Tick::None {
                    let tick_direction = if self.tick == Tick::Inward { -1.0 } else { 1.0 };
                    let tick_x_left = plot_area_x_start;
                    let tick_x_right = plot_area_x_start + plot_area_width;

                    match self.axis {
                        Axis::BottomLeft | Axis::Box => {
                            let tick_line_left = SvgLine::new()
                                .set("x1", tick_x_left)
                                .set("y1", screen_y)
                                .set("x2", tick_x_left - self.tick_config.length * tick_direction)
                                .set("y2", screen_y)
                                .set("stroke", tick_line_color_svg.clone())
                                .set("stroke-width", 1.0);
                            document = document.add(tick_line_left);
                        }
                    }
                     if self.axis == Axis::Box {
                        let tick_line_right = SvgLine::new()
                            .set("x1", tick_x_right)
                            .set("y1", screen_y)
                            .set("x2", tick_x_right + self.tick_config.length * tick_direction)
                            .set("y2", screen_y)
                            .set("stroke", tick_line_color_svg.clone())
                            .set("stroke-width", 1.0);
                        document = document.add(tick_line_right);
                    }

                    let tick_label_text = format!("{:.1}", tick_val);
                    let tick_label_svg = SvgText::new()
                        .set("x", tick_x_left - self.tick_config.text_padding - (if tick_direction > 0.0 { 0.0 } else { self.tick_config.length }))
                        .set("y", screen_y)
                        .set("font-family", self.font.clone())
                        .set("font-size", self.tick_config.font_size)
                        .set("fill", tick_label_color_svg.clone())
                        .set("text-anchor", "end")
                        .set("dominant-baseline", "middle")
                        .add(SvgNodeText::new(tick_label_text));
                    document = document.add(tick_label_svg);
                }
            }
        }
        
        // --- Clipping Path for Plot Area ---
        let clip_path_id = "plotAreaClip";
        let clip_rect = Rectangle::new()
            .set("x", plot_area_x_start)
            .set("y", plot_area_y_start)
            .set("width", plot_area_width)
            .set("height", plot_area_height);
        let clip_path = ClipPath::new().set("id", clip_path_id).add(clip_rect);
        let mut defs = Definitions::new();
        defs = defs.add(clip_path);
        document = document.add(defs);

        // Group for data series, applying the clip path
        let mut data_group = Group::new().set("clip-path", format!("url(#{})", clip_path_id));

        // --- Data Series Drawing ---
        for series in &self.data {
            let color_val = match pigment::color(&series.color) {
                Some(c) => c,
                None => color("Black").unwrap(),
            };
            let series_color_svg = to_svg_color_string(&color_val);

            // Line
            if series.line != Line::None && series.data.len() > 1 {
                let mut line_data = Data::new();
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

            // Markers
            if series.marker != Marker::None {
                let marker_size = series.marker_size;
                for &(data_x, data_y) in &series.data {
                    let screen_x = map_x(data_x);
                    let screen_y = map_y(data_y);

                    match series.marker {
                        Marker::Circle => {
                            let circle = Circle::new()
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
                            let cross_data = Data::new()
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
        document = document.add(data_group);
        
        // --- Legend Drawing ---
        if self.legend != Legend::None && !self.data.is_empty() {
            let legend_x_base;
            let legend_y_base;

            match self.legend {
                Legend::TopRightInside => {
                    legend_x_base = plot_area_x_start + plot_area_width - legend_actual_box_width - self.legend_config.padding;
                    legend_y_base = plot_area_y_start + self.legend_config.padding;
                }
                Legend::TopRightOutside => {
                    legend_x_base = total_width as f32 - current_effective_margin_right + self.legend_config.padding;
                    legend_y_base = plot_area_y_start + self.legend_config.padding;
                }
                Legend::BottomRightInside => {
                    legend_x_base = plot_area_x_start + plot_area_width - legend_actual_box_width - self.legend_config.padding;
                    legend_y_base = plot_area_y_start + plot_area_height - legend_height - self.legend_config.padding;
                }
                Legend::BottomRightOutside => {
                    legend_x_base = total_width as f32 - current_effective_margin_right + self.legend_config.padding;
                    legend_y_base = plot_area_y_start + plot_area_height - legend_height - self.legend_config.padding;
                }
                Legend::TopLeftInside => {
                    legend_x_base = plot_area_x_start + self.legend_config.padding;
                    legend_y_base = plot_area_y_start + self.legend_config.padding;
                }
                Legend::TopLeftOutside => {
                    legend_x_base = current_effective_margin_left - legend_actual_box_width - self.legend_config.padding;
                    legend_y_base = plot_area_y_start + self.legend_config.padding;
                }
                Legend::BottomLeftInside => {
                    legend_x_base = plot_area_x_start + self.legend_config.padding;
                    legend_y_base = plot_area_y_start + plot_area_height - legend_height - self.legend_config.padding;
                }
                Legend::BottomLeftOutside => {
                    legend_x_base = self.margin.left - legend_actual_box_width - self.legend_config.padding;
                    legend_y_base = plot_area_y_start + plot_area_height - legend_height - self.legend_config.padding;
                }
                Legend::RightCenterInside => {
                    legend_x_base = plot_area_x_start + plot_area_width - legend_actual_box_width - self.legend_config.padding;
                    legend_y_base = plot_area_y_start + (plot_area_height - legend_height) / 2.0;
                }
                Legend::RightCenterOutside => {
                    legend_x_base = total_width as f32 - current_effective_margin_right + self.legend_config.padding;
                    legend_y_base = plot_area_y_start + (plot_area_height - legend_height) / 2.0;
                }
                Legend::LeftCenterInside => {
                    legend_x_base = plot_area_x_start + self.legend_config.padding;
                    legend_y_base = plot_area_y_start + (plot_area_height - legend_height) / 2.0;
                }
                Legend::LeftCenterOutside => {
                    legend_x_base = self.margin.left - legend_actual_box_width - self.legend_config.padding;
                    legend_y_base = plot_area_y_start + (plot_area_height - legend_height) / 2.0;
                }
                Legend::TopCenter => {
                    legend_x_base = plot_area_x_start + (plot_area_width - legend_actual_box_width) / 2.0;
                    legend_y_base = plot_area_y_start + self.legend_config.padding;
                }
                Legend::BottomCenter => {
                    legend_x_base = plot_area_x_start + (plot_area_width - legend_actual_box_width) / 2.0;
                    legend_y_base = plot_area_y_start + plot_area_height - legend_height - self.legend_config.padding;
                }
                Legend::None => { legend_x_base = 0.0; legend_y_base = 0.0; }
            }

            let legend_box_svg = Rectangle::new()
                .set("x", legend_x_base)
                .set("y", legend_y_base)
                .set("width", legend_actual_box_width)
                .set("height", legend_height)
                .set("fill", "white")
                .set("stroke", to_svg_color_string(&self.legend_config.border_color))
                .set("stroke-width", 1.0);
            document = document.add(legend_box_svg);

            for (i, series) in self.data.iter().enumerate() {
                let item_base_y = legend_y_base + self.legend_config.padding + i as f32 * self.legend_config.item_height;
                let swatch_x = legend_x_base + self.legend_config.padding;
                let swatch_y = item_base_y + (self.legend_config.item_height - self.legend_config.item_height * 0.8) / 2.0;
                
                let color_val = match pigment::color(&series.color) {
                    Some(c) => c,
                    None => color("Black").unwrap(),
                };

                let swatch_svg = Rectangle::new()
                    .set("x", swatch_x)
                    .set("y", swatch_y)
                    .set("width", self.legend_config.color_swatch_width)
                    .set("height", self.legend_config.item_height * 0.8)
                    .set("fill", to_svg_color_string(&color_val));
                document = document.add(swatch_svg);

                let text_x = swatch_x + self.legend_config.color_swatch_width + self.legend_config.text_offset;
                let text_y = item_base_y + self.legend_config.item_height / 2.0;
                
                let legend_text_svg = SvgText::new()
                    .set("x", text_x)
                    .set("y", text_y)
                    .set("font-family", self.font.clone())
                    .set("font-size", self.legend_config.font_size)
                    .set("fill", to_svg_color_string(&self.legend_config.text_color))
                    .set("text-anchor", "start")
                    .set("dominant-baseline", "middle")
                    .add(SvgNodeText::new(series.name.clone()));
                document = document.add(legend_text_svg);
            }
        }
        svg::save(filename, &document)
    }
}