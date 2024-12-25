mod ui;

use eframe::{run_native, App};
use ui::game_ui::GameUI;

fn main() {
    run_native("TwentyOne is Gay", eframe::NativeOptions::default(), Box::new(|_cc| {
        Ok(Box::new(GameUI::default()) as Box<dyn App>)
    }));
}
