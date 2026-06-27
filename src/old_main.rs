use eframe::egui;

// mod app;
// use app::PortfolioApp; 
mod app_decoupled;
use app_decoupled::PortfolioApp; 

pub struct MyApp {
    my_portfolio: PortfolioApp,
}

impl MyApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            my_portfolio: PortfolioApp::default(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Forward the context and frame directly down to your portfolio app logic
        self.my_portfolio.update_portfolio(ctx, frame);
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
fn main() {} // Dummy main for WASM

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
                "the_canvas_id", // Must match your canvas ID in index.html
                web_options,
                Box::new(|cc| Box::new(MyApp::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });

    Ok(())
}


// trunk serve

// trunk build --release