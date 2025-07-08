use quill::*;

fn line_data() -> Vec<(f64, f64)> {
    // Simple sine wave data for line graph
    (0..=100)
        .map(|x| {
            let xf = x as f64 * 0.1;
            (xf, xf.sin())
        })
        .collect()
}

fn main() {
    let line_plot = Plot::builder()
        .dimensions((600, 400))
        .title("Line Graph Example")
        .x_label("X Axis")
        .y_label("Y Axis")
        .legend(Legend::TopRightOutside)
        .grid(Grid::Solid)
        .data([Series::builder()
            .name("Sine Curve")
            .color(Color::Blue)
            .data(line_data())
            .marker(Marker::None)
            .line(Line::Solid)
            .build()])
        .build();
    line_plot.to_svg("./gallery/line.svg").unwrap();
}
