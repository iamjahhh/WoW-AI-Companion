pub mod my_widget;
pub mod system_theme;
pub mod theme_colors;

use eframe::egui::{Color32, Visuals};
use my_widget::MyWidget;
use crate::ui::theme::system_theme::SystemTheme;
use crate::ui::theme::theme_colors::{ColorScheme, CommonColors, TextColors, BackgroundColors, ThemeColors, ComponentStyles, PanelStyles, BorderStyles};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ThemeStyle {
    #[default]
    Alliance,
    Horde,
    Dark,
    Light,
}

pub struct AppColors {
    pub background: Color32,
    pub text_primary: Color32,
    pub text_secondary: Color32,
    pub alliance_bg: Color32,
    pub horde_bg: Color32,
    pub panel_bg: Color32,
    pub shadow: Color32,
    pub header_bg: Color32,
    pub header_text: Color32,
    pub alliance_text: Color32,
    pub horde_text: Color32,
    pub button_active: Color32,
}

impl Default for AppColors {
    fn default() -> Self {
        Self {
            background: Color32::from_rgb(0, 0, 0),
            text_primary: Color32::from_rgb(255, 255, 255),
            text_secondary: Color32::from_gray(200),
            alliance_bg: Color32::from_rgb(0, 0, 255),
            horde_bg: Color32::from_rgb(255, 0, 0),
            panel_bg: Color32::from_rgb(50, 50, 50),
            shadow: Color32::from_rgb(30, 30, 30),
            header_bg: Color32::from_rgb(30, 144, 255),
            header_text: Color32::from_rgb(255, 255, 255),
            alliance_text: Color32::from_rgb(255, 255, 255),
            horde_text: Color32::from_rgb(255, 255, 255),
            button_active: Color32::from_rgb(33, 150, 243),
        }
    }
}

pub struct Theme {
    pub my_widget_instance: MyWidget,
    pub colors: AppColors,
    pub visuals: Visuals,
    pub style: ThemeStyle,
}

impl Theme {
    pub fn new() -> Self {
        Self {
            my_widget_instance: my_widget::MyWidget::new(),
            colors: AppColors::default(),
            visuals: Visuals::dark(), // You can change this to Visuals::light() if you prefer the light theme
            style: ThemeStyle::default(),
        }
    }
}

pub fn run_theme() {
    let colors = ThemeColors {
        alliance: ColorScheme {
            primary: Color32::from_rgb(0, 0, 255),
            secondary: Color32::from_rgb(100, 100, 255),
            accent: Color32::from_rgb(255, 255, 0),
        },
        horde: ColorScheme {
            primary: Color32::from_rgb(255, 0, 0),
            secondary: Color32::from_rgb(255, 100, 100),
            accent: Color32::from_rgb(0, 255, 0),
        },
        common: CommonColors {
            text: TextColors {
                primary: Color32::from_rgb(255, 255, 255),
                secondary: Color32::from_gray(200),
                disabled: Color32::from_gray(100),
            },
            background: BackgroundColors {
                primary: Color32::from_rgb(50, 50, 50),
                secondary: Color32::from_rgb(30, 30, 30),
                elevated: Color32::from_rgb(70, 70, 70),
            },
        },
    };

    let styles = ComponentStyles {
        panels: PanelStyles::default(),
        borders: BorderStyles::default(),
    };
    let system_theme: SystemTheme = SystemTheme::new(colors, styles);
    // Use `system_theme` as needed
}
