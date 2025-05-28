use quill::{Plot, Series};

fn main() {
    // Create a new plot with a builder
    let plot = Plot::builder()
        .dimensions((800, 600))
        .title("My First Plot".to_string())
        .x_label("X Axis".to_string())
        .y_label("Y Axis".to_string())
        .x_min(0.0)
        .x_max(10.0)
        .y_min(0.0)
        .y_max(10.0)
        .legend(true)
        .grid(true)
        .data(vec![
            Series::builder()
                .name("Series 1".to_string())
                .color("Red".to_string())
                .data(vec![(1.0, 1.0), (2.0, 3.0), (3.0, 2.0), (4.0, 5.0), (5.0, 4.0), (9.0, -1.0)])
                .build(),
            Series::builder()
                .name("Series 2".to_string())
                .color("Blue".to_string())
                .data(vec![(1.0, 2.5), (2.0, 1.5), (3.0, 4.5), (4.0, 3.5), (5.0, 6.0)])
                .build(),
        ])
        .build();

    // Plot the data to a PNG file
    match plot.plot("./target/examples/line_graph_example.png") {
        Ok(_) => println!("Plot created successfully at ./target/examples/line_graph_example.png"),
        Err(e) => eprintln!("Error creating plot: {:?}", e),
    }
}
