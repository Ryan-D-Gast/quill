use quill::*;

fn exponential_data() -> Vec<(f64, f64)> {
    // Exponential growth data - perfect for log scale demonstration
    (0..=20)
        .map(|x| {
            let xf = x as f64;
            (xf, 10.0_f64.powf(xf * 0.1)) // 10^(x/10)
        })
        .collect()
}

fn power_law_data() -> Vec<(f64, f64)> {
    // Power law data - another good candidate for log scale
    (1..=20)
        .map(|x| {
            let xf = x as f64;
            (xf, xf.powi(3)) // x^3
        })
        .collect()
}

fn main() {
    // Example 1: Exponential data with log Y-axis
    let log_plot = Plot::builder()
        .dimensions((800, 600))
        .title("Logarithmic Y-Scale Example")
        .x_label("Time")
        .y_label("Value (Log Scale)")
        .legend(Legend::TopLeftInside)
        .grid(Grid::Solid)
        .tick_config(TickConfig {
            y_scale_type: Scale::Log, // Enable logarithmic scaling on Y-axis
            ..Default::default()
        })
        .data([
            Series::builder()
                .name("10^(x/10)")
                .color("Red")
                .data(exponential_data())
                .marker(Marker::Circle)
                .marker_size(4.0)
                .line(Line::Solid)
                .line_width(2.0)
                .interpolation(Interpolation::Linear)
                .build(),
            Series::builder()
                .name("x³")
                .color("Blue")
                .data(power_law_data())
                .marker(Marker::Square)
                .marker_size(4.0)
                .line(Line::Dashed)
                .line_width(2.0)
                .interpolation(Interpolation::Linear)
                .build(),
        ])
        .build();

    match log_plot.to_svg("./gallery/logarithmic.svg") {
        Ok(_) => println!("Logarithmic plot saved as SVG."),
        Err(e) => eprintln!("Failed to save SVG: {}", e),
    }
}
