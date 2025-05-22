use quill::Quill;

fn main() {
    let content = r#"
In this report, we will explore the
various factors that influence fluid
dynamics in glaciers and how they
contribute to the formation and
behaviour of these natural structures.
"#
    .to_owned();

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