use std::path::PathBuf;
use tera::{Context, Tera};

// Helper function to format data points into a Typst-compatible string
fn format_data_points(points: &Vec<(f64, f64)>) -> String {
    if points.is_empty() {
        return "(())".to_string(); 
    }
    let inner = points
        .iter()
        .map(|(x, y)| format!("({:.3},{:.3})", x, y))
        .collect::<Vec<String>>()
        .join(", ");
    format!("({})", inner)
}

#[derive(Debug)]
pub struct LinePlot {
    // Page settings
    pub page_margin: String,

    // Main style
    pub main_style_stroke: String,
    pub main_style_fill_rgb: String,

    // Primary data set
    pub f1_data: Vec<(f64, f64)>,
    pub f1_label: String,
    pub f1_plot_style_stroke: String,

    // Text settings
    pub text_size: String,

    // Axis and legend styles
    pub axis_style_stroke: String,
    pub tick_style_stroke: String,
    pub legend_style_stroke: String,
    pub legend_orientation: String,
    pub legend_item_spacing: String,
    pub legend_scale_percentage: String,

    // Plot settings
    pub plot_size_width: String,
    pub plot_size_height: String,
    pub plot_x_tick_step: String,
    pub plot_y_tick_step: String,
    pub plot_y_min: String,
    pub plot_y_max: String,
    pub plot_legend_position: String,

    // Path to the template file, not serialized
    template_path: PathBuf,
    template_name: String,
}

impl LinePlot {
    pub fn new(f1_data: Vec<(f64, f64)>) -> Self {
        Self {
            page_margin: ".5cm".to_string(),
            main_style_stroke: "black".to_string(),
            main_style_fill_rgb: "(0, 0, 200, 75)".to_string(),
            f1_data,
            f1_label: "f(x)".to_string(),
            f1_plot_style_stroke: "red".to_string(),
            text_size: "10pt".to_string(),
            axis_style_stroke: ".5pt".to_string(),
            tick_style_stroke: ".5pt".to_string(),
            legend_style_stroke: "none".to_string(),
            legend_orientation: "ttb".to_string(),
            legend_item_spacing: ".3".to_string(),
            legend_scale_percentage: "80%".to_string(),
            plot_size_width: "12".to_string(),
            plot_size_height: "8".to_string(),
            plot_x_tick_step: "1".to_string(),
            plot_y_tick_step: "0.5".to_string(),
            plot_y_min: "-2.5".to_string(),
            plot_y_max: "2.5".to_string(),
            plot_legend_position: "default".to_string(),
            template_path: PathBuf::from("templates/line.typ.tera"),
            template_name: "line.typ.tera".to_string(),
        }
    }

    pub fn with_template(mut self, path: &str, name: &str) -> Self {
        self.template_path = PathBuf::from(path);
        self.template_name = name.to_string();
        self
    }
    
    // Renders the plot to a Typst string
    pub fn render(&self) -> Result<String, tera::Error> {
        let mut tera = Tera::default();
        tera.add_template_file(&self.template_path, Some(&self.template_name))?;
        
        let mut context = Context::new();

        context.insert("page_margin", &self.page_margin);
        context.insert("main_style_stroke", &self.main_style_stroke);
        context.insert("main_style_fill_rgb", &self.main_style_fill_rgb);
        context.insert("f1_label", &self.f1_label);
        context.insert("f1_plot_style_stroke", &self.f1_plot_style_stroke);
        context.insert("text_size", &self.text_size);
        context.insert("axis_style_stroke", &self.axis_style_stroke);
        context.insert("tick_style_stroke", &self.tick_style_stroke);
        context.insert("legend_style_stroke", &self.legend_style_stroke);
        context.insert("legend_orientation", &self.legend_orientation);
        context.insert("legend_item_spacing", &self.legend_item_spacing);
        context.insert("legend_scale_percentage", &self.legend_scale_percentage);
        context.insert("plot_size_width", &self.plot_size_width);
        context.insert("plot_size_height", &self.plot_size_height);
        context.insert("plot_x_tick_step", &self.plot_x_tick_step);
        context.insert("plot_y_tick_step", &self.plot_y_tick_step);
        context.insert("plot_y_min", &self.plot_y_min);
        context.insert("plot_y_max", &self.plot_y_max);
        context.insert("plot_legend_position", &self.plot_legend_position);
        
        context.insert("data_points", &format_data_points(&self.f1_data));
        
        tera.render(&self.template_name, &context)
    }
}

// Example usage (you can move this to an example file or test)
#[cfg(test)]
mod tests {
    use super::*;
    use crate::Quill;

    #[test]
    fn test_render_line_plot() {
        let mut plot = LinePlot::new(vec![(0.0,0.0), (1.0,1.0), (2.0,4.0), (3.0,9.0)])
            .with_template("./templates/line.typ.tera", "line.typ.tera");

        // Example of setting fields (they are now non-optional)
        plot.f1_label = "f_1(x)".to_string();
        plot.plot_y_max = "10".to_string();
        plot.f1_plot_style_stroke = "blue".to_string();
        plot.plot_legend_position = "default".to_string();
        plot.page_margin = "1cm".to_string();

        let rendered_string = match plot.render() {
            Ok(rendered_string) => {
                println!("Rendered Typst string:\n{}", rendered_string);
                assert!(!rendered_string.is_empty());
                assert!(rendered_string.contains("#let data = ((0,0), (1,1), (2,4), (3,9))"));
                assert!(rendered_string.contains("label: $f_1(x)$"));
                assert!(rendered_string.contains("y-max: 10")); 
                assert!(rendered_string.contains("stroke: blue")); 
                assert!(rendered_string.contains("legend: \"default\""));
                assert!(rendered_string.contains("margin: 1cm"));
                rendered_string
            }
            Err(e) => {
                panic!("Failed to render plot: {:?}", e);
            }
        };
        let document = Quill::new(rendered_string);
        let pdf_filename = "output.pdf";
        document.pdf(pdf_filename);
    }

    #[test]
    fn test_format_data_points_empty() {
        assert_eq!(format_data_points(&vec![]), "(())");
    }

    #[test]
    fn test_format_data_points_single() {
        assert_eq!(format_data_points(&vec![(1.0, 2.0)]), "((1,2))");
    }

    #[test]
    fn test_format_data_points_multiple() {
        assert_eq!(format_data_points(&vec![(0.0,0.0), (1.56,2.33), (4.0, -5.13)]), "((0,0), (1.56,2.33), (4,-5.13))");
    }
}