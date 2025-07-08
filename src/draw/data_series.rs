use crate::PlotValue;
use crate::elements::{Interpolation, Line, Marker};
use crate::series::Series;
use svg::node::element::{Group, Path, Rectangle};

pub fn draw_data_series<T, Fx, Fy>(
    data: &[Series<T>],
    map_x: Fx,
    map_y: Fy,
) -> Group
where
    T: PlotValue,
    Fx: Fn(T) -> f32,
    Fy: Fn(T) -> f32,
{
    let mut data_group = Group::new().set("clip-path", "url(#plotAreaClip)");
    for series in data {
        let series_color_hex = series.color.to_hex_string();

        // Draw lines/curves based on interpolation type
        if series.line != Line::None && series.data.len() > 1 {
            let line_path = match series.interpolation {
                Interpolation::Linear => draw_linear_path(series, &map_x, &map_y),
                Interpolation::Step => draw_step_path(series, &map_x, &map_y),
                Interpolation::Bezier => draw_bezier_path(series, &map_x, &map_y),
                Interpolation::Spline => draw_spline_path(series, &map_x, &map_y),
            };

            if let Some(mut path) = line_path {
                path = path
                    .set("fill", "none")
                    .set("stroke", series_color_hex.clone())
                    .set("stroke-width", series.line_width);
                if series.line == Line::Dashed {
                    path = path.set("stroke-dasharray", "5 5");
                }
                data_group = data_group.add(path);
            }
        }

        // Draw markers
        if series.marker != Marker::None {
            let marker_size = series.marker_size;
            for &(data_x, data_y) in &series.data {
                let screen_x = map_x(data_x);
                let screen_y = map_y(data_y);
                match series.marker {
                    Marker::Circle => {
                        let circle = svg::node::element::Circle::new()
                            .set("cx", screen_x)
                            .set("cy", screen_y)
                            .set("r", marker_size / 2.0)
                            .set("fill", series_color_hex.clone());
                        data_group = data_group.add(circle);
                    }
                    Marker::Square => {
                        let square = Rectangle::new()
                            .set("x", screen_x - marker_size / 2.0)
                            .set("y", screen_y - marker_size / 2.0)
                            .set("width", marker_size)
                            .set("height", marker_size)
                            .set("fill", series_color_hex.clone());
                        data_group = data_group.add(square);
                    }
                    Marker::Cross => {
                        let d = marker_size / 2.0;
                        let cross_data = svg::node::element::path::Data::new()
                            .move_to((screen_x - d, screen_y - d))
                            .line_to((screen_x + d, screen_y + d))
                            .move_to((screen_x - d, screen_y + d))
                            .line_to((screen_x + d, screen_y - d));
                        let cross_path = Path::new()
                            .set("d", cross_data)
                            .set("stroke", series_color_hex.clone())
                            .set("stroke-width", 1.0)
                            .set("fill", "none");
                        data_group = data_group.add(cross_path);
                    }
                    Marker::None => {}
                }
            }
        }
    }
    data_group
}

// Helper functions for different interpolation types
fn draw_linear_path<T, Fx, Fy>(series: &Series<T>, map_x: &Fx, map_y: &Fy) -> Option<Path>
where
    T: PlotValue,
    Fx: Fn(T) -> f32,
    Fy: Fn(T) -> f32,
{
    if series.data.is_empty() {
        return None;
    }

    let mut line_data = svg::node::element::path::Data::new();
    if let Some((first_x, first_y)) = series.data.first() {
        line_data = line_data.move_to((map_x(*first_x), map_y(*first_y)));
        for (x, y) in series.data.iter().skip(1) {
            line_data = line_data.line_to((map_x(*x), map_y(*y)));
        }
    }
    Some(Path::new().set("d", line_data))
}

fn draw_step_path<T, Fx, Fy>(series: &Series<T>, map_x: &Fx, map_y: &Fy) -> Option<Path>
where
    T: PlotValue,
    Fx: Fn(T) -> f32,
    Fy: Fn(T) -> f32,
{
    if series.data.is_empty() {
        return None;
    }

    let mut line_data = svg::node::element::path::Data::new();
    if let Some((first_x, first_y)) = series.data.first() {
        line_data = line_data.move_to((map_x(*first_x), map_y(*first_y)));

        for window in series.data.windows(2) {
            let (_curr_x, curr_y) = window[0];
            let (next_x, _next_y) = window[1];

            // Draw horizontal line to next x position
            line_data = line_data.line_to((map_x(next_x), map_y(curr_y)));
            // Draw vertical line to next y position
            line_data = line_data.line_to((map_x(next_x), map_y(window[1].1)));
        }
    }
    Some(Path::new().set("d", line_data))
}

