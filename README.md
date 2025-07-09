# Quill 🪶

A lightweight Rust plotting library for creating simple 2D plots. Quill is designed for simplicity and ease of use, making it perfect for generating basic plots for reports, examples, or any application that needs clean, vector-based visualizations.

## Features

- 🎨 **Basic Styling**: Choose colors, marker types, and line styles
- 📏 **Simple Layouts**: Configurable dimensions, titles, axis labels, legends, and grids
- 🔧 **Builder Pattern**: Clean API with method chaining
- 📈 **Multiple Data Series**: Support for multiple datasets on a single plot
- 🖼️ **SVG Output**: Export to SVG files or return as `svg::Document` for programmatic use
- 🖼️ **PNG Support**: Optional PNG output via `png` feature
- ⚡ **Lightweight**: Minimal dependencies for fast compilation

## Quick Start

Add Quill to your `Cargo.toml`:

```toml
[dependencies]
quill = "0.1.6"
```

### Basic Line Plot

```rust
use quill::*;

fn main() {
    let data = (0..=100).map(|x| {
        let xf = x as f64 * 0.1;
        (xf, xf.sin())
    }).collect();

    let plot = Plot::builder()
        .dimensions((600, 400))
        .title("Sine Wave")
        .x_label("X Axis")
        .y_label("Y Axis")
        .data([
            Series::builder()
                .name("sin(x)")
                .color("Blue")
                .data(data)
                .line(Line::Solid)
                .build(),
        ])
        .build();
    
    plot.to_svg("output.svg").unwrap();
    
    // Or get the SVG document for programmatic use
    let svg_doc = plot.to_document();
}
```

## Examples

### Line Plot
A simple sine wave visualization with connected points:

![Line Chart](gallery/line.svg)

```rust
use quill::*;

let line_plot = Plot::builder()
    .dimensions((600, 400))
    .title("Line Graph Example")
    .x_label("X Axis")
    .y_label("Y Axis")
    .legend(Legend::TopRightOutside)
    .grid(Grid::Solid)
    .data([
        Series::builder()
            .name("Sine Curve")
            .color("Blue")
            .data(line_data())
            .marker(Marker::None)
            .line(Line::Solid)
            .build(),
    ])
    .build();
```

### Scatter Plot
Data points without connecting lines:

![Scatter Plot](gallery/scatter.svg)

```rust
use quill::*;

let scatter_plot = Plot::builder()
    .dimensions((600, 400))
    .title("Scatter Graph Example")
    .legend(Legend::TopRightOutside)
    .grid(Grid::Dashed)
    .data([
        Series::builder()
            .name("Lissajous Curve")
            .color("Red")
            .data(scatter_data())
            .marker(Marker::Circle)
            .marker_size(5.0)
            .line(Line::None)  // No connecting lines
            .build(),
    ])
    .build();
```

### Multi-Series Plot
Multiple datasets on the same plot:

![Monthly Sales](gallery/monthly_sales.svg)

```rust
use quill::*;

let plot = Plot::builder()
    .dimensions((900, 500))
    .title("Sales Data")
    .x_label("Month")
    .y_label("Units Sold")
    .legend(Legend::TopLeftInside)
    .grid(Grid::Solid)
    .data([
        Series::builder()
            .name("Product A")
            .color(Color::Blue) // Both Color::(ColorName) and string colors are supported
            .data(product_a_data)
            .marker(Marker::Circle)
            .line(Line::Solid)
            .build(),
        Series::builder()
            .name("Product B")
            .color("Red")
            .data(product_b_data)
            .marker(Marker::Square)
            .line(Line::Dotted)
            .build(),
        // Add more series as needed
    ])
    .build();
```

### Investment Growth Data
Points with different line styles:

![Investment Growth](gallery/investment_growth.svg)

```rust
use quill::*;

let plot = Plot::builder()
    .dimensions((800, 600))
    .title("Hypothetical Investment Growth")
    .x_label("Years")
    .y_label("Value ($)")
    .x_range(Range::Manual { min: 0.0, max: 10.0 })
    .legend(Legend::TopLeftInside)
    .grid(Grid::Dotted)
    .font("Times New Roman")
    .data([
        Series::builder()
            .name("Low-Risk Investment")
            .color("Green")
            .data(low_risk_data)
            .marker(Marker::Circle)
            .line(Line::Solid)
            .build(),
        // Add more investment types
    ])
    .build();
```

## API Overview

### Plot Builder
Configure your plot with the builder pattern:

```rust
Plot::builder()
    .dimensions((width, height))           // Plot size
    .title("Plot Title")                   // Chart title
    .x_label("X Axis")                     // X-axis label
    .y_label("Y Axis")                     // Y-axis label
    .x_range(Range::Auto)                  // X-axis range (Auto or Manual)
    .y_range(Range::Auto)                  // Y-axis range (Auto or Manual)
    .legend(Legend::TopRightOutside)       // Legend position
    .grid(Grid::Solid)                     // Grid style
    .font("Arial")                         // Font family
    .margin(Margin::default())             // Plot margins
    .data([Series])                        // Data series
    .build()
```

### Series Builder
Define data series with markers and line styling:

```rust
Series::builder()
    .name("Series Name")       // Legend name
    .color("Blue")             // Line/marker color
    .data(vec![(x, y)])        // Data points (f32, f64, i32, or i64 tuples)
    .marker(Marker::Circle)    // Point markers
    .marker_size(5.0)          // Marker size
    .line(Line::Solid)         // Line style (or Line::None for scatter)
    .build()
```

### Output Options

```rust
// Save to SVG file
plot.to_svg("output.svg").unwrap();

// Get SVG document for programmatic use
let svg_doc: svg::Document = plot.to_document();
```

### Example of Available Options

#### Markers
- `Marker::None` - No markers
- `Marker::Circle` - Circular points
- `Marker::Square` - Square points  
- `Marker::Cross` - Cross markers

#### Line Styles
- `Line::Solid` - Solid lines
- `Line::Dashed` - Dashed lines
- `Line::Dotted` - Dotted lines
- `Line::None` - No connecting lines

#### Grid Styles
- `Grid::Solid` - Solid grid lines
- `Grid::Dashed` - Dashed grid lines
- `Grid::Dotted` - Dotted grid lines
- `Grid::None` - No grid

#### Legend Positions
- `Legend::TopLeftInside`
- `Legend::TopRightInside`
- `Legend::TopRightOutside`
- `Legend::BottomLeftInside`
- `Legend::BottomRightInside`
- `Legend::BottomRightOutside`
- `Legend::LeftCenterInside`
- `Legend::RightCenterInside`
- `Legend::RightCenterOutside`
- `Legend::TopCenter`
- `Legend::BottomCenter`
- `Legend::None`

## License

This project is licensed under the Apache-2.0 License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit issues, feature requests, or pull requests.
