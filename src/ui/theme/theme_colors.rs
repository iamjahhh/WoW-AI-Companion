use eframe::egui::Color32;  // Use Color32 from eframe::egui

pub struct PanelStyles {
    pub border_radius: f32,
    pub padding: f32,
}

impl Default for PanelStyles {
    fn default() -> Self {
        PanelStyles {
            border_radius: 10.0, // Default value
            padding: 8.0, // Default value
        }
    }
}

// Define BorderStyles struct
pub struct BorderStyles {
    pub width: f32,
    pub color: Color32,
}

impl Default for BorderStyles {
    fn default() -> Self {
        BorderStyles {
            width: 2.0, // Default value
            color: Color32::from_rgb(0, 0, 0), // Default black border color
        }
    }
}

pub struct ComponentStyles {
    pub panels: PanelStyles,
    pub borders: BorderStyles,
}

impl Default for ComponentStyles {
    fn default() -> Self {
        ComponentStyles {
            panels: PanelStyles::default(),
            borders: BorderStyles::default(),
        }
    }
}

// Define TextColors struct
pub struct TextColors {
    pub primary: Color32,
    pub secondary: Color32,
    pub disabled: Color32,
}

// Define BackgroundColors struct
pub struct BackgroundColors {
    pub primary: Color32,
    pub secondary: Color32,
    pub elevated: Color32,
}

// Define CommonColors struct
pub struct CommonColors {
    pub text: TextColors,
    pub background: BackgroundColors,
}

// Define ThemeColors struct
pub struct ThemeColors {
    pub alliance: ColorScheme,
    pub horde: ColorScheme,
    pub common: CommonColors,
}

// Define ColorScheme struct
pub struct ColorScheme {
    pub primary: Color32,
    pub secondary: Color32,
    pub accent: Color32,
}
