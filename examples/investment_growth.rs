use quill::*;

fn main() {
    // Create a new plot with a builder
    let plot = Plot::builder()
        .dimensions((800, 600))
        .title("Hypothetical Investment Growth")
        .x_label("Years")
        .y_label("Value ($)")
        .x_range(Range::Manual { min: 0.0, max: 10.0 })
        .legend(Legend::TopLeftInside)
        .grid(Grid::Dotted)
        .font("Times New Roman")
        .data(vec![
            Series::builder()
                .name("Low-Risk Investment")
                .color("Green")
                .data(vec![
                    (0.0, 1000.0),
                    (1.0, 1050.0),
                    (2.0, 1102.5),
                    (3.0, 1157.6),
                    (4.0, 1215.5),
                    (5.0, 1276.3),
                    (6.0, 1340.1),
                    (7.0, 1407.1),
                    (8.0, 1477.5),
                    (9.0, 1551.3),
                    (10.0, 1628.9),
                ])
                .marker(Marker::Circle)
                .line(Line::Solid)
                .build(),
            Series::builder()
                .name("Medium-Risk Investment")
                .color("Orange")
                .data(vec![
                    (0.0, 1000.0),
                    (1.0, 1100.0),
                    (2.0, 1210.0),
                    (3.0, 1331.0),
                    (4.0, 1464.1),
                    (5.0, 1610.5),
                    (6.0, 1771.6),
                    (7.0, 1948.7),
                    (8.0, 2143.6),
                    (9.0, 2357.9),
                    (10.0, 2593.7),
                ])
                .marker(Marker::Square)
                .line(Line::Solid)
                .build(),
            Series::builder()
                .name("High-Risk Investment")
                .color("Red")
                .data(vec![
                    (0.0, 1000.0),
                    (1.0, 1200.0),
                    (2.0, 900.0),  // Illustrative dip for high-risk
                    (3.0, 1500.0),
                    (4.0, 2000.0),
                    (5.0, 1800.0), // Another dip
                    (6.0, 2500.0),
                    (7.0, 3500.0),
                    (8.0, 5000.0),
                    (9.0, 4500.0), // Volatility
                    (10.0, 6000.0),
                ])
                .marker(Marker::Cross)
                .line(Line::Dashed)
                .build(),
        ])
        .build();

    // Plot the data to a PNG file in the gallery
    match plot.to_svg("./gallery/investment_growth.svg") {
        Ok(_) => println!("Plot created successfully at ./gallery/investment_growth.svg"),
        Err(e) => eprintln!("Error creating plot: {:?}", e),
    }
}
