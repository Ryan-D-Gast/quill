use svg::node::element::{Rectangle, Path, Group};
use pigment::{color, Color};
use crate::series::Series;
use crate::elements::{Line, Marker};
use crate::PlotValue;
use super::to_svg_color_string;

pub fn draw_data_series<T, Fx, Fy>(data: &[Series<T>], color_fn: fn(&str) -> Option<Color>, map_x: Fx, map_y: Fy) -> Group
where
    T: PlotValue,
    Fx: Fn(T) -> f32,
    Fy: Fn(T) -> f32,
{
    let mut data_group = Group::new().set("clip-path", "url(#plotAreaClip)");
    for series in data {
        let color_val = match color_fn(&series.color) {
            Some(c) => c,
            None => color("Black").unwrap(),
        };
        let series_color_svg = to_svg_color_string(&color_val);
        if series.line != Line::None && series.data.len() > 1 {
            let mut line_data = svg::node::element::path::Data::new();
            if let Some((first_x, first_y)) = series.data.first() {
                line_data = line_data.move_to((map_x(*first_x), map_y(*first_y)));
                for (x, y) in series.data.iter().skip(1) {
                    line_data = line_data.line_to((map_x(*x), map_y(*y)));
                }
            }
            let mut line_path = Path::new()
                .set("d", line_data)
                .set("fill", "none")
                .set("stroke", series_color_svg.clone())
                .set("stroke-width", series.line_width);
            if series.line == Line::Dashed {
                line_path = line_path.set("stroke-dasharray", "5 5");
            }
            data_group = data_group.add(line_path);
        }
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
                            .set("fill", series_color_svg.clone());
                        data_group = data_group.add(circle);
                    }
                    Marker::Square => {
                        let square = Rectangle::new()
                            .set("x", screen_x - marker_size / 2.0)
                            .set("y", screen_y - marker_size / 2.0)
                            .set("width", marker_size)
                            .set("height", marker_size)
                            .set("fill", series_color_svg.clone());
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
                            .set("stroke", series_color_svg.clone())
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

