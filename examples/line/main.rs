use quill::{
    LinePlot,
    Quill
};

fn main() {
    // Data points for the line plot.
    let func = |x: f64| x.sin() * 2.0;
    let data_points = (0..100)
        .map(|x| (x as f64 / 10.0, func(x as f64 / 10.0)))
        .collect::<Vec<_>>();

    // Create a line plot with the data points.
    let content = LinePlot::new(data_points)
        .with_template("./templates/line.typ.tera", "line.typ.tera") // Using forward slashes
        .render()
        .unwrap();

    // Write the string to stdout.
    println!("Rendered Typst string:\n{}", content);

    // Create world with content.
    let document = Quill::new(content);

    // Filename to write to.
    let filename = "output";
    let folder = "./target/examples";
    let pdf_filename = format!("{}/{}.pdf", folder, filename);
    let svg_filename = format!("{}/{}.svg", folder, filename);

    // Write to file.
    document.pdf(&pdf_filename);
    document.svg(&svg_filename);
}