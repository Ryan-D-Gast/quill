use quill::*;

fn main() {
    // Create a new plot with a builder
    let plot = Plot::builder()
        .dimensions((900, 500))
        .title("Monthly Sales Data - 2024".to_string())
        .x_label("Month".to_string())
        .y_label("Units Sold".to_string())
        // Using 1-12 for months, and then mapping to names for ticks would be ideal,
        // but for simplicity with current library features, we'll use numbers.
        .x_range(Range::Manual { min: 1.0, max: 12.0 })
        .y_range(Range::Manual { min: 0.0, max: 300.0 })
        .legend(Legend::TopLeftInside)
        .grid(Grid::Solid)
        .font("Verdana".to_string())
        .data(vec![
            Series::builder()
                .name("Product A".to_string())
                .color("Blue".to_string())
                .data(vec![
                    (1.0, 150.0), (2.0, 160.0), (3.0, 170.0), (4.0, 155.0),
                    (5.0, 180.0), (6.0, 190.0), (7.0, 200.0), (8.0, 185.0),
                    (9.0, 210.0), (10.0, 220.0), (11.0, 240.0), (12.0, 250.0),
                ])
                .marker(Marker::Circle)
                .line(Line::Solid)
                .build(),
            Series::builder()
                .name("Product B".to_string())
                .color("Firebrick".to_string())
                .data(vec![
                    (1.0, 80.0), (2.0, 85.0), (3.0, 90.0), (4.0, 100.0),
                    (5.0, 95.0), (6.0, 110.0), (7.0, 105.0), (8.0, 120.0),
                    (9.0, 130.0), (10.0, 115.0), (11.0, 140.0), (12.0, 150.0),
                ])
                .marker(Marker::Square)
                .line(Line::Dotted)
                .build(),
            Series::builder()
                .name("Product C (New)".to_string())
                .color("Dark Sea Green".to_string())
                // Product C launched in April (month 4)
                .data(vec![
                    (4.0, 30.0), (5.0, 45.0), (6.0, 60.0), (7.0, 70.0),
                    (8.0, 85.0), (9.0, 100.0), (10.0, 110.0), (11.0, 125.0), (12.0, 140.0),
                ])
                .marker(Marker::Cross)
                .line(Line::Dashed)
                .build(),
        ])
        .build();

    // Plot the data to a PNG file in the gallery
    match plot.plot("./gallery/monthly_sales.svg") {
        Ok(_) => println!("Plot created successfully at ./gallery/monthly_sales.svg"),
        Err(e) => eprintln!("Error creating plot: {:?}", e),
    }
}
