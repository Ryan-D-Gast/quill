//! Quill: A plotting library for Rust.

#![allow(dead_code)]

use bon::Builder;
// Added Path for clipping
use raqote::{DrawTarget, PathBuilder, Source, SolidSource, DrawOptions, StrokeStyle, Point};
use font_kit::{ // Ensure font-kit is used for text
    family_name::FamilyName,
    properties::Properties,
    source::SystemSource
};
use pigment::color;

#[derive(Builder)]
pub struct Plot {
    dimensions: (i32, i32),
    title: String,
    x_label: String,
    y_label: String,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    data: Vec<Series>,
    legend: bool,
    grid: bool,
}

#[derive(Builder)]
pub struct Series {
    name: String,
    color: String,
    data: Vec<(f64, f64)>,
}

// Constants for styling
const MARGIN_TOP: f32 = 60.0;
const MARGIN_BOTTOM: f32 = 70.0; // Increased for x-label and potential ticks
const MARGIN_LEFT: f32 = 80.0;  // Increased for y-label and potential ticks
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

// Helper function to calculate nice tick values
fn calculate_ticks(min_val: f64, max_val: f64, max_ticks: usize) -> Vec<f64> {
    if (max_val - min_val).abs() < f64::EPSILON {
        return vec![min_val];
    }
    let range = max_val - min_val;
    let rough_step = range / (max_ticks.saturating_sub(1)) as f64;

    // Calculate a "nice" step (e.g., 1, 2, 5, 10, ...)
    let exponent = rough_step.log10().floor();
    let fraction = rough_step / 10f64.powf(exponent);

    let nice_fraction = if fraction < 1.5 { 1.0 }
    else if fraction < 3.5 { 2.0 } // or 2.5
    else if fraction < 7.5 { 5.0 }
    else { 10.0 };

    let step = nice_fraction * 10f64.powf(exponent);

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
}

