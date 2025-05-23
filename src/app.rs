use serde::Serialize;

use crate::course_format::CourseFormat;
use crate::snowflake_id_format::SnowflakeIdFormat;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    tabs: Tabs,
    course_format: CourseFormat,
    snowflake_id_format: SnowflakeIdFormat,
}

#[derive(serde::Deserialize, Serialize, Default, Debug, PartialEq, Eq, Clone, Copy)]
enum Tabs {
    #[default]
    Welcome,
    CourseFormat,
    SnowflakeIdFormat,
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                ui.selectable_value(&mut self.tabs, Tabs::Welcome, "Welcome");
                ui.selectable_value(&mut self.tabs, Tabs::CourseFormat, "Format your course!");
                ui.selectable_value(&mut self.tabs, Tabs::SnowflakeIdFormat, "Format your Snowflake ID!");
                ui.add_space(16.0);

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| match self.tabs {
            Tabs::Welcome => crate::welcome::ui(ui),
            Tabs::CourseFormat => self.course_format.ui(ui),
            Tabs::SnowflakeIdFormat => self.snowflake_id_format.ui(ui),
        });
    }
}