//! Quill: A plotting library for Rust.

// TODO: Turn draft in this one file into multiple files and broken up components.
// Other things to add:
// - Axis tick settings like log scales specialized ticks etc.
// - Better legend styling
// - Move all these constant plot style qualties into the plot builder
// - Support annotations
// - Add caption below the plot
// - if y_min and x_min are the same use one number for the origin e.g. (0.0 y axis, 0.0 x axis) is rendered as one 0.0 at vertex of x-y axis

use bon::Builder;
use raqote::{DrawTarget, PathBuilder, Source, SolidSource, DrawOptions, StrokeStyle, Point};
use font_kit::{
    family_name::FamilyName,
    properties::Properties,
    source::SystemSource
};
use pigment::color;
use png::EncodingError;

mod elements;
pub use elements::*;

#[derive(Builder)]
pub struct Series {
    data: Vec<(f32, f32)>,
    #[builder(default = "".to_string())]
    name: String,
    #[builder(default = "Black".to_string())]
    color: String,
    #[builder(default = Line::Solid)]
    line: Line,
    #[builder(default = Marker::None)]
    marker: Marker,
}

#[derive(Builder)]
pub struct Plot {
    // --- Plot Settings ---
    #[builder(default = (800, 600))]
    dimensions: (i32, i32),
    #[builder(default = "".to_string())]
    title: String,
    #[builder(default = "".to_string())]
    x_label: String,
    #[builder(default = "".to_string())]
    y_label: String,
    #[builder(default = Range::Auto)]
    x_range: Range,
    #[builder(default = Range::Auto)]
    y_range: Range,
    #[builder(default = Legend::None)]
    legend: Legend,
    #[builder(default = Axis::Box)]
    axis: Axis,
    #[builder(default = Tick::Inward)]
    tick: Tick, // Controls tick direction (Inward/Outward) or disables them (None)
    #[builder(default = Grid::Solid)]
    grid: Grid, // Controls grid style (Solid/Dashed/None)
    #[builder(default = "Times New Roman".to_string())]
    font: String, // Global font family

    // --- Style Configurations ---
    #[builder(default = Margin::default())]
    margin: Margin,
    #[builder(default = TitleConfig::default())]
    title_config: TitleConfig,
    #[builder(default = LabelConfig::default())]
    x_label_config: LabelConfig,
    #[builder(default = LabelConfig::default())]
    y_label_config: LabelConfig,
    #[builder(default = TickConfig::default())]
    tick_config: TickConfig,
    #[builder(default = LegendConfig::default())]
    legend_config: LegendConfig,
    #[builder(default = AxisConfig::default())]
    axis_config: AxisConfig,
    #[builder(default = GridConfig::default())]
    grid_config: GridConfig,

    // --- Data ---
    data: Vec<Series>,
}

