use quill::*;

fn scatter_data() -> Vec<(f32, f32)> {
    // Lissajous curve for interesting scatter pattern
    (0..=100)
        .map(|i| {
            let t = i as f32 * 0.1;
            let x = 10.0 * (2.0 * t).sin();
            let y = 10.0 * (3.0 * t + 0.5).cos();
            (x, y)
        })
        .collect()
}

fn main() {
    let scatter_plot = Plot::builder()
        .dimensions((600, 400))
        .title("Scatter Graph Example")
        .x_label("X Axis")
        .y_label("Y Axis")
        .legend(Legend::TopRightOutside)
        .grid(Grid::Dashed)
        .data([Series::builder()
            .name("Lissajous Curve")
            .color("Red")
            .data(scatter_data())
            .marker(Marker::Circle)
            .marker_size(5.0)
            .line(Line::None)
            .build()])
        .build();
    scatter_plot.to_svg("./gallery/scatter.svg").unwrap();
}
