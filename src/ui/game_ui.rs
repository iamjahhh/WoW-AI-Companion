use eframe::{egui, App};
use egui::{Button, Ui};

pub struct GameUI<'a> {
    main_menu: MainMenu<'a>,
    game_screen: GameScreen,
    settings_screen: SettingsScreen,
    current_screen: Screen,
}

enum Screen {
    MainMenu,
    Game,
    Settings,
}

impl<'a> Default for GameUI<'a> {
    fn default() -> Self {
        GameUI {
            main_menu: MainMenu::default(),
            game_screen: GameScreen::default(),
            settings_screen: SettingsScreen::default(),
            current_screen: Screen::MainMenu,
        }
    }
}

impl<'a> App for GameUI<'a> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_screen {
                Screen::MainMenu => self.main_menu.show(ui),
                Screen::Game => self.game_screen.show(ui),
                Screen::Settings => self.settings_screen.show(ui),
            }
        });
    }
}

struct MainMenu<'a> {
    start_button: Button<'a>,
    settings_button: Button<'a>,
}

impl<'a> Default for MainMenu<'a> {
    fn default() -> Self {
        MainMenu {
            start_button: Button::new("Start Game"),
            settings_button: Button::new("Settings"),
        }
    }
}

impl<'a> MainMenu<'a> {
    fn show(&mut self, ui: &mut Ui) {
        
    }
}

struct GameScreen;

impl Default for GameScreen {
    fn default() -> Self {
        GameScreen
    }
}

impl GameScreen {
    fn show(&mut self, ui: &mut egui::Ui) {
        ui.label("Game Screen");
    }
}

struct SettingsScreen;

impl Default for SettingsScreen {
    fn default() -> Self {
        SettingsScreen
    }
}

impl SettingsScreen {
    fn show(&mut self, ui: &mut egui::Ui) {
        ui.label("Settings Screen");
    }
}