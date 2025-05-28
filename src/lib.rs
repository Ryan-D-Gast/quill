//! Quill: A plotting library for Rust.

// TODO: Turn draft in this one file into multiple files and broken up components.
// Other things to add:
// - Axis tick settings like log scales specialized ticks etc.
// - Better legend styling
// - Move all these constant plot style qualties into the plot builder
// - Support annotations
// - Add caption below the plot

use bon::Builder;
use raqote::{DrawTarget, PathBuilder, Source, SolidSource, DrawOptions, StrokeStyle, Point};
use font_kit::{
    family_name::FamilyName,
    properties::Properties,
    source::SystemSource
};
use pigment::color;
use png::EncodingError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Legend {
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
    RightCenter,
    LeftCenter,
    TopCenter,
    BottomCenter,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Axis {
    BottomLeft,
    TopRight,
    Centered,
    Box,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tick {
    Inward,
    Outward,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Grid {
    Solid,
    Dashed,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Range {
    Auto,
    Manual { min: f32, max: f32 },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineStyle {
    Solid,
    Dashed,
    Dotted,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PointStyle {
    Circle,
    Square,
    Cross,
    None,
}

#[derive(Builder)]
pub struct Series {
    data: Vec<(f32, f32)>,
    #[builder(default = "".to_string())]
    name: String,
    #[builder(default = "Black".to_string())]
    color: String,
    #[builder(default = LineStyle::Solid)]
    line_style: LineStyle,
    #[builder(default = PointStyle::None)]
    point_style: PointStyle,
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
    tick: Tick,
    #[builder(default = Grid::Solid)]
    grid: Grid,
    #[builder(default = "Times New Roman".to_string())]
    font: String,

    // --- Data ---
    data: Vec<Series>,
}

// Constants for styling
const MARGIN_TOP: f32 = 60.0;
const MARGIN_BOTTOM: f32 = 70.0;
const MARGIN_LEFT: f32 = 80.0;
const MARGIN_RIGHT: f32 = 30.0;
const AXIS_COLOR: SolidSource = SolidSource { r: 0x33, g: 0x33, b: 0x33, a: 0xff }; // Dark gray
const TEXT_COLOR: SolidSource = SolidSource { r: 0x00, g: 0x00, b: 0x00, a: 0xff }; // Black
const GRID_COLOR: SolidSource = SolidSource { r: 0xcc, g: 0xcc, b: 0xcc, a: 0xff }; // Light gray
const TITLE_FONT_SIZE: f32 = 20.0;
const LABEL_FONT_SIZE: f32 = 14.0;
const TICK_FONT_SIZE: f32 = 10.0;
const LEGEND_FONT_SIZE: f32 = 12.0;

const TICK_LENGTH: f32 = 5.0;
const TICK_TEXT_PADDING: f32 = 3.0;
const LEGEND_PADDING: f32 = 10.0;
const LEGEND_ITEM_HEIGHT: f32 = 18.0;
const LEGEND_COLOR_SWATCH_WIDTH: f32 = 15.0;
const LEGEND_TEXT_OFFSET: f32 = 5.0;
const LEGEND_BOX_STROKE_COLOR: SolidSource = SolidSource { r: 0xaa, g: 0xaa, b: 0xaa, a: 0xff };

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
                .map(|s| s.name.len() as f32 * LEGEND_FONT_SIZE * 0.6)
                .fold(0.0f32, |a, b| a.max(b));
        }

        let legend_actual_box_width = if self.legend != Legend::None && !self.data.is_empty() {
            LEGEND_COLOR_SWATCH_WIDTH + LEGEND_TEXT_OFFSET + calculated_max_series_name_width + LEGEND_PADDING * 2.0
        } else {
            0.0
        };
        let legend_height = if self.legend != Legend::None && !self.data.is_empty() {
            self.data.len()  as f32 * LEGEND_ITEM_HEIGHT + LEGEND_PADDING * 2.0
        } else {
            0.0
        };

        // Adjust margins based on legend position if legend is active
        let current_effective_margin_left = MARGIN_LEFT;
        let mut current_effective_margin_right = MARGIN_RIGHT;
        let current_effective_margin_top = MARGIN_TOP;
        let current_effective_margin_bottom = MARGIN_BOTTOM;

        if self.legend != Legend::None && !self.data.is_empty() {
            match self.legend {
                Legend::TopRight | Legend::RightCenter | Legend::BottomRight => {
                    current_effective_margin_right += legend_actual_box_width + LEGEND_PADDING;
                }
                Legend::TopLeft | Legend::LeftCenter | Legend::BottomLeft => {
                    // Potentially adjust left margin if legend is on the left and outside plot area
                    // For now, assuming it fits or overlays.
                }
                // For TopCenter, BottomCenter, None, no specific margin adjustment here.
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
                12.0, 
                "Error: Plot area too small", 
                Point::new(10.0, 20.0), 
                &Source::Solid(TEXT_COLOR), 
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
        // For more accurate centering, one might need to measure text width with the font API
        let title_text_x = plot_area_x_start + (plot_area_width - (self.title.len() as f32 * TITLE_FONT_SIZE * 0.45)) / 2.0;
        let title_text_y = MARGIN_TOP * 0.5 + TITLE_FONT_SIZE / 2.0; // Center in top margin
        dt.draw_text(
            &font,
            TITLE_FONT_SIZE,
            &self.title,
            Point::new(title_text_x.max(0.0), title_text_y.max(TITLE_FONT_SIZE)), // Ensure not off-screen
            &Source::Solid(TEXT_COLOR),
            &DrawOptions::new(),
        );

        // Draw X-axis Label
        let x_label_text_x = plot_area_x_start + (plot_area_width - (self.x_label.len() as f32  * LABEL_FONT_SIZE * 0.45)) / 2.0;
        let x_label_text_y = plot_area_y_start + plot_area_height + MARGIN_BOTTOM * 0.6 + LABEL_FONT_SIZE / 2.0; // Below x-axis
        dt.draw_text(
            &font,
            LABEL_FONT_SIZE,
            &self.x_label,
            Point::new(x_label_text_x.max(0.0), x_label_text_y.max(plot_area_y_start + plot_area_height + LABEL_FONT_SIZE)),
            &Source::Solid(TEXT_COLOR),
            &DrawOptions::new(),
        );

        // Draw Y-axis Label (Horizontal for simplicity, rotation is more complex with raqote)
        let y_label_text_x = MARGIN_LEFT * 0.3 - (self.y_label.len() as f32 * LABEL_FONT_SIZE * 0.45) / 2.0; // Center in left margin
        let y_label_text_y = plot_area_y_start + plot_area_height / 2.0 + LABEL_FONT_SIZE / 2.0; // Vertically centered to plot area
        dt.draw_text(
            &font,
            LABEL_FONT_SIZE,
            &self.y_label,
            Point::new(y_label_text_x.max(0.0), y_label_text_y.max(plot_area_y_start + LABEL_FONT_SIZE)),
            &Source::Solid(TEXT_COLOR),
            &DrawOptions::new(),
        );

        // Draw Axis Lines based on self.axis
        let axis_stroke_style = StrokeStyle { width: 1.5, ..Default::default() };
        let mut pb_axis = PathBuilder::new();

        match self.axis { // Updated field name
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
            Axis::TopRight => {
                // X-axis (top)
                pb_axis.move_to(plot_area_x_start, plot_area_y_start);
                pb_axis.line_to(plot_area_x_start + plot_area_width, plot_area_y_start);
                // Y-axis (right)
                pb_axis.move_to(plot_area_x_start + plot_area_width, plot_area_y_start);
                pb_axis.line_to(plot_area_x_start + plot_area_width, plot_area_y_start + plot_area_height);
            }
            Axis::Centered => {
                let origin_x_on_screen = map_x(0.0).max(plot_area_x_start).min(plot_area_x_start + plot_area_width - 1.0); 
                let origin_y_on_screen = map_y(0.0).max(plot_area_y_start).min(plot_area_y_start + plot_area_height - 1.0); 

                // Centered X-axis
                pb_axis.move_to(plot_area_x_start, origin_y_on_screen);
                pb_axis.line_to(plot_area_x_start + plot_area_width, origin_y_on_screen);
                // Centered Y-axis
                pb_axis.move_to(origin_x_on_screen, plot_area_y_start);
                pb_axis.line_to(origin_x_on_screen, plot_area_y_start + plot_area_height);
            }
        }
        let path_axis = pb_axis.finish();
        if !path_axis.ops.is_empty() { 
            dt.stroke(&path_axis, &Source::Solid(AXIS_COLOR), &axis_stroke_style, &DrawOptions::new());
        }

        // --- Tick Marks, Grid Lines, and Tick Labels ---
        let num_x_ticks = (plot_area_width / 80.0).max(2.0) as usize; // Aim for ticks every ~80px
        let num_y_ticks = (plot_area_height / 50.0).max(2.0) as usize; // Aim for ticks every ~50px

        // Helper function to calculate nice tick values
        let calculate_ticks = |min_val: f32, max_val: f32, max_ticks: usize| -> Vec<f32> {
            if (max_val - min_val).abs() < f32::EPSILON {
                return vec![min_val];
            }
            let range = max_val - min_val;
            let rough_step = range / (max_ticks.saturating_sub(1) as f32);

            // Calculate a "nice" step (e.g., 1, 2, 5, 10, ...)
            let exponent = rough_step.log10().floor();
            let fraction = rough_step / 10f32.powf(exponent);

            let nice_fraction = if fraction < 1.5 { 1.0 }
            else if fraction < 3.5 { 2.0 } // or 2.5
            else if fraction < 7.5 { 5.0 }
            else { 10.0 };

            let step = nice_fraction * 10f32.powf(exponent);

            let start_tick = (min_val / step).ceil() * step;
            let mut ticks = Vec::new();
            let mut current_tick = start_tick;
            while current_tick <= max_val + step * 0.5 { // Add a little buffer for floating point issues
                ticks.push(current_tick);
                current_tick += step;
                if ticks.len() > max_ticks * 2 { break; } // Safety break
            }
            // Ensure at least two ticks if possible, or one if min == max
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

        let tick_stroke_style = StrokeStyle { width: 1.0, ..Default::default() };
        
        // X Ticks and Grid Lines
        for &tick_val in &x_ticks {
            let screen_x = map_x(tick_val);
            // Basic check to draw ticks only within the plot area width, adjusted for tick length
            if screen_x >= plot_area_x_start - TICK_LENGTH && screen_x <= plot_area_x_start + plot_area_width + TICK_LENGTH {
                match self.grid { // Updated field name
                    Grid::Solid | Grid::Dashed => {
                        let mut pb_grid_x = PathBuilder::new();
                        pb_grid_x.move_to(screen_x, plot_area_y_start);
                        pb_grid_x.line_to(screen_x, plot_area_y_start + plot_area_height);
                        
                        let grid_stroke_style = StrokeStyle {
                            width: 0.5,
                            dash_array: if self.grid == Grid::Dashed { vec![4.0, 4.0] } else { vec![] }, // Updated field name
                            dash_offset: 0.0,
                            ..Default::default()
                        };

                        // Clip grid lines to plot area explicitly
                        let mut temp_clip_pb = PathBuilder::new();
                        temp_clip_pb.rect(plot_area_x_start, plot_area_y_start, plot_area_width, plot_area_height);
                        dt.push_clip(&temp_clip_pb.finish());
                        dt.stroke(&pb_grid_x.finish(), &Source::Solid(GRID_COLOR), &grid_stroke_style, &DrawOptions::new());
                        dt.pop_clip();
                    }
                    Grid::None => {}
                }

                if self.tick != Tick::None { // Updated field name
                    let mut pb_tick_x = PathBuilder::new();
                    let tick_label = format!("{:.1}", tick_val);
                    let label_width_approx = tick_label.len() as f32 * TICK_FONT_SIZE * 0.5;
                    let tick_direction = if self.tick == Tick::Inward { -1.0 } else { 1.0 }; // Updated field name

                    match self.axis { // Updated field name
                        Axis::BottomLeft => {
                            pb_tick_x.move_to(screen_x, plot_area_y_start + plot_area_height);
                            pb_tick_x.line_to(screen_x, plot_area_y_start + plot_area_height + TICK_LENGTH * tick_direction);
                            if screen_x >= plot_area_x_start && screen_x <= plot_area_x_start + plot_area_width {
                                dt.draw_text(
                                    &font, TICK_FONT_SIZE, &tick_label,
                                    Point::new(screen_x - label_width_approx / 2.0, plot_area_y_start + plot_area_height + TICK_TEXT_PADDING + TICK_FONT_SIZE),
                                    &Source::Solid(TEXT_COLOR), &DrawOptions::new()
                                );
                            }
                        }
                        Axis::Box => {
                            // Bottom Ticks
                            pb_tick_x.move_to(screen_x, plot_area_y_start + plot_area_height);
                            pb_tick_x.line_to(screen_x, plot_area_y_start + plot_area_height + TICK_LENGTH * tick_direction);
                            // Top Ticks
                            pb_tick_x.move_to(screen_x, plot_area_y_start);
                            pb_tick_x.line_to(screen_x, plot_area_y_start - TICK_LENGTH * tick_direction);
                            if screen_x >= plot_area_x_start && screen_x <= plot_area_x_start + plot_area_width {
                                // Bottom Labels
                                dt.draw_text(
                                    &font, TICK_FONT_SIZE, &tick_label,
                                    Point::new(screen_x - label_width_approx / 2.0, plot_area_y_start + plot_area_height + TICK_TEXT_PADDING + TICK_FONT_SIZE),
                                    &Source::Solid(TEXT_COLOR), &DrawOptions::new()
                                );
                            }
                        }
                        Axis::TopRight => {
                            pb_tick_x.move_to(screen_x, plot_area_y_start);
                            pb_tick_x.line_to(screen_x, plot_area_y_start - TICK_LENGTH * tick_direction);
                            if screen_x >= plot_area_x_start && screen_x <= plot_area_x_start + plot_area_width {
                                dt.draw_text(
                                    &font, TICK_FONT_SIZE, &tick_label,
                                    Point::new(screen_x - label_width_approx / 2.0, plot_area_y_start - TICK_TEXT_PADDING),
                                    &Source::Solid(TEXT_COLOR), &DrawOptions::new()
                                );
                            }
                        }
                        Axis::Centered => {
                            let origin_y_on_screen = map_y(0.0).max(plot_area_y_start).min(plot_area_y_start + plot_area_height - 1.0);
                            pb_tick_x.move_to(screen_x, origin_y_on_screen);
                            pb_tick_x.line_to(screen_x, origin_y_on_screen + TICK_LENGTH * tick_direction);
                            if screen_x >= plot_area_x_start && screen_x <= plot_area_x_start + plot_area_width {
                                // Assuming labels below the centered axis
                                dt.draw_text(
                                    &font, TICK_FONT_SIZE, &tick_label,
                                    Point::new(screen_x - label_width_approx / 2.0, origin_y_on_screen + TICK_TEXT_PADDING + TICK_FONT_SIZE),
                                    &Source::Solid(TEXT_COLOR), &DrawOptions::new()
                                );
                            }
                        }
                    }
                    let final_tick_path_x = pb_tick_x.finish();
                    if !final_tick_path_x.ops.is_empty() {
                        dt.stroke(&final_tick_path_x, &Source::Solid(AXIS_COLOR), &tick_stroke_style, &DrawOptions::new());
                    }
                }
            }
        }

        // Y Ticks and Grid Lines
        for &tick_val in &y_ticks {
            let screen_y = map_y(tick_val);
            if screen_y >= plot_area_y_start - TICK_LENGTH && screen_y <= plot_area_y_start + plot_area_height + TICK_LENGTH {
                match self.grid { // Updated field name
                    Grid::Solid | Grid::Dashed => {
                        let mut pb_grid_y = PathBuilder::new();
                        pb_grid_y.move_to(plot_area_x_start, screen_y);
                        pb_grid_y.line_to(plot_area_x_start + plot_area_width, screen_y);

                        let grid_stroke_style = StrokeStyle {
                            width: 0.5,
                            dash_array: if self.grid == Grid::Dashed { vec![4.0, 4.0] } else { vec![] }, // Updated field name
                            dash_offset: 0.0,
                            ..Default::default()
                        };
                        
                        // Clip grid lines
                        let mut temp_clip_pb = PathBuilder::new();
                        temp_clip_pb.rect(plot_area_x_start, plot_area_y_start, plot_area_width, plot_area_height);
                        dt.push_clip(&temp_clip_pb.finish());
                        dt.stroke(&pb_grid_y.finish(), &Source::Solid(GRID_COLOR), &grid_stroke_style, &DrawOptions::new());
                        dt.pop_clip();
                    }
                    Grid::None => {}
                }
                if self.tick != Tick::None { // Updated field name
                    let mut pb_tick_y = PathBuilder::new();
                    let tick_label = format!("{:.1}", tick_val);
                    let label_width_approx = tick_label.len() as f32 * TICK_FONT_SIZE * 0.5;
                    let tick_direction = if self.tick == Tick::Inward { -1.0 } else { 1.0 }; // Updated field name

                    match self.axis { // Updated field name
                        Axis::BottomLeft => {
                            // Tick line from axis outwards/inwards
                            pb_tick_y.move_to(plot_area_x_start, screen_y);
                            pb_tick_y.line_to(plot_area_x_start - TICK_LENGTH * tick_direction, screen_y);
                            if screen_y >= plot_area_y_start && screen_y <= plot_area_y_start + plot_area_height {
                                dt.draw_text(
                                    &font, TICK_FONT_SIZE, &tick_label,
                                    Point::new(plot_area_x_start - TICK_TEXT_PADDING - label_width_approx, screen_y + TICK_FONT_SIZE / 3.0),
                                    &Source::Solid(TEXT_COLOR), &DrawOptions::new()
                                );
                            }
                        }
                        Axis::Box => {
                            // Left Ticks
                            pb_tick_y.move_to(plot_area_x_start, screen_y);
                            pb_tick_y.line_to(plot_area_x_start - TICK_LENGTH * tick_direction, screen_y);
                            // Right Ticks
                            pb_tick_y.move_to(plot_area_x_start + plot_area_width, screen_y);
                            pb_tick_y.line_to(plot_area_x_start + plot_area_width + TICK_LENGTH * tick_direction, screen_y);
                            if screen_y >= plot_area_y_start && screen_y <= plot_area_y_start + plot_area_height {
                                // Left Labels
                                dt.draw_text(
                                    &font, TICK_FONT_SIZE, &tick_label,
                                    Point::new(plot_area_x_start - TICK_TEXT_PADDING - label_width_approx, screen_y + TICK_FONT_SIZE / 3.0),
                                    &Source::Solid(TEXT_COLOR), &DrawOptions::new()
                                );
                            }
                        }
                        Axis::TopRight => {
                            // Tick line from axis outwards/inwards
                            pb_tick_y.move_to(plot_area_x_start + plot_area_width, screen_y);
                            pb_tick_y.line_to(plot_area_x_start + plot_area_width + TICK_LENGTH * tick_direction, screen_y);
                            if screen_y >= plot_area_y_start && screen_y <= plot_area_y_start + plot_area_height {
                                dt.draw_text(
                                    &font, TICK_FONT_SIZE, &tick_label,
                                    Point::new(plot_area_x_start + plot_area_width + TICK_TEXT_PADDING, screen_y + TICK_FONT_SIZE / 3.0),
                                    &Source::Solid(TEXT_COLOR), &DrawOptions::new()
                                );
                            }
                        }
                         Axis::Centered => {
                            let origin_x_on_screen = map_x(0.0).max(plot_area_x_start).min(plot_area_x_start + plot_area_width - 1.0);
                            // Tick line from axis outwards/inwards
                            pb_tick_y.move_to(origin_x_on_screen, screen_y);
                            pb_tick_y.line_to(origin_x_on_screen + TICK_LENGTH * tick_direction * (if origin_x_on_screen < plot_area_x_start + plot_area_width / 2.0 {1.0} else {-1.0}), screen_y); // Adjust direction based on side of origin
                            // Y-Tick Labels for Centered Axis
                            if screen_y >= plot_area_y_start && screen_y <= plot_area_y_start + plot_area_height && tick_val.abs() > f32::EPSILON { // Avoid label at origin for Y
                                let label_x_pos;
                                // Position labels to the left of the Y-axis if data is mostly positive X, right otherwise
                                // Or, more simply, always to the left of the centered Y axis, or right if it's near the left edge.
                                if origin_x_on_screen < plot_area_x_start + label_width_approx + TICK_TEXT_PADDING * 2.0 { // If axis is too close to left
                                    label_x_pos = origin_x_on_screen + TICK_TEXT_PADDING;
                                } else {
                                    label_x_pos = origin_x_on_screen - TICK_TEXT_PADDING - label_width_approx;
                                }
                                dt.draw_text(
                                    &font, TICK_FONT_SIZE, &tick_label,
                                    Point::new(label_x_pos , screen_y + TICK_FONT_SIZE / 3.0),
                                    &Source::Solid(TEXT_COLOR), &DrawOptions::new()
                                );
                            }
                        }
                    }
                    let final_tick_path_y = pb_tick_y.finish();
                    if !final_tick_path_y.ops.is_empty() {
                        dt.stroke(&final_tick_path_y, &Source::Solid(AXIS_COLOR), &tick_stroke_style, &DrawOptions::new());
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
                None => (0, 0, 0), // Default to black if color name is invalid
            };
            let series_color_source = SolidSource {
                r: series_rgb_tuple.0,
                g: series_rgb_tuple.1,
                b: series_rgb_tuple.2,
                a: 0xff,
            };

            // Draw line for the series
            if series.line_style != LineStyle::None && series.data.len() > 1 {
                let mut pb_series_line = PathBuilder::new();
                let first_point = series.data[0];
                pb_series_line.move_to(map_x(first_point.0), map_y(first_point.1));

                for point in series.data.iter().skip(1) {
                    pb_series_line.line_to(map_x(point.0), map_y(point.1));
                }
                let path_series_line = pb_series_line.finish();
                
                let line_stroke_style = StrokeStyle {
                    width: 1.5, // Slightly thinner than default data stroke for better distinction if points are also drawn
                    dash_array: match series.line_style {
                        LineStyle::Dashed => vec![6.0, 3.0],
                        LineStyle::Dotted => vec![2.0, 2.0],
                        _ => vec![], // Solid or None (None handled by outer if)
                    },
                    dash_offset: 0.0,
                    ..Default::default()
                };
                dt.stroke(&path_series_line, &Source::Solid(series_color_source), &line_stroke_style, &DrawOptions::new());
            }

            // Draw points for the series
            if series.point_style != PointStyle::None {
                let point_size = 5.0; // Diameter for circle/square, size for cross
                for &(data_x, data_y) in &series.data {
                    let screen_x = map_x(data_x);
                    let screen_y = map_y(data_y);
                    let mut pb_point = PathBuilder::new();

                    match series.point_style {
                        PointStyle::Circle => {
                            pb_point.arc(screen_x, screen_y, point_size / 2.0, 0.0, 2.0 * std::f32::consts::PI);
                            dt.fill(&pb_point.finish(), &Source::Solid(series_color_source), &DrawOptions::new());
                        }
                        PointStyle::Square => {
                            pb_point.rect(screen_x - point_size / 2.0, screen_y - point_size / 2.0, point_size, point_size);
                            dt.fill(&pb_point.finish(), &Source::Solid(series_color_source), &DrawOptions::new());
                        }
                        PointStyle::Cross => {
                            let half_size = point_size / 2.0;
                            pb_point.move_to(screen_x - half_size, screen_y - half_size);
                            pb_point.line_to(screen_x + half_size, screen_y + half_size);
                            pb_point.move_to(screen_x - half_size, screen_y + half_size);
                            pb_point.line_to(screen_x + half_size, screen_y - half_size);
                            let point_stroke_style = StrokeStyle { width: 1.0, ..Default::default() }; 
                            dt.stroke(&pb_point.finish(), &Source::Solid(series_color_source), &point_stroke_style, &DrawOptions::new());
                        }
                        PointStyle::None => {}
                    }
                }
            }
        }
        
        dt.pop_clip(); // Remove the clipping path for data series

        // --- Legend Drawing (drawn after popping data clip) ---
        if self.legend != Legend::None && !self.data.is_empty() { // Updated condition
            // legend_actual_box_width and legend_height are already calculated

            let mut legend_x_start;
            let mut legend_y_start;

            // Determine legend_x_start and legend_y_start based on legend_position
            match self.legend { // Updated field name
                Legend::TopRight => {
                    legend_x_start = total_width as f32 - current_effective_margin_right + LEGEND_PADDING; // Positioned within the adjusted margin
                    legend_y_start = current_effective_margin_top;
                }
                Legend::TopLeft => {
                    legend_x_start = current_effective_margin_left;
                    legend_y_start = current_effective_margin_top;
                }
                Legend::BottomRight => {
                    legend_x_start = total_width as f32 - current_effective_margin_right + LEGEND_PADDING;
                    legend_y_start = total_height as f32 - current_effective_margin_bottom - legend_height;
                }
                Legend::BottomLeft => {
                    legend_x_start = current_effective_margin_left;
                    legend_y_start = total_height as f32 - current_effective_margin_bottom - legend_height;
                }
                Legend::RightCenter => {
                    legend_x_start = total_width as f32 - current_effective_margin_right + LEGEND_PADDING;
                    legend_y_start = plot_area_y_start + (plot_area_height - legend_height) / 2.0;
                }
                Legend::LeftCenter => {
                    legend_x_start = current_effective_margin_left; // Needs adjustment if legend is outside plot
                    legend_y_start = plot_area_y_start + (plot_area_height - legend_height) / 2.0;
                }
                Legend::TopCenter => {
                    legend_x_start = plot_area_x_start + (plot_area_width - legend_actual_box_width) / 2.0;
                    legend_y_start = current_effective_margin_top;
                }
                Legend::BottomCenter => {
                    legend_x_start = plot_area_x_start + (plot_area_width - legend_actual_box_width) / 2.0;
                    legend_y_start = total_height as f32 - current_effective_margin_bottom - legend_height;
                }
                Legend::None => { // Should not happen due to outer if, but good for completeness
                    legend_x_start = 0.0;
                    legend_y_start = 0.0;
                }
            }
            // Ensure legend is not drawn off-screen due to extreme calculations
            legend_x_start = legend_x_start.max(0.0);
            legend_y_start = legend_y_start.max(0.0);


            // Draw legend box
            let mut pb_legend_box = PathBuilder::new();
            // Use legend_actual_box_width for drawing the rectangle
            pb_legend_box.rect(legend_x_start, legend_y_start, legend_actual_box_width, legend_height);
            let legend_box_path = pb_legend_box.finish();
            dt.stroke(&legend_box_path, &Source::Solid(LEGEND_BOX_STROKE_COLOR), &StrokeStyle{width: 0.5, ..Default::default()}, &DrawOptions::new());

            let mut current_legend_y = legend_y_start + LEGEND_PADDING;
            for series in &self.data {
                let series_rgb_tuple = match color(&series.color) {
                    Some(c) => c.rgb(),
                    None => (0, 0, 0),
                };
                let series_color_source = SolidSource {
                    r: series_rgb_tuple.0,
                    g: series_rgb_tuple.1,
                    b: series_rgb_tuple.2,
                    a: 0xff,
                };

                // Draw color swatch
                let mut pb_swatch = PathBuilder::new();
                pb_swatch.rect(legend_x_start + LEGEND_PADDING, current_legend_y, LEGEND_COLOR_SWATCH_WIDTH, LEGEND_ITEM_HEIGHT * 0.8);
                dt.fill(&pb_swatch.finish(), &Source::Solid(series_color_source), &DrawOptions::new());

                // Draw series name
                dt.draw_text(
                    &font, 
                    LEGEND_FONT_SIZE, 
                    &series.name, 
                    Point::new(legend_x_start + LEGEND_PADDING + LEGEND_COLOR_SWATCH_WIDTH + LEGEND_TEXT_OFFSET, current_legend_y + LEGEND_ITEM_HEIGHT * 0.8 * 0.5 + LEGEND_FONT_SIZE / 3.0),
                    &Source::Solid(TEXT_COLOR), 
                    &DrawOptions::new()
                );
                current_legend_y += LEGEND_ITEM_HEIGHT;
            }
        }

        // Save to file
        dt.write_png(file)
    }
}