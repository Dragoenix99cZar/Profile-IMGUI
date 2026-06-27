use eframe::egui;

mod app_decoupled;
mod graph_mix_2; // <-- Register your new file module

use app_decoupled::PortfolioApp;
use graph_mix_2::GraphingApp;

// Create a state enum to keep track of what screen is active
#[derive(PartialEq)]
pub enum ActivePage {
    Portfolio,
    Graphing,
}

pub struct MyApp {
    active_page: ActivePage,
    my_portfolio: PortfolioApp,
    my_graphing: GraphingApp, // <-- Store graphing application state
}

impl MyApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            active_page: ActivePage::Portfolio, // Start on the portfolio page
            my_portfolio: PortfolioApp::default(),
            my_graphing: GraphingApp::default(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        
        // Render a persistent Top Navigation Bar across your whole site
        egui::TopBottomPanel::top("nav_bar")
            .frame(egui::Frame::none().fill(egui::Color32::from_rgb(0x11, 0x11, 0x11)).inner_margin(10.0))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.active_page, ActivePage::Portfolio, "👤 My Portfolio");
                    ui.add_space(20.0);
                    ui.selectable_value(&mut self.active_page, ActivePage::Graphing, "📊 Signal Grapher");
                });
            });

        // Switch screens dynamically depending on navigation selection state
        match self.active_page {
            ActivePage::Portfolio => {
                self.my_portfolio.update_portfolio(ctx, frame);
            }
            ActivePage::Graphing => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        self.my_graphing.render(ui);
                    });
                });
            }
        }
    }
}

// ==========================================
// Launcher logic for Native (Desktop) vs Web
// ==========================================
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My App",
        native_options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let web_options = eframe::WebOptions::default();
    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id",
                web_options,
                Box::new(|cc| Box::new(MyApp::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
    Ok(())
}