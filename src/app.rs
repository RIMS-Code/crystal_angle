use egui::RichText;

use crate::{calculate_angle_walkoff, Harmonic};

// #[derive(PartialEq)]
// #[derive(serde::Deserialize, serde::Serialize)]
// enum Harmonic { Second, Third }

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct XtalCalculatorApp {
    wavelength: f32,
    harmonic: Harmonic,
    #[serde(skip)] //
    crystal: String,
    angle: f32,
    walkoff: f32,

}

impl Default for XtalCalculatorApp {
    fn default() -> Self {
        Self {
            wavelength: 800.0,
            harmonic: Harmonic::Second,
            crystal: "".to_string(),
            angle: 0.0,
            walkoff: 0.0,
        }
    }
}

impl XtalCalculatorApp {
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

impl eframe::App for XtalCalculatorApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading(RichText::new("Ti:Sapphire Harmonic Generation Crystal Calculator").strong());
            ui.add_space(VERTICAL_SPACE);

            ui.label(INTRO_MESSAGE);
            ui.add_space(VERTICAL_SPACE);

            ui.collapsing("Usage", |ui| {
                ui.label(USAGE_MESSAGE);
            });
            ui.add_space(VERTICAL_SPACE);

            ui.horizontal(|ui| {
                ui.label("Wavelength: ");
            });

            let wavelength_slider = egui::Slider::new(&mut self.wavelength, 700.0..=1000.0)
                .drag_value_speed(1.0)
                .smallest_positive(1.0)
                .suffix("nm");
            ui.add(wavelength_slider);
            ui.add_space(VERTICAL_SPACE);

            ui.label("Select Harmonic: ");


            ui.horizontal(|ui| {
                ui.radio_value(&mut self.harmonic, Harmonic::Second, "Second");
                ui.radio_value(&mut self.harmonic, Harmonic::Third, "Third");
            });

            ui.add_space(VERTICAL_SPACE);
            ui.separator();
            ui.add_space(VERTICAL_SPACE);

            (self.angle, self.walkoff) = calculate_angle_walkoff(self.wavelength, &self.harmonic);

            ui.label(RichText::new("Results").strong().size(14.0));
            ui.add_space(VERTICAL_SPACE);

            ui.horizontal(|ui| {
                ui.label("Crystal: ");
                ui.label(which_crystal(&self.harmonic));
            });
            ui.horizontal(|ui| {
                ui.label("Angle: ");
                ui.label(format!("{:.2}°", self.angle));
            });
            ui.horizontal(|ui| {
                ui.label("Walk-off: ");
                ui.label(format!("{:.2}mrad", self.walkoff));
            });


            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                ui.hyperlink_to("Source code.",
                                "https://github.com/RIMS-Code/crystal_angle",
                );
                egui::warn_if_debug_build(ui);
            });
        });
    }

    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}


fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}

// Constants to configure the app
const INTRO_MESSAGE: &str = "This small app calculates crystal angles and walk-off angles \
for second and third harmonic generation with Ti:Sapphire lasers.\n
The following assumptions are made:\n
- LBO crystal for second harmonic generation\n\
- BBO crystal for third harmonic generation";

const USAGE_MESSAGE: &str = "Enter the wavelength in nanometers by either dragging the slider \
to the preferred value or by clicking on the box to the right of the slider and enter the value \
You can also click and drag on the box.\n
Next select from the radio buttons if you want to generate a second or third harmonic.";

const VERTICAL_SPACE: f32 = 12.0;


fn which_crystal(harmonic: &Harmonic) -> &'static str {
    match harmonic {
        Harmonic::Second => "LBO",
        Harmonic::Third => "BBO",
    }
}