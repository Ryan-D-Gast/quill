use crate::style::*;
use svg::Document;
use svg::node::Text as SvgNodeText;
use svg::node::element::Text;

pub fn draw_title(
    document: Document,
    title: &str,
    font: &str,
    title_config: &TitleConfig,
    plot_area_x_start: f32,
    plot_area_width: f32,
    current_effective_margin_top: f32,
) -> Document {
    if !title.is_empty() {
        let title_text_x = plot_area_x_start + plot_area_width / 2.0;
        let title_text_y = current_effective_margin_top * 0.5;
        let title_svg = Text::new()
            .set("x", title_text_x)
            .set("y", title_text_y)
            .set("font-family", font)
            .set("font-size", title_config.font_size)
            .set("fill", title_config.color.to_hex_string())
            .set("text-anchor", "middle")
            .set("dominant-baseline", "middle")
            .add(SvgNodeText::new(title));
        return document.add(title_svg);
    }
    document
}

pub fn draw_x_label(
    document: Document,
    x_label: &str,
    font: &str,
    x_label_config: &LabelConfig,
    plot_area_x_start: f32,
    plot_area_width: f32,
    plot_area_y_start: f32,
    plot_area_height: f32,
    current_effective_margin_bottom: f32,
) -> Document {
    if !x_label.is_empty() {
        let x_label_text_x = plot_area_x_start + plot_area_width / 2.0;
        let x_label_text_y =
            plot_area_y_start + plot_area_height + current_effective_margin_bottom * 0.6;
        let x_label_svg = Text::new()
            .set("x", x_label_text_x)
            .set("y", x_label_text_y)
            .set("font-family", font)
            .set("font-size", x_label_config.font_size)
            .set("fill", x_label_config.color.to_hex_string())
            .set("text-anchor", "middle")
            .set("dominant-baseline", "middle")
            .add(SvgNodeText::new(x_label));
        return document.add(x_label_svg);
    }
    document
}

pub fn draw_y_label(
    document: Document,
    y_label: &str,
    font: &str,
    y_label_config: &LabelConfig,
    current_effective_margin_left: f32,
    plot_area_y_start: f32,
    plot_area_height: f32,
) -> Document {
    if !y_label.is_empty() {
        let y_label_text_x = current_effective_margin_left * 0.3;
        let y_label_text_y = plot_area_y_start + plot_area_height / 2.0;
        let y_label_svg = Text::new()
            .set("x", y_label_text_x)
            .set("y", y_label_text_y)
            .set("font-family", font)
            .set("font-size", y_label_config.font_size)
            .set("fill", y_label_config.color.to_hex_string())
            .set("text-anchor", "middle")
            .set("dominant-baseline", "middle")
            .set(
                "transform",
                format!("rotate(-90, {}, {})", y_label_text_x, y_label_text_y),
            )
            .add(SvgNodeText::new(y_label));
        return document.add(y_label_svg);
    }
    document
}