#[derive(Clone, Debug)]
pub struct Margin {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

impl Default for Margin {
    fn default() -> Self {
        Self {
            top: 60.0,
            bottom: 70.0,
            left: 80.0,
            right: 30.0,
        }
    }
}

// --- New Style Struct Definitions ---
#[derive(Clone, Debug)]
pub struct TitleConfig {
    pub font_size: f32,
    pub color: SolidSource,
}

impl Default for TitleConfig {
    fn default() -> Self {
        Self {
            font_size: 20.0,
            color: SolidSource { r: 0x00, g: 0x00, b: 0x00, a: 0xff }, // Black
        }
    }
}

#[derive(Clone, Debug)]
pub struct LabelConfig {
    pub font_size: f32,
    pub color: SolidSource,
}

impl Default for LabelConfig {
    fn default() -> Self {
        Self {
            font_size: 14.0,
            color: SolidSource { r: 0x00, g: 0x00, b: 0x00, a: 0xff }, // Black
        }
    }
}

#[derive(Clone, Debug)]
pub struct TickConfig {
    pub font_size: f32,
    pub label_color: SolidSource,
    pub line_color: SolidSource,
    pub length: f32,
    pub text_padding: f32,
    pub density_x: f32, // Target pixels per tick for X-axis
    pub density_y: f32, // Target pixels per tick for Y-axis
}

impl Default for TickConfig {
    fn default() -> Self {
        Self {
            font_size: 10.0,
            label_color: SolidSource { r: 0x00, g: 0x00, b: 0x00, a: 0xff }, // Black
            line_color: SolidSource { r: 0x00, g: 0x00, b: 0x00, a: 0xff },  // Black
            length: 5.0,
            text_padding: 3.0,
            density_x: 50.0,
            density_y: 50.0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct LegendConfig {
    pub font_size: f32,
    pub text_color: SolidSource,
    pub border_color: SolidSource,
    pub padding: f32,
    pub item_height: f32,
    pub color_swatch_width: f32,
    pub text_offset: f32,
}

impl Default for LegendConfig {
    fn default() -> Self {
        Self {
            font_size: 12.0,
            text_color: SolidSource { r: 0x00, g: 0x00, b: 0x00, a: 0xff }, // Black
            border_color: SolidSource { r: 0x00, g: 0x00, b: 0x00, a: 0xff }, // Black
            padding: 10.0,
            item_height: 18.0,
            color_swatch_width: 15.0,
            text_offset: 5.0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct AxisConfig {
    pub color: SolidSource,
    pub line_width: f32,
}

impl Default for AxisConfig {
    fn default() -> Self {
        Self {
            color: SolidSource { r: 0x00, g: 0x00, b: 0x00, a: 0xff }, // Black
            line_width: 1.5,
        }
    }
}

#[derive(Clone, Debug)]
pub struct GridConfig {
    pub color: SolidSource,
    pub line_width: f32,
}

impl Default for GridConfig {
    fn default() -> Self {
        Self {
            color: SolidSource { r: 0xcc, g: 0xcc, b: 0xcc, a: 0xff }, // Light gray
            line_width: 0.5,
        }
    }
}

impl Plot {
    /// Creates a png plot with the given dimensions, title, x and y labels, and data points.
    pub fn plot(&self, file: &str) -> Result<(), EncodingError> {
        let (total_width, total_height) = self.dimensions;
        let mut dt = DrawTarget::new(total_width, total_height);

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
                    // Add a small padding if min and max are the same
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
                    (0.0, 1.0) // Default range if no data
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

        // Load font
        let font_families = vec![FamilyName::Title(self.font.clone())];

        let font = SystemSource::new()
            .select_best_match(&font_families, &Properties::new())
            .unwrap_or_else(|_| {
                eprintln!("Warning: Specified font family '{:?}' not found or no suitable font. Falling back to generic sans-serif.", self.font);
                SystemSource::new()
                    .select_best_match(&[FamilyName::SansSerif], &Properties::new())
                    .expect("Failed to find any sans-serif font as fallback")
            })
            .load()
            .expect("Failed to load font");

        // Background
        let mut pb_bg = PathBuilder::new();
        pb_bg.rect(0., 0., total_width as f32, total_height as f32);
        let path_bg = pb_bg.finish();
        dt.fill(&path_bg, &Source::Solid(SolidSource { r: 0xff, g: 0xff, b: 0xff, a: 0xff }), &DrawOptions::new());

        // Calculate legend dimensions (width and height)
        let mut calculated_max_series_name_width = 0.0f32;
        if self.legend != Legend::None && !self.data.is_empty() {
            calculated_max_series_name_width = self.data.iter()
                .map(|s| s.name.len() as f32 * self.legend_config.font_size * 0.6) // Use legend_config
                .fold(0.0f32, |a, b| a.max(b));
        }

        let legend_actual_box_width = if self.legend != Legend::None && !self.data.is_empty() {
            self.legend_config.color_swatch_width + self.legend_config.text_offset + calculated_max_series_name_width + self.legend_config.padding * 2.0 // Use legend_config
        } else {
            0.0
        };
        let legend_height = if self.legend != Legend::None && !self.data.is_empty() {
            self.data.len()  as f32 * self.legend_config.item_height + self.legend_config.padding * 2.0 // Use legend_config
        } else {
            0.0
        };

        // Adjust margins based on legend position if legend is active
        let mut current_effective_margin_left = self.margin.left;
        let mut current_effective_margin_right = self.margin.right;

        // These can be made mut later if legend bottom or top of title/x-axis label is supported in future
        let current_effective_margin_top = self.margin.top;
        let current_effective_margin_bottom = self.margin.bottom;

        if self.legend != Legend::None && !self.data.is_empty() {
            match self.legend {
                Legend::TopRightOutside | Legend::RightCenterOutside | Legend::BottomRightOutside => {
                    current_effective_margin_right += legend_actual_box_width + self.legend_config.padding; // Use legend_config
                }
                Legend::TopLeftOutside | Legend::LeftCenterOutside | Legend::BottomLeftOutside => {
                    current_effective_margin_left += legend_actual_box_width + self.legend_config.padding; // Use legend_config
                }
                // Removed TopCenterOutside and BottomCenterOutside from margin adjustments
                // For Inside, TopCenter, BottomCenter, None, no specific margin adjustment here as legend is within plot area or not present.
                _ => {}
            }
        }

        // Calculate plot area dimensions using potentially adjusted margins
        let plot_area_x_start = current_effective_margin_left;
        let plot_area_y_start = current_effective_margin_top;
        let plot_area_width = total_width as f32 - current_effective_margin_left - current_effective_margin_right;
        let plot_area_height = total_height as f32 - current_effective_margin_top - current_effective_margin_bottom;

        if plot_area_width <= 0.0 || plot_area_height <= 0.0 {
            eprintln!(
                "Plot area is too small (width: {}, height: {}). Check dimensions and margins.",
                plot_area_width,
                plot_area_height
            );
            // Draw an error message on the image if it's too small
            dt.draw_text(
                &font, 
                12.0, // Default error font size
                "Error: Plot area too small", 
                Point::new(10.0, 20.0), 
                &Source::Solid(self.title_config.color), // Use a default color like title's
                &DrawOptions::new()
            );
            return dt.write_png(file);
        }

        // Helper closures to map data coordinates to screen coordinates within the plot area
        // Define these early as they might be used by axis drawing (e.g., for Centered style)
        let map_x = |data_x: f32| -> f32 {
            if (actual_x_max - actual_x_min).abs() < f32::EPSILON {
                return plot_area_x_start + plot_area_width / 2.0;
            }
            plot_area_x_start + ((data_x - actual_x_min) / (actual_x_max - actual_x_min) * plot_area_width)
        };
        let map_y = |data_y: f32| -> f32 {
            if (actual_y_max - actual_y_min).abs() < f32::EPSILON {
                return plot_area_y_start + plot_area_height / 2.0;
            }
            plot_area_y_start + plot_area_height - ((data_y - actual_y_min) / (actual_y_max - actual_y_min) * plot_area_height)
        };
        
        // Draw Title
        let title_text_x = plot_area_x_start + (plot_area_width - (self.title.len() as f32 * self.title_config.font_size * 0.45)) / 2.0; // Use title_config
        let title_text_y = current_effective_margin_top * 0.5 + self.title_config.font_size / 2.0; // Use title_config
        dt.draw_text(
            &font,
            self.title_config.font_size, // Use title_config
            &self.title,
            Point::new(title_text_x.max(0.0), title_text_y.max(self.title_config.font_size)), // Use title_config
            &Source::Solid(self.title_config.color), // Use title_config
            &DrawOptions::new(),
        );

        // Draw X-axis Label
        let x_label_text_x = plot_area_x_start + (plot_area_width - (self.x_label.len() as f32  * self.x_label_config.font_size * 0.45)) / 2.0; // Use x_label_config
        let x_label_text_y = plot_area_y_start + plot_area_height + current_effective_margin_bottom * 0.6 + self.x_label_config.font_size / 2.0; // Use x_label_config
        dt.draw_text(
            &font,
            self.x_label_config.font_size, // Use x_label_config
            &self.x_label,
            Point::new(x_label_text_x.max(0.0), x_label_text_y.max(plot_area_y_start + plot_area_height + self.x_label_config.font_size)), // Use x_label_config
            &Source::Solid(self.x_label_config.color), // Use x_label_config
            &DrawOptions::new(),
        );

        // Draw Y-axis Label
        let y_label_text_x = current_effective_margin_left * 0.3 - (self.y_label.len() as f32 * self.y_label_config.font_size * 0.45) / 2.0; // Use y_label_config
        let y_label_text_y = plot_area_y_start + plot_area_height / 2.0 + self.y_label_config.font_size / 2.0; // Use y_label_config
        dt.draw_text(
            &font,
            self.y_label_config.font_size, // Use y_label_config
            &self.y_label,
            Point::new(y_label_text_x.max(0.0), y_label_text_y.max(plot_area_y_start + self.y_label_config.font_size)), // Use y_label_config
            &Source::Solid(self.y_label_config.color), // Use y_label_config
            &DrawOptions::new(),
        );

        // Draw Axis Lines
        let axis_stroke_style = StrokeStyle { width: self.axis_config.line_width, ..Default::default() }; // Use axis_config
        let mut pb_axis = PathBuilder::new();

        match self.axis {
            Axis::BottomLeft => {
                // X-axis (bottom)
                pb_axis.move_to(plot_area_x_start, plot_area_y_start + plot_area_height);
                pb_axis.line_to(plot_area_x_start + plot_area_width, plot_area_y_start + plot_area_height);
                // Y-axis (left)
                pb_axis.move_to(plot_area_x_start, plot_area_y_start);
                pb_axis.line_to(plot_area_x_start, plot_area_y_start + plot_area_height);
            }
            Axis::Box => {
                pb_axis.rect(plot_area_x_start, plot_area_y_start, plot_area_width, plot_area_height);
            }
        }
        let path_axis = pb_axis.finish();
        if !path_axis.ops.is_empty() { 
            dt.stroke(&path_axis, &Source::Solid(self.axis_config.color), &axis_stroke_style, &DrawOptions::new()); // Use axis_config
        }

        // --- Tick Marks, Grid Lines, and Tick Labels ---
        let num_x_ticks = (plot_area_width / self.tick_config.density_x).max(2.0) as usize; // Use tick_config
        let num_y_ticks = (plot_area_height / self.tick_config.density_y).max(2.0) as usize; // Use tick_config

        // Helper function to calculate nice tick values
        let calculate_ticks = |min_val: f32, max_val: f32, max_ticks: usize| -> Vec<f32> {
            if (max_val - min_val).abs() < f32::EPSILON { return vec![min_val]; }
            let range = max_val - min_val;
            let rough_step = range / (max_ticks.saturating_sub(1) as f32);
            let exponent = rough_step.log10().floor();
            let fraction = rough_step / 10f32.powf(exponent);
            let nice_fraction = if fraction < 1.5 { 1.0 }
            else if fraction < 3.5 { 2.0 }
            else if fraction < 7.5 { 5.0 }
            else { 10.0 };
            let step = nice_fraction * 10f32.powf(exponent);
            let start_tick = (min_val / step).ceil() * step;
            let mut ticks = Vec::new();
            let mut current_tick = start_tick;
            while current_tick <= max_val + step * 0.5 {
                ticks.push(current_tick);
                current_tick += step;
                if ticks.len() > max_ticks * 2 { break; }
            }
            if ticks.is_empty() && min_val == max_val { ticks.push(min_val); }
            else if ticks.len() < 2 && min_val != max_val {
                ticks.clear();
                ticks.push(min_val);
                ticks.push(max_val);
            }
            ticks
        };

        let x_ticks = calculate_ticks(actual_x_min, actual_x_max, num_x_ticks);
        let y_ticks = calculate_ticks(actual_y_min, actual_y_max, num_y_ticks);

        let tick_line_stroke_style = StrokeStyle { width: 1.0, ..Default::default() }; // Assuming tick line width is fixed for now, or add to TickConfig
        
        // X Ticks and Grid Lines
        for &tick_val in &x_ticks {
            let screen_x = map_x(tick_val);
            // Basic check to draw ticks only within the plot area width, adjusted for tick length
            if screen_x >= plot_area_x_start - self.tick_config.length && screen_x <= plot_area_x_start + plot_area_width + self.tick_config.length { // Use tick_config
                if self.grid != Grid::None {
                    let mut pb_grid_x = PathBuilder::new();
                    pb_grid_x.move_to(screen_x, plot_area_y_start);
                    pb_grid_x.line_to(screen_x, plot_area_y_start + plot_area_height);
                    let grid_stroke_style = StrokeStyle {
                        width: self.grid_config.line_width, // Use grid_config
                        dash_array: if self.grid == Grid::Dashed { vec![4.0, 4.0] } else { vec![] },
                        ..Default::default()
                    };
                    let mut temp_clip_pb = PathBuilder::new();
                    temp_clip_pb.rect(plot_area_x_start, plot_area_y_start, plot_area_width, plot_area_height);
                    dt.push_clip(&temp_clip_pb.finish());
                    dt.stroke(&pb_grid_x.finish(), &Source::Solid(self.grid_config.color), &grid_stroke_style, &DrawOptions::new()); // Use grid_config
                    dt.pop_clip();
                }

                if self.tick != Tick::None {
                    let mut pb_tick_x = PathBuilder::new();
                    let tick_label = format!("{:.1}", tick_val);
                    let label_width_approx = tick_label.len() as f32 * self.tick_config.font_size * 0.5; // Use tick_config
                    let tick_direction = if self.tick == Tick::Inward { -1.0 } else { 1.0 }; 

                    match self.axis { 
                        Axis::BottomLeft => {
                            pb_tick_x.move_to(screen_x, plot_area_y_start + plot_area_height);
                            pb_tick_x.line_to(screen_x, plot_area_y_start + plot_area_height + self.tick_config.length * tick_direction); // Use tick_config
                            if screen_x >= plot_area_x_start && screen_x <= plot_area_x_start + plot_area_width {
                                dt.draw_text(&font, self.tick_config.font_size, &tick_label, // Use tick_config
                                    Point::new(screen_x - label_width_approx / 2.0, plot_area_y_start + plot_area_height + self.tick_config.text_padding + self.tick_config.font_size), // Use tick_config
                                    &Source::Solid(self.tick_config.label_color), &DrawOptions::new()); // Use tick_config
                            }
                        }
                        Axis::Box => {
                            pb_tick_x.move_to(screen_x, plot_area_y_start + plot_area_height);
                            pb_tick_x.line_to(screen_x, plot_area_y_start + plot_area_height + self.tick_config.length * tick_direction); // Use tick_config
                            pb_tick_x.move_to(screen_x, plot_area_y_start);
                            pb_tick_x.line_to(screen_x, plot_area_y_start - self.tick_config.length * tick_direction); // Use tick_config
                            if screen_x >= plot_area_x_start && screen_x <= plot_area_x_start + plot_area_width {
                                dt.draw_text(&font, self.tick_config.font_size, &tick_label, // Use tick_config
                                    Point::new(screen_x - label_width_approx / 2.0, plot_area_y_start + plot_area_height + self.tick_config.text_padding + self.tick_config.font_size), // Use tick_config
                                    &Source::Solid(self.tick_config.label_color), &DrawOptions::new()); // Use tick_config
                            }
                        }
                    }
                    let final_tick_path_x = pb_tick_x.finish();
                    if !final_tick_path_x.ops.is_empty() {
                        dt.stroke(&final_tick_path_x, &Source::Solid(self.tick_config.line_color), &tick_line_stroke_style, &DrawOptions::new()); // Use tick_config for color
                    }
                }
            }
        }

        // Y Ticks and Grid Lines
        for &tick_val in &y_ticks {
            let screen_y = map_y(tick_val);
            if screen_y >= plot_area_y_start - self.tick_config.length && screen_y <= plot_area_y_start + plot_area_height + self.tick_config.length { // Use tick_config
                if self.grid != Grid::None {
                    let mut pb_grid_y = PathBuilder::new();
                    pb_grid_y.move_to(plot_area_x_start, screen_y);
                    pb_grid_y.line_to(plot_area_x_start + plot_area_width, screen_y);
                    let grid_stroke_style = StrokeStyle {
                        width: self.grid_config.line_width, // Use grid_config
                        dash_array: if self.grid == Grid::Dashed { vec![4.0, 4.0] } else { vec![] },
                        ..Default::default()
                    };
                    let mut temp_clip_pb = PathBuilder::new();
                    temp_clip_pb.rect(plot_area_x_start, plot_area_y_start, plot_area_width, plot_area_height);
                    dt.push_clip(&temp_clip_pb.finish());
                    dt.stroke(&pb_grid_y.finish(), &Source::Solid(self.grid_config.color), &grid_stroke_style, &DrawOptions::new()); // Use grid_config
                    dt.pop_clip();
                }
                if self.tick != Tick::None {
                    let mut pb_tick_y = PathBuilder::new();
                    let tick_label = format!("{:.1}", tick_val);
                    let label_width_approx = tick_label.len() as f32 * self.tick_config.font_size * 0.5; // Use tick_config
                    let tick_direction = if self.tick == Tick::Inward { -1.0 } else { 1.0 }; 

                    match self.axis { 
                        Axis::BottomLeft => {
                            // Tick line from axis outwards/inwards
                            pb_tick_y.move_to(plot_area_x_start, screen_y);
                            pb_tick_y.line_to(plot_area_x_start - self.tick_config.length * tick_direction, screen_y); // Use tick_config
                            if screen_y >= plot_area_y_start && screen_y <= plot_area_y_start + plot_area_height {
                                dt.draw_text(&font, self.tick_config.font_size, &tick_label, // Use tick_config
                                    Point::new(plot_area_x_start - self.tick_config.text_padding - label_width_approx, screen_y + self.tick_config.font_size / 3.0), // Use tick_config
                                    &Source::Solid(self.tick_config.label_color), &DrawOptions::new()); // Use tick_config
                            }
                        }
                        Axis::Box => {
                            // Left Ticks
                            pb_tick_y.move_to(plot_area_x_start, screen_y);
                            pb_tick_y.line_to(plot_area_x_start - self.tick_config.length * tick_direction, screen_y); // Use tick_config
                            // Right Ticks
                            pb_tick_y.move_to(plot_area_x_start + plot_area_width, screen_y);
                            pb_tick_y.line_to(plot_area_x_start + plot_area_width + self.tick_config.length * tick_direction, screen_y); // Use tick_config
                            if screen_y >= plot_area_y_start && screen_y <= plot_area_y_start + plot_area_height {
                                dt.draw_text(&font, self.tick_config.font_size, &tick_label, // Use tick_config
                                    Point::new(plot_area_x_start - self.tick_config.text_padding - label_width_approx, screen_y + self.tick_config.font_size / 3.0), // Use tick_config
                                    &Source::Solid(self.tick_config.label_color), &DrawOptions::new()); // Use tick_config
                                // Optionally draw labels on the right for Box axis if needed
                            }
                        }
                    }
                    let final_tick_path_y = pb_tick_y.finish();
                    if !final_tick_path_y.ops.is_empty() {
                        dt.stroke(&final_tick_path_y, &Source::Solid(self.tick_config.line_color), &tick_line_stroke_style, &DrawOptions::new()); // Use tick_config for color
                    }
                }
            }
        }

        // --- Clipping Path for Plot Area ---
        let mut pb_clip = PathBuilder::new();
        pb_clip.rect(plot_area_x_start, plot_area_y_start, plot_area_width, plot_area_height);
        let plot_area_clip_path = pb_clip.finish();
        dt.push_clip(&plot_area_clip_path); // Apply clipping FOR DATA SERIES

        // --- Data Series Drawing (now clipped) ---
        for series in &self.data {
            let series_rgb_tuple = match color(&series.color) {
                Some(c) => c.rgb(),
                None => (0, 0, 0), 
            };
            let series_color_source = SolidSource { r: series_rgb_tuple.0, g: series_rgb_tuple.1, b: series_rgb_tuple.2, a: 0xff };

            // Draw line for the series
            if series.line != Line::None && series.data.len() > 1 {
                let mut pb_series_line = PathBuilder::new();
                let first_point = series.data[0];
                pb_series_line.move_to(map_x(first_point.0), map_y(first_point.1));
                for point in series.data.iter().skip(1) { pb_series_line.line_to(map_x(point.0), map_y(point.1)); }
                let path_series_line = pb_series_line.finish();
                let line_stroke_style = StrokeStyle {
                    width: 1.5, 
                    dash_array: match series.line {
                        Line::Dashed => vec![6.0, 3.0],
                        Line::Dotted => vec![2.0, 2.0],
                        _ => vec![], 
                    },
                    ..Default::default()
                };
                dt.stroke(&path_series_line, &Source::Solid(series_color_source), &line_stroke_style, &DrawOptions::new());
            }

            // Draw points for the series
            if series.marker != Marker::None {
                let point_size = 5.0; 
                for &(data_x, data_y) in &series.data {
                    let screen_x = map_x(data_x);
                    let screen_y = map_y(data_y);
                    let mut pb_point = PathBuilder::new();
                    match series.marker {
                        Marker::Circle => {
                            pb_point.arc(screen_x, screen_y, point_size / 2.0, 0.0, 2.0 * std::f32::consts::PI);
                            dt.fill(&pb_point.finish(), &Source::Solid(series_color_source), &DrawOptions::new());
                        }
                        Marker::Square => {
                            pb_point.rect(screen_x - point_size / 2.0, screen_y - point_size / 2.0, point_size, point_size);
                            dt.fill(&pb_point.finish(), &Source::Solid(series_color_source), &DrawOptions::new());
                        }
                        Marker::Cross => {
                            let half_size = point_size / 2.0;
                            pb_point.move_to(screen_x - half_size, screen_y - half_size);
                            pb_point.line_to(screen_x + half_size, screen_y + half_size);
                            pb_point.move_to(screen_x - half_size, screen_y + half_size);
                            pb_point.line_to(screen_x + half_size, screen_y - half_size);
                            let point_stroke_style = StrokeStyle { width: 1.0, ..Default::default() }; 
                            dt.stroke(&pb_point.finish(), &Source::Solid(series_color_source), &point_stroke_style, &DrawOptions::new());
                        }
                        Marker::None => {}
                    }
                }
            }
        }
        
        dt.pop_clip(); // Remove clipping

        // --- Legend Drawing ---
        if self.legend != Legend::None && !self.data.is_empty() {
            let legend_x;
            let legend_y;

            match self.legend {
                Legend::TopRightInside => {
                    legend_x = plot_area_x_start + plot_area_width - legend_actual_box_width - self.legend_config.padding;
                    legend_y = plot_area_y_start + self.legend_config.padding;
                }
                Legend::TopRightOutside => {
                    legend_x = total_width as f32 - current_effective_margin_right + self.legend_config.padding;
                    legend_y = plot_area_y_start + self.legend_config.padding;
                }
                Legend::BottomRightInside => {
                    legend_x = plot_area_x_start + plot_area_width - legend_actual_box_width - self.legend_config.padding;
                    legend_y = plot_area_y_start + plot_area_height - legend_height - self.legend_config.padding;
                }
                Legend::BottomRightOutside => {
                    legend_x = total_width as f32 - current_effective_margin_right + self.legend_config.padding;
                    legend_y = plot_area_y_start + plot_area_height - legend_height - self.legend_config.padding;
                }
                Legend::TopLeftInside => {
                    legend_x = plot_area_x_start + self.legend_config.padding;
                    legend_y = plot_area_y_start + self.legend_config.padding;
                }
                Legend::TopLeftOutside => {
                    legend_x = self.margin.left - legend_actual_box_width - self.legend_config.padding; 
                    legend_y = plot_area_y_start + self.legend_config.padding;
                }
                Legend::BottomLeftInside => {
                    legend_x = plot_area_x_start + self.legend_config.padding;
                    legend_y = plot_area_y_start + plot_area_height - legend_height - self.legend_config.padding;
                }
                Legend::BottomLeftOutside => {
                    legend_x = self.margin.left - legend_actual_box_width - self.legend_config.padding;
                    legend_y = plot_area_y_start + plot_area_height - legend_height - self.legend_config.padding;
                }
                Legend::RightCenterInside => {
                    legend_x = plot_area_x_start + plot_area_width - legend_actual_box_width - self.legend_config.padding;
                    legend_y = plot_area_y_start + (plot_area_height - legend_height) / 2.0;
                }
                Legend::RightCenterOutside => {
                    legend_x = total_width as f32 - current_effective_margin_right + self.legend_config.padding;
                    legend_y = plot_area_y_start + (plot_area_height - legend_height) / 2.0;
                }
                Legend::LeftCenterInside => {
                    legend_x = plot_area_x_start + self.legend_config.padding;
                    legend_y = plot_area_y_start + (plot_area_height - legend_height) / 2.0;
                }
                Legend::LeftCenterOutside => {
                    legend_x = self.margin.left - legend_actual_box_width - self.legend_config.padding;
                    legend_y = plot_area_y_start + (plot_area_height - legend_height) / 2.0;
                }
                Legend::TopCenter => { 
                    legend_x = plot_area_x_start + (plot_area_width - legend_actual_box_width) / 2.0;
                    legend_y = plot_area_y_start + self.legend_config.padding;
                }
                Legend::BottomCenter => { 
                    legend_x = plot_area_x_start + (plot_area_width - legend_actual_box_width) / 2.0;
                    legend_y = plot_area_y_start + plot_area_height - legend_height - self.legend_config.padding;
                }
                Legend::None => { legend_x = 0.0; legend_y = 0.0; }
            }

            // Draw legend box
            let mut pb_legend_box = PathBuilder::new();
            pb_legend_box.rect(legend_x, legend_y, legend_actual_box_width, legend_height);
            let legend_box_path = pb_legend_box.finish();
            dt.stroke(&legend_box_path, &Source::Solid(self.legend_config.border_color), &StrokeStyle { width: 1.0, ..Default::default() }, &DrawOptions::new()); // Use legend_config

            // Draw legend items
            for (i, series) in self.data.iter().enumerate() {
                let series_rgb_tuple = match color(&series.color) { Some(c) => c.rgb(), None => (0,0,0) };
                let series_color_source = SolidSource { r: series_rgb_tuple.0, g: series_rgb_tuple.1, b: series_rgb_tuple.2, a: 0xff };

                let item_y = legend_y + self.legend_config.padding + i as f32 * self.legend_config.item_height; // Use legend_config
                let swatch_x = legend_x + self.legend_config.padding; // Use legend_config
                let text_x = swatch_x + self.legend_config.color_swatch_width + self.legend_config.text_offset; // Use legend_config

                // Draw color swatch
                let mut pb_swatch = PathBuilder::new();
                pb_swatch.rect(swatch_x, item_y, self.legend_config.color_swatch_width, self.legend_config.item_height * 0.8); // Use legend_config
                dt.fill(&pb_swatch.finish(), &Source::Solid(series_color_source), &DrawOptions::new());

                // Draw series name
                dt.draw_text(
                    &font,
                    self.legend_config.font_size, // Use legend_config
                    &series.name,
                    Point::new(text_x, item_y + self.legend_config.font_size * 0.8), // Use legend_config
                    &Source::Solid(self.legend_config.text_color), // Use legend_config
                    &DrawOptions::new(),
                );
            }
        }

        // Save to file
        dt.write_png(file)
    }
}