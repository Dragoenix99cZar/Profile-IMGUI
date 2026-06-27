use eframe::egui;
use egui::{Color32, FontId, Layout, RichText, Vec2, Visuals};

pub struct PortfolioApp {}

impl Default for PortfolioApp {
    fn default() -> Self {
        Self {}
    }
}

fn get_rich_text(text: impl Into<String>, size: f32, color: Color32) -> RichText {
    RichText::new(text.into())
        .font(FontId::proportional(size))
        .color(color)
}

impl PortfolioApp {
    // Renamed custom method to avoid trait conflicts and properly receive the context
    pub fn update_portfolio(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        // 1. Setup Custom Dark Theme (Matching your original CSS hex codes)
        let mut visuals = Visuals::dark();
        visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(0x1E, 0x1E, 0x1E); // #1e1e1e
        visuals.widgets.noninteractive.fg_stroke.color = Color32::WHITE;
        ctx.set_visuals(visuals);

        // Define Color Constants
        let accent_cyan = Color32::from_rgb(0x00, 0xBC, 0xD4); // #00bcd4
        let link_blue = Color32::from_rgb(0x4F, 0xC3, 0xF7);   // #4fc3f7
        let header_bg = Color32::from_rgb(0x11, 0x11, 0x11);   // #111111
        let text_muted = Color32::from_rgb(0xAA, 0xAA, 0xAA);  // #aaa
        let color_white = Color32::from_rgb(0xEE, 0xEE, 0xEE);  // #aaa

        // 2. HEADER PANEL
        egui::TopBottomPanel::top("header")
            .frame(egui::Frame::none().fill(header_bg).inner_margin(40.0))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    // ui.heading(RichText::new("Aman Karn").font(FontId::proportional(40.0)).strong());
                    ui.heading(
                        get_rich_text("Aman Karn", 40.0, color_white).strong()
                    );
                    ui.add_space(8.0);
                    ui.label(RichText::new("Game Developer | Unity | C++ | WebGL").font(FontId::proportional(20.0)).color(text_muted));
                });
            });

        // 3. FOOTER PANEL
        egui::TopBottomPanel::bottom("footer")
            .frame(egui::Frame::none().fill(header_bg).inner_margin(20.0))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 5.0;
                        ui.label(
                            get_rich_text("© 2026 Aman Karn |", 13.0, text_muted)
                        );
                        ui.hyperlink_to(
                            get_rich_text("Github", 13.0, link_blue),
                            "https://github.com/Dragoenix99cZar");
                    });
                });
            });

        // 4. MAIN CONTENT AREA
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    // 1. Calculate 90% of the currently available screen width
                    let dynamic_width = ui.available_width() * 0.60;

                    ui.allocate_ui_with_layout(
                        Vec2::new(dynamic_width, ui.available_height()),
                        Layout::top_down(egui::Align::Center),
                        |ui| {
                            ui.set_max_width(ui.available_width());
                            
                            let section_heading = |ui: &mut egui::Ui, text: &str| {
                                ui.add_space(30.0);
                                ui.heading(
                                    RichText::new(text).font(FontId::proportional(28.0)).color(accent_cyan).strong()
                                );
                                ui.separator(); 
                                ui.add_space(10.0);
                            };

                            // --- ABOUT ME ---
                            section_heading(ui, "About Me");
                            ui.label(
                                RichText::new(
                                    "I'm a passionate game developer focused on building engaging gameplay experiences with Unity, C++, and JavaScript. \
                                    I love experimenting with mechanics, AI, and interactive systems. Below is a selection of my personal and academic projects."
                                ).font(FontId::proportional(20.0))
                            );

                            // --- FEATURED PROJECTS ---
                            section_heading(ui, "Featured Projects");
                            let projects = [
                                ("Maze Bot (Pathfinding AI)", "js-experiments/project/index.html"),
                                ("Stealth Game (Prototype)", "stealth_game/index.html"),
                                ("First-Person Shooter (Basic Mechanics)", "fps/index.html"),
                                ("Mini Brain Game (In Progress)", "recall/index.html"),
                                ("Wave Battle Game (In Progress)", "Wave/index.html"),
                            ];
                            for (name, url) in projects {
                                ui.horizontal(|ui| {
                                    ui.label("💠");
                                    ui.hyperlink_to(
                                        RichText::new(name).font(FontId::proportional(20.0)).color(link_blue),
                                        url
                                    );
                                });
                            }
                            
                            // --- JAVASCRIPT EXPERIMENTS ---
                            section_heading(ui, "JavaScript Experiments");
                            let assignments = [
                                ("Assignment 1", "js-experiments/ast1/index.html"),
                                ("Assignment 2", "js-experiments/ast2/index.html"),
                                ("Assignment 3", "js-experiments/ast3/index.html"),
                                ("Assignment 4", "js-experiments/ast4/index.html"),
                                ("Assignment 5", "js-experiments/ast5/index.html"),
                                ("Assignment 6", "js-experiments/ast6/index.html"),
                                ("Assignment 7", "js-experiments/ast7/index.html"),
                                ("Assignment Pasco Page", "pasco/index.html"),
                                ];
                                for (name, url) in assignments {
                                    ui.horizontal(|ui| {
                                    ui.label("💠");
                                    // ui.hyperlink_to(RichText::new(name).color(link_blue), url);
                                    ui.hyperlink_to(
                                        RichText::new(name).font(FontId::proportional(20.0)).color(link_blue),
                                        url
                                    );
                                });
                            }
                            
                            // --- SKILLS ---
                            section_heading(ui, "Skills");
                            let skills = [
                                ("Languages:", "C++, C#, JavaScript, Python"),
                                ("Engines:", "Unity, Unreal (basic)"),
                                ("Web:", "HTML, CSS, JS"),
                                ("Tools:", "Git, VS Code, Blender (basic), Figma"),
                                ("Platforms:", "WebGL, PC, Android (Unity export)"),
                                ];
                                for (category, list) in skills {
                                    ui.horizontal(|ui| {
                                    ui.label("💠");
                                    ui.label(
                                        RichText::new(category).font(FontId::proportional(14.0)).color(color_white).strong(),
                                    );
                                    ui.label(
                                        RichText::new(list).font(FontId::proportional(14.0)).color(color_white),
                                    );
                                });
                            }
                            
                            ui.add_space(40.0);
                        },
                    );
                });
            });
        });
    }
}