impl Plot {
    /// Creates a png plot with the given dimensions, title, x and y labels, and data points.
    pub fn plot(&self, file: &str) -> Result<(), png::EncodingError> {
        let (total_width, total_height) = self.dimensions;
        let mut dt = DrawTarget::new(total_width, total_height);

        // Load font
        let font = SystemSource::new()
            .select_best_match(&[FamilyName::SansSerif], &Properties::new())
            .expect("Failed to find a sans-serif font")
            .load()
            .expect("Failed to load font");

        // Background
        let mut pb_bg = PathBuilder::new();
        pb_bg.rect(0., 0., total_width as f32, total_height as f32);
        let path_bg = pb_bg.finish();
        dt.fill(&path_bg, &Source::Solid(SolidSource { r: 0xff, g: 0xff, b: 0xff, a: 0xff }), &DrawOptions::new());

        // Calculate legend dimensions first if legend is enabled, as it affects margins
        let mut calculated_max_series_name_width = 0.0f32;
        if self.legend && !self.data.is_empty() {
            calculated_max_series_name_width = self.data.iter()
                .map(|s| {
                    // Crude approximation of text width. Using font metrics would be more accurate.
                    s.name.len() as f32 * LEGEND_FONT_SIZE * 0.6 // 0.6 is an empirical factor
                })
                .fold(0.0f32, |a, b| a.max(b));
        }

        let legend_actual_box_width = if self.legend && !self.data.is_empty() {
            LEGEND_COLOR_SWATCH_WIDTH + LEGEND_TEXT_OFFSET + calculated_max_series_name_width + LEGEND_PADDING * 2.0
        } else {
            0.0 // No legend, no additional width needed for it
        };

        // Calculate plot area (origin is top-left of the plot area, not the image)
        let plot_area_x_start = MARGIN_LEFT;
        let plot_area_y_start = MARGIN_TOP;
        let plot_area_height = total_height as f32 - MARGIN_TOP - MARGIN_BOTTOM;

        // Adjust MARGIN_RIGHT if legend is enabled to make space
        let effective_margin_right = if self.legend && !self.data.is_empty() {
            MARGIN_RIGHT + legend_actual_box_width // Add space for the legend box itself plus the original MARGIN_RIGHT
        } else {
            MARGIN_RIGHT
        };
        let plot_area_width = total_width as f32 - MARGIN_LEFT - effective_margin_right;


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
        
        // Draw Title
        // For more accurate centering, one might need to measure text width with the font API
        let title_text_x = plot_area_x_start + (plot_area_width - (self.title.len() as f32 * TITLE_FONT_SIZE * 0.45)) / 2.0; // Approx centering
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
        let x_label_text_x = plot_area_x_start + (plot_area_width - (self.x_label.len() as f32 * LABEL_FONT_SIZE * 0.45)) / 2.0; // Approx centering
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

        // Draw Axis Lines (drawn before grid and ticks, so they are underneath)
        let axis_stroke_style = StrokeStyle { width: 1.5, ..Default::default() };
        let mut pb_axis = PathBuilder::new();
        pb_axis.move_to(plot_area_x_start, plot_area_y_start + plot_area_height);
        pb_axis.line_to(plot_area_x_start + plot_area_width, plot_area_y_start + plot_area_height);
        pb_axis.move_to(plot_area_x_start, plot_area_y_start);
        pb_axis.line_to(plot_area_x_start, plot_area_y_start + plot_area_height);
        let path_axis = pb_axis.finish();
        dt.stroke(&path_axis, &Source::Solid(AXIS_COLOR), &axis_stroke_style, &DrawOptions::new());

        // Helper closures to map data coordinates to screen coordinates within the plot area
        // Ensure these are defined before being used by ticks, grid, or data series drawing.
        let map_x = |data_x: f64| -> f32 {
            if (self.x_max - self.x_min).abs() < f64::EPSILON { // Avoid division by zero if x_max == x_min
                return plot_area_x_start + plot_area_width / 2.0; 
            }
            plot_area_x_start + ((data_x - self.x_min) / (self.x_max - self.x_min) * plot_area_width as f64) as f32
        };
        let map_y = |data_y: f64| -> f32 {
            if (self.y_max - self.y_min).abs() < f64::EPSILON { // Avoid division by zero if y_max == y_min
                return plot_area_y_start + plot_area_height / 2.0;
            }
            // Y is inverted: 0 at top for screen, but higher values usually at top for plots
            plot_area_y_start + plot_area_height - ((data_y - self.y_min) / (self.y_max - self.y_min) * plot_area_height as f64) as f32
        };

        // --- Tick Marks, Grid Lines, and Tick Labels ---
        let num_x_ticks = (plot_area_width / 80.0).max(2.0) as usize; // Aim for ticks every ~80px
        let num_y_ticks = (plot_area_height / 50.0).max(2.0) as usize; // Aim for ticks every ~50px

        let x_ticks = calculate_ticks(self.x_min, self.x_max, num_x_ticks);
        let y_ticks = calculate_ticks(self.y_min, self.y_max, num_y_ticks);

        let tick_stroke_style = StrokeStyle { width: 1.0, ..Default::default() };
        let grid_stroke_style = StrokeStyle { width: 0.5, ..Default::default() };

        // X Ticks and Grid Lines
        for &tick_val in &x_ticks {
            let screen_x = map_x(tick_val);
            if screen_x >= plot_area_x_start && screen_x <= plot_area_x_start + plot_area_width { // Draw only within plot area bounds
                if self.grid {
                    let mut pb_grid_x = PathBuilder::new();
                    pb_grid_x.move_to(screen_x, plot_area_y_start);
                    pb_grid_x.line_to(screen_x, plot_area_y_start + plot_area_height);
                    dt.stroke(&pb_grid_x.finish(), &Source::Solid(GRID_COLOR), &grid_stroke_style, &DrawOptions::new());
                }
                let mut pb_tick_x = PathBuilder::new();
                pb_tick_x.move_to(screen_x, plot_area_y_start + plot_area_height);
                pb_tick_x.line_to(screen_x, plot_area_y_start + plot_area_height + TICK_LENGTH);
                dt.stroke(&pb_tick_x.finish(), &Source::Solid(AXIS_COLOR), &tick_stroke_style, &DrawOptions::new());
                
                let tick_label = format!("{:.1}", tick_val);
                // Approx centering for tick labels, could be improved with text metrics
                let label_width_approx = tick_label.len() as f32 * TICK_FONT_SIZE * 0.5;
                dt.draw_text(
                    &font, TICK_FONT_SIZE, &tick_label, 
                    Point::new(screen_x - label_width_approx / 2.0, plot_area_y_start + plot_area_height + TICK_LENGTH + TICK_TEXT_PADDING + TICK_FONT_SIZE),
                    &Source::Solid(TEXT_COLOR), &DrawOptions::new()
                );
            }
        }

        // Y Ticks and Grid Lines
        for &tick_val in &y_ticks {
            let screen_y = map_y(tick_val);
            if screen_y >= plot_area_y_start && screen_y <= plot_area_y_start + plot_area_height { // Draw only within plot area bounds
                if self.grid {
                    let mut pb_grid_y = PathBuilder::new();
                    pb_grid_y.move_to(plot_area_x_start, screen_y);
                    pb_grid_y.line_to(plot_area_x_start + plot_area_width, screen_y);
                    dt.stroke(&pb_grid_y.finish(), &Source::Solid(GRID_COLOR), &grid_stroke_style, &DrawOptions::new());
                }
                let mut pb_tick_y = PathBuilder::new();
                pb_tick_y.move_to(plot_area_x_start - TICK_LENGTH, screen_y);
                pb_tick_y.line_to(plot_area_x_start, screen_y);
                dt.stroke(&pb_tick_y.finish(), &Source::Solid(AXIS_COLOR), &tick_stroke_style, &DrawOptions::new());

                let tick_label = format!("{:.1}", tick_val);
                let label_width_approx = tick_label.len() as f32 * TICK_FONT_SIZE * 0.5;
                dt.draw_text(
                    &font, TICK_FONT_SIZE, &tick_label, 
                    Point::new(plot_area_x_start - TICK_LENGTH - TICK_TEXT_PADDING - label_width_approx, screen_y + TICK_FONT_SIZE / 3.0),
                    &Source::Solid(TEXT_COLOR), &DrawOptions::new()
                );
            }
        }

        // --- Clipping Path for Plot Area ---
        let mut pb_clip = PathBuilder::new();
        pb_clip.rect(plot_area_x_start, plot_area_y_start, plot_area_width, plot_area_height);
        let plot_area_clip_path = pb_clip.finish();
        dt.push_clip(&plot_area_clip_path); // Apply clipping FOR DATA SERIES

        // --- Data Series Drawing (now clipped) ---
        for series in &self.data {
            if series.data.len() > 1 {
                let mut pb_series = PathBuilder::new();
                let first_point = series.data[0];
                
                pb_series.move_to(map_x(first_point.0), map_y(first_point.1));

                for point in series.data.iter().skip(1) {
                    pb_series.line_to(map_x(point.0), map_y(point.1));
                }
                let path_series = pb_series.finish();
                
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
                let data_stroke_style = StrokeStyle { width: 2.0, ..Default::default() }; // Slightly thicker lines for data
                dt.stroke(&path_series, &Source::Solid(series_color_source), &data_stroke_style, &DrawOptions::new());
            }
        }
        
        dt.pop_clip(); // Remove the clipping path for data series

        // --- Legend Drawing (drawn after popping data clip, outside plot area) ---
        if self.legend && !self.data.is_empty() {
            // calculated_max_series_name_width and legend_actual_box_width are already computed

            let legend_height = self.data.len() as f32 * LEGEND_ITEM_HEIGHT + LEGEND_PADDING * 2.0 
                                - (if self.data.len() > 1 { LEGEND_ITEM_HEIGHT * 0.2 } else { 0.0 }); // User's original height calculation

            // effective_margin_right was used to calculate plot_area_width.
            // legend_x_start positions the legend box within the space defined by effective_margin_right.
            let legend_x_start = total_width as f32 - effective_margin_right + LEGEND_PADDING; 
            let legend_y_start = MARGIN_TOP; // Align with top of plot area or slightly offset

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