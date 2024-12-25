use crate::ui::theme::theme_colors::{ThemeColors, ComponentStyles};

pub struct SystemTheme {
    pub colors: ThemeColors,
    pub styles: ComponentStyles,
}

impl SystemTheme {
    // Define the `new` method to initialize the `SystemTheme`
    pub fn new(colors: ThemeColors, styles: ComponentStyles) -> Self {
        SystemTheme { colors, styles }
    }
}
