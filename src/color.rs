#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    // Popular colors with their RGB values
    Black,
    White,
    Red,
    Green,
    Blue,
    Yellow,
    Orange,
    Purple,
    Pink,
    Brown,
    Gray,
    DarkGray,
    LightGray,
    Cyan,
    Magenta,
    Lime,
    Navy,
    Teal,
    Silver,
    Maroon,
    Olive,
    Aqua,
    Fuchsia,
    DarkRed,
    DarkGreen,
    DarkBlue,
    LightRed,
    LightGreen,
    LightBlue,
    Crimson,
    Gold,
    Indigo,
    Coral,
    Salmon,
    Khaki,
    Violet,
    // Custom color variants
    Rgb(u8, u8, u8),
    Hex(String),
}

impl Color {
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        match self {
            Color::Black => (0, 0, 0),
            Color::White => (255, 255, 255),
            Color::Red => (255, 0, 0),
            Color::Green => (0, 128, 0),
            Color::Blue => (0, 0, 255),
            Color::Yellow => (255, 255, 0),
            Color::Orange => (255, 165, 0),
            Color::Purple => (128, 0, 128),
            Color::Pink => (255, 192, 203),
            Color::Brown => (165, 42, 42),
            Color::Gray => (128, 128, 128),
            Color::DarkGray => (64, 64, 64),
            Color::LightGray => (192, 192, 192),
            Color::Cyan => (0, 255, 255),
            Color::Magenta => (255, 0, 255),
            Color::Lime => (0, 255, 0),
            Color::Navy => (0, 0, 128),
            Color::Teal => (0, 128, 128),
            Color::Silver => (192, 192, 192),
            Color::Maroon => (128, 0, 0),
            Color::Olive => (128, 128, 0),
            Color::Aqua => (0, 255, 255),
            Color::Fuchsia => (255, 0, 255),
            Color::DarkRed => (139, 0, 0),
            Color::DarkGreen => (0, 100, 0),
            Color::DarkBlue => (0, 0, 139),
            Color::LightRed => (255, 182, 193),
            Color::LightGreen => (144, 238, 144),
            Color::LightBlue => (173, 216, 230),
            Color::Crimson => (220, 20, 60),
            Color::Gold => (255, 215, 0),
            Color::Indigo => (75, 0, 130),
            Color::Coral => (255, 127, 80),
            Color::Salmon => (250, 128, 114),
            Color::Khaki => (240, 230, 140),
            Color::Violet => (238, 130, 238),
            Color::Rgb(r, g, b) => (*r, *g, *b),
            Color::Hex(hex) => {
                // Parse hex color (with or without #)
                let hex = hex.trim_start_matches('#');
                if hex.len() == 6 {
                    if let (Ok(r), Ok(g), Ok(b)) = (
                        u8::from_str_radix(&hex[0..2], 16),
                        u8::from_str_radix(&hex[2..4], 16),
                        u8::from_str_radix(&hex[4..6], 16),
                    ) {
                        (r, g, b)
                    } else {
                        (0, 0, 0) // Default to black if parsing fails
                    }
                } else if hex.len() == 3 {
                    // Handle short hex format like #RGB -> #RRGGBB
                    if let (Ok(r), Ok(g), Ok(b)) = (
                        u8::from_str_radix(&hex[0..1].repeat(2), 16),
                        u8::from_str_radix(&hex[1..2].repeat(2), 16),
                        u8::from_str_radix(&hex[2..3].repeat(2), 16),
                    ) {
                        (r, g, b)
                    } else {
                        (0, 0, 0) // Default to black if parsing fails
                    }
                } else {
                    (0, 0, 0) // Default to black for invalid hex
                }
            }
        }
    }
    
    pub fn to_hex_string(&self) -> String {
        let (r, g, b) = self.to_rgb();
        format!("#{:02x}{:02x}{:02x}", r, g, b)
    }
    
    pub fn to_rgb_string(&self) -> String {
        let (r, g, b) = self.to_rgb();
        format!("rgb({}, {}, {})", r, g, b)
    }
    
    /// Create an RGB color
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color::Rgb(r, g, b)
    }
    
    /// Create a hex color
    pub fn hex(hex_str: &str) -> Self {
        Color::Hex(hex_str.to_string())
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::Black
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from(rgb: (u8, u8, u8)) -> Self {
        Color::Rgb(rgb.0, rgb.1, rgb.2)
    }
}

impl From<&str> for Color {
    fn from(s: &str) -> Self {
        if s.starts_with('#') || (s.len() == 6 && s.chars().all(|c| c.is_ascii_hexdigit())) {
            Color::Hex(s.to_string())
        } else {
            // Parse common color names
            match s.to_lowercase().as_str() {
                "black" => Color::Black,
                "white" => Color::White,
                "red" => Color::Red,
                "green" => Color::Green,
                "blue" => Color::Blue,
                "yellow" => Color::Yellow,
                "orange" => Color::Orange,
                "purple" => Color::Purple,
                "pink" => Color::Pink,
                "brown" => Color::Brown,
                "gray" | "grey" => Color::Gray,
                "darkgray" | "darkgrey" => Color::DarkGray,
                "lightgray" | "lightgrey" => Color::LightGray,
                "cyan" => Color::Cyan,
                "magenta" => Color::Magenta,
                "lime" => Color::Lime,
                "navy" => Color::Navy,
                "teal" => Color::Teal,
                "silver" => Color::Silver,
                "maroon" => Color::Maroon,
                "olive" => Color::Olive,
                "aqua" => Color::Aqua,
                "fuchsia" => Color::Fuchsia,
                "darkred" => Color::DarkRed,
                "darkgreen" => Color::DarkGreen,
                "darkblue" => Color::DarkBlue,
                "lightred" => Color::LightRed,
                "lightgreen" => Color::LightGreen,
                "lightblue" => Color::LightBlue,
                "crimson" => Color::Crimson,
                "gold" => Color::Gold,
                "indigo" => Color::Indigo,
                "coral" => Color::Coral,
                "salmon" => Color::Salmon,
                "khaki" => Color::Khaki,
                "violet" => Color::Violet,
                _ => Color::Black, // Default to black for unknown names
            }
        }
    }
}

impl From<String> for Color {
    fn from(s: String) -> Self {
        Color::from(s.as_str())
    }
}
