use std::sync::mpsc::{Receiver, Sender};

use crate::mdata;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]

pub struct Octo {
    // Example stuff:
    enabled: bool,
    hiValue: f64,
    lowValue: f64,
    linear : bool,
    #[serde(skip)]
    mtx: Option<Sender<mdata>>,
}

impl Octo {
    const APP_KEY: &str = "bugfree";
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>, mtx: Sender<mdata>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            if let Some(mut data) = eframe::get_value::<Octo>(storage, Self::APP_KEY) {
                
                mtx.send(mdata::LowValue(data.lowValue)).unwrap();
                mtx.send(mdata::HiValue(data.hiValue)).unwrap();
                data.mtx = Some(mtx);
                data.enabled = false;

                return data;
            }
        }

        Self {
            mtx: Some(mtx),
            lowValue: 8.0,
            hiValue: 10.0,
            linear: true,
            enabled: false,
        }
    }
}

impl eframe::App for Octo {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, Self::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { enabled, lowValue, hiValue, mtx, linear } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Octo Clicker");

            ui.horizontal(|ui| {
                ui.label("press ctrl + shift + l to enable loser");
                ui.separator();
                if ui.checkbox(linear, "Linear?").changed() {
                    mtx.as_ref().unwrap().send(mdata::Linear(*linear)).unwrap();
                }
            });
            if *linear {
                ui.horizontal(|ui| {
                    if ui.add(egui::Slider::new(lowValue, 3.0..=30.0).text("CPS")).changed() {
                        mtx.as_ref().unwrap().send(mdata::LowValue(*lowValue)).unwrap();
                    }
                });
            } else {
                ui.horizontal(|ui| {
                    if ui.add(egui::Slider::new(lowValue, 3.0..=30.0).text("low CPS")).changed() {
                        mtx.as_ref().unwrap().send(mdata::LowValue(*lowValue)).unwrap();
                    }
                    if ui.add(egui::Slider::new(hiValue, 3.0..=30.0).text("high CPS")).changed() {
                        mtx.as_ref().unwrap().send(mdata::HiValue(*hiValue)).unwrap();
                    }
                });
            }
        });

    }
}
