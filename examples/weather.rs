use quill::*;

fn daily_temperature_data() -> Vec<(i32, i32)> {
    // Daily temperatures over a month (day, temperature in Fahrenheit)
    vec![
        (1, 32),
        (2, 28),
        (3, 35),
        (4, 42),
        (5, 38),
        (6, 45),
        (7, 52),
        (8, 48),
        (9, 55),
        (10, 61),
        (11, 58),
        (12, 65),
        (13, 72),
        (14, 68),
        (15, 75),
        (16, 78),
        (17, 82),
        (18, 79),
        (19, 85),
        (20, 88),
        (21, 84),
        (22, 81),
        (23, 77),
        (24, 73),
        (25, 69),
        (26, 66),
        (27, 62),
        (28, 58),
        (29, 54),
        (30, 51),
    ]
}

fn daily_humidity_data() -> Vec<(i32, i32)> {
    // Daily humidity percentages over the same month
    vec![
        (1, 85),
        (2, 88),
        (3, 82),
        (4, 75),
        (5, 78),
        (6, 72),
        (7, 68),
        (8, 71),
        (9, 65),
        (10, 62),
        (11, 66),
        (12, 58),
        (13, 55),
        (14, 59),
        (15, 52),
        (16, 48),
        (17, 45),
        (18, 49),
        (19, 42),
        (20, 38),
        (21, 41),
        (22, 44),
        (23, 47),
        (24, 51),
        (25, 54),
        (26, 57),
        (27, 61),
        (28, 64),
        (29, 68),
        (30, 72),
    ]
}

fn main() {
    let weather_plot = Plot::builder()
        .dimensions((800, 500))
        .title("Daily Weather Data")
        .x_label("Day of Month")
        .y_label("Temperature (°F) and Humidity (%)")
        .x_range(Range::Manual { min: 1, max: 30 })
        .y_range(Range::Manual { min: 25, max: 90 })
        .legend(Legend::TopRightOutside)
        .grid(Grid::Solid)
        .font("Arial")
        .data([
            Series::builder()
                .name("Temperature (°F)")
                .color("Red")
                .data(daily_temperature_data())
                .marker(Marker::Circle)
                .marker_size(4.0)
                .line(Line::Solid)
                .interpolation(Interpolation::Spline)
                .build(),
            Series::builder()
                .name("Humidity (%)")
                .color("Blue")
                .data(daily_humidity_data())
                .marker(Marker::Square)
                .marker_size(4.0)
                .line(Line::Dashed)
                .interpolation(Interpolation::Spline)
                .build(),
        ])
        .build();

    // Render the plot to SVG
    match weather_plot.to_svg("./gallery/weather.svg") {
        Ok(_) => println!("Weather plot created successfully at ./gallery/weather.svg"),
        Err(e) => eprintln!("Error creating plot: {:?}", e),
    }

    // Optionally, render to PNG with feature = ["png"]
    #[cfg(feature = "png")]
    match weather_plot.to_png("./gallery/weather.png", 2.0) {
        Ok(_) => println!("Weather plot created successfully at ./gallery/weather.png"),
        Err(e) => eprintln!("Error creating plot: {:?}", e),
    }
}
