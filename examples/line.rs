use quill::*;

fn line_data() -> Vec<(f32, f32)> {
    // Sine wave with a linear trend
    (0..=100).map(|x| {
        let xf = x as f32 * 0.1;
        (xf, xf.sin())
    }).collect()
}

fn main() {
    let line_plot = Plot::builder()
        .dimensions((600, 400))
        .title("Line Graph Example".to_string())
        .x_label("X Axis".to_string())
        .y_label("Y Axis".to_string())
        .legend(Legend::TopRightOutside)
        .grid(Grid::Solid)
        .data(vec![
            Series::builder()
                .name("Sine Curve".to_string())
                .color("Blue".to_string())
                .data(line_data())
                .marker(Marker::None)
                .line(Line::Solid)
                .build(),
        ])
        .build();
    line_plot.plot("./gallery/line.svg").unwrap();
}
