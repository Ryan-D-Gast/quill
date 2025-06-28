use bon::Builder;
use crate::{
    style::*,
    elements::*,
    series::Series,
    PlotValue,
    draw::{
        draw_title, draw_x_label, draw_y_label, draw_axis_lines, draw_ticks_and_grids,
        draw_data_series, draw_legend,
    }
};
use svg::{
    Document,
    node::element::{
        Rectangle, Definitions, ClipPath,
    },
};

#[derive(Builder)]
pub struct Plot<T: PlotValue = f32> {
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
    pub x_range: Range<T>,
    #[builder(default = Range::Auto)]
    pub y_range: Range<T>,
    #[builder(default = Legend::None)]
    pub legend: Legend,
    #[builder(default = Axis::Box)]
    pub axis: Axis,
    #[builder(default = Tick::Inward)]
    pub tick: Tick,
    #[builder(default = Grid::Solid)]
    pub grid: Grid,
    #[builder(default = "Times New Roman".to_string())]
    pub font: String,

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
    pub data: Vec<Series<T>>,
}

impl<T: PlotValue> Plot<T> {
    /// Creates a new plot with the specified dimensions.
    pub fn to_svg(&self, filename: &str) -> Result<(), std::io::Error> {
        let document = self.plot()?;
        svg::save(filename, &document)?;
        Ok(())
    }

    /// Converts the plot to an SVG document.
    pub fn to_document(&self) -> Result<Document, std::io::Error> {
        self.plot()
    }

    /// Generates an SVG document representing the plot.
    fn plot(&self) -> Result<Document, std::io::Error> {
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
                    (T::from_f32(0.0), T::from_f32(1.0))
                } else {
                    let mut min_x = T::max_value();
                    let mut max_x = T::min_value();
                    for series in &self.data {
                        for (x, _) in &series.data {
                            if *x < min_x { min_x = *x; }
                            if *x > max_x { max_x = *x; }
                        }
                    }
                    if (max_x - min_x) < T::epsilon() {
                        (min_x - T::from_f32(0.5), max_x + T::from_f32(0.5))
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
                    (T::from_f32(0.0), T::from_f32(1.0))
                } else {
                    let mut min_y = T::max_value();
                    let mut max_y = T::min_value();
                    for series in &self.data {
                        for (_, y) in &series.data {
                            if *y < min_y { min_y = *y; }
                            if *y > max_y { max_y = *y; }
                        }
                    }
                    if (max_y - min_y) < T::epsilon() {
                        (min_y - T::from_f32(0.5), max_y + T::from_f32(0.5))
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
            self.legend_config.color_swatch_width + self.legend_config.text_offset + calculated_max_series_name_width
        } else {
            0.0
        };
        let legend_height = if self.legend != Legend::None && !self.data.is_empty() {
            self.data.len() as f32 * self.legend_config.item_height + self.legend_config.padding * 2.0
        } else {
            0.0
        };

        // Adjust margins based on legend position
        let current_effective_margin_left = self.margin.left;
        let mut current_effective_margin_right = self.margin.right;
        let current_effective_margin_top = self.margin.top;
        let current_effective_margin_bottom = self.margin.bottom;

        if self.legend != Legend::None && !self.data.is_empty() {
            match self.legend {
                Legend::TopRightOutside | Legend::RightCenterOutside | Legend::BottomRightOutside => {
                    current_effective_margin_right += legend_actual_box_width + self.legend_config.padding;
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
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("Plot area is too small (width: {}, height: {}). Check dimensions and margins.", plot_area_width, plot_area_height)));
        }

        // Helper closures to map data coordinates to screen coordinates
        let map_x = |data_x: T| -> f32 {
            if (actual_x_max - actual_x_min) < T::epsilon() {
                plot_area_x_start + plot_area_width / 2.0
            } else {
                let data_x_f32 = data_x.to_f32();
                let actual_x_min_f32 = actual_x_min.to_f32();
                let actual_x_max_f32 = actual_x_max.to_f32();
                plot_area_x_start + ((data_x_f32 - actual_x_min_f32) / (actual_x_max_f32 - actual_x_min_f32) * plot_area_width)
            }
        };
        let map_y = |data_y: T| -> f32 {
            if (actual_y_max - actual_y_min) < T::epsilon() {
                plot_area_y_start + plot_area_height / 2.0
            } else {
                let data_y_f32 = data_y.to_f32();
                let actual_y_min_f32 = actual_y_min.to_f32();
                let actual_y_max_f32 = actual_y_max.to_f32();
                plot_area_y_start + plot_area_height - ((data_y_f32 - actual_y_min_f32) / (actual_y_max_f32 - actual_y_min_f32) * plot_area_height)
            }
        };

        // --- Draw Title ---
        document = draw_title(document, &self.title, &self.font, &self.title_config, plot_area_x_start, plot_area_width, current_effective_margin_top);

        // --- Draw X-axis Label ---
        document = draw_x_label(document, &self.x_label, &self.font, &self.x_label_config, plot_area_x_start, plot_area_width, plot_area_y_start, plot_area_height, current_effective_margin_bottom);

        // --- Draw Y-axis Label ---
        document = draw_y_label(document, &self.y_label, &self.font, &self.y_label_config, current_effective_margin_left, plot_area_y_start, plot_area_height);

        // --- Draw Axis Lines ---
        document = draw_axis_lines(document, self.axis, &self.axis_config, plot_area_x_start, plot_area_y_start, plot_area_width, plot_area_height);

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

            let start_tick = (min_val / step).floor() * step;
            let mut ticks = Vec::new();
            let mut current_tick = start_tick;
            
            while current_tick <= max_val + step * 0.5 {
                if current_tick >= min_val - step * 0.1 && current_tick <= max_val + step * 0.1 {
                    ticks.push(current_tick);
                }
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

        let x_ticks = calculate_ticks(actual_x_min.to_f32(), actual_x_max.to_f32(), num_x_ticks);
        let y_ticks = calculate_ticks(actual_y_min.to_f32(), actual_y_max.to_f32(), num_y_ticks);
        
        document = draw_ticks_and_grids(
            document,
            self.axis,
            self.tick,
            self.grid,
            &self.tick_config,
            &self.grid_config,
            &self.font,
            plot_area_x_start,
            plot_area_y_start,
            plot_area_width,
            plot_area_height,
            &x_ticks,
            &y_ticks,
            |x_f32| map_x(T::from_f32(x_f32)),
            |y_f32| map_y(T::from_f32(y_f32)),
        );

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

        // --- Data Series Drawing ---
        let data_group = draw_data_series(&self.data, pigment::color, &map_x, &map_y);
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
                Legend::BottomLeftInside => {
                    legend_x_base = plot_area_x_start + self.legend_config.padding;
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

            document = draw_legend(
                document,
                &self.data,
                &self.font,
                &self.legend_config,
                legend_x_base,
                legend_y_base,
                pigment::color,
                legend_actual_box_width,
                legend_height,
            );
        }
        Ok(document)
    }
}