fn draw_bezier_path<T, Fx, Fy>(series: &Series<T>, map_x: &Fx, map_y: &Fy) -> Option<Path>
where
    T: PlotValue,
    Fx: Fn(T) -> f32,
    Fy: Fn(T) -> f32,
{
    if series.data.len() < 2 {
        return None;
    }

    let mut line_data = svg::node::element::path::Data::new();
    let points: Vec<(f32, f32)> = series
        .data
        .iter()
        .map(|(x, y)| (map_x(*x), map_y(*y)))
        .collect();

    if let Some(&first_point) = points.first() {
        line_data = line_data.move_to(first_point);

        for i in 1..points.len() {
            let current = points[i - 1];
            let next = points[i];

            // Calculate control points for smooth curve
            let control_distance =
                ((next.0 - current.0).powi(2) + (next.1 - current.1).powi(2)).sqrt() * 0.25;

            let prev_point = if i > 1 { points[i - 2] } else { current };
            let next_next_point = if i < points.len() - 1 {
                points[i + 1]
            } else {
                next
            };

            // Control point 1 (end of current segment)
            let slope1_x = (next.0 - prev_point.0) / 2.0;
            let slope1_y = (next.1 - prev_point.1) / 2.0;
            let length1 = (slope1_x.powi(2) + slope1_y.powi(2)).sqrt();
            let cp1_x = current.0
                + if length1 > 0.0 {
                    slope1_x / length1 * control_distance
                } else {
                    0.0
                };
            let cp1_y = current.1
                + if length1 > 0.0 {
                    slope1_y / length1 * control_distance
                } else {
                    0.0
                };

            // Control point 2 (start of next segment)
            let slope2_x = (next_next_point.0 - current.0) / 2.0;
            let slope2_y = (next_next_point.1 - current.1) / 2.0;
            let length2 = (slope2_x.powi(2) + slope2_y.powi(2)).sqrt();
            let cp2_x = next.0
                - if length2 > 0.0 {
                    slope2_x / length2 * control_distance
                } else {
                    0.0
                };
            let cp2_y = next.1
                - if length2 > 0.0 {
                    slope2_y / length2 * control_distance
                } else {
                    0.0
                };

            line_data = line_data.cubic_curve_to(((cp1_x, cp1_y), (cp2_x, cp2_y), next));
        }
    }
    Some(Path::new().set("d", line_data))
}

fn draw_spline_path<T, Fx, Fy>(series: &Series<T>, map_x: &Fx, map_y: &Fy) -> Option<Path>
where
    T: PlotValue,
    Fx: Fn(T) -> f32,
    Fy: Fn(T) -> f32,
{
    if series.data.len() < 3 {
        // Fall back to linear for insufficient points
        return draw_linear_path(series, map_x, map_y);
    }

    let mut line_data = svg::node::element::path::Data::new();
    let points: Vec<(f32, f32)> = series
        .data
        .iter()
        .map(|(x, y)| (map_x(*x), map_y(*y)))
        .collect();

    if let Some(&first_point) = points.first() {
        line_data = line_data.move_to(first_point);

        // Simple cardinal spline implementation
        let tension = 0.5; // Controls how tight the curve is

        for i in 1..points.len() {
            let p0 = if i > 1 { points[i - 2] } else { points[i - 1] };
            let p1 = points[i - 1];
            let p2 = points[i];
            let p3 = if i < points.len() - 1 {
                points[i + 1]
            } else {
                points[i]
            };

            // Calculate control points using cardinal spline formula
            let cp1_x = p1.0 + tension * (p2.0 - p0.0) / 6.0;
            let cp1_y = p1.1 + tension * (p2.1 - p0.1) / 6.0;
            let cp2_x = p2.0 - tension * (p3.0 - p1.0) / 6.0;
            let cp2_y = p2.1 - tension * (p3.1 - p1.1) / 6.0;

            line_data = line_data.cubic_curve_to(((cp1_x, cp1_y), (cp2_x, cp2_y), p2));
        }
    }
    Some(Path::new().set("d", line_data))
}
