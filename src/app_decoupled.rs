use eframe::egui;
use egui::{Color32, FontId, Layout, RichText, Vec2, Visuals};
use serde::Deserialize;

// Helper structures matching our JSON schema for seamless deserialization
#[derive(Deserialize, Default)]
struct HeaderSchema {
    name: String,
    roles: String,
}

#[derive(Deserialize)]
struct ProjectItem {
    name: String,
    url: String,
}

#[derive(Deserialize)]
struct SkillItem {
    category: String,
    items: String,
}

#[derive(Deserialize, Default)]
struct PortfolioData {
    header: HeaderSchema,
    about_me: String,
    featured_projects: Vec<ProjectItem>,
    js_experiments: Vec<ProjectItem>,
    skills: Vec<SkillItem>,
}

pub struct PortfolioApp {
    data: PortfolioData,
}

impl Default for PortfolioApp {
    fn default() -> Self {
        let mut app = Self {
            data: PortfolioData::default(),
        };
        app.load_and_parse_data();
        app
    }
}

impl eframe::App for PortfolioApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.update_portfolio(ctx, frame);
    }
}

fn get_rich_text(text: impl Into<String>, size: f32, color: Color32) -> RichText {
    RichText::new(text.into())
        .font(FontId::proportional(size))
        .color(color)
}

impl PortfolioApp {
    /// Embeds data.json at compile time and deserializes it directly.
    fn load_and_parse_data(&mut self) {
        let content = include_str!("../assets/data.json");
        
        // Directly deserialize JSON payload without brittle line-by-line parsing loops
        match serde_json::from_str::<PortfolioData>(content) {
            Ok(parsed_data) => {
                self.data = parsed_data;
            }
            Err(err) => {
                eprintln!("Failed to parse data.json: {}. Using hardcoded fallbacks.", err);
                // Apply fallback safety guards
                self.data.header.name = "Aman Karn".to_string();
                self.data.header.roles = "Game Developer | Unity | C++ | WebGL".to_string();
                self.data.about_me = "Passionate game developer focused on building interactive experiences.".to_string();
            }
        }
    }

    /// Presentation layer: Renders content natively using structural layout patterns.
    pub fn update_portfolio(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut visuals = Visuals::dark();
        visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(0x1E, 0x1E, 0x1E); 
        visuals.widgets.noninteractive.fg_stroke.color = Color32::WHITE;
        ctx.set_visuals(visuals);

        let accent_cyan = Color32::from_rgb(0x00, 0xBC, 0xD4); 
        let link_blue = Color32::from_rgb(0x4F, 0xC3, 0xF7);   
        let header_bg = Color32::from_rgb(0x11, 0x11, 0x11);   
        let text_muted = Color32::from_rgb(0xAA, 0xAA, 0xAA);  
        let color_white = Color32::from_rgb(0xEE, 0xEE, 0xEE);  

        // 2. HEADER PANEL
        egui::TopBottomPanel::top("header")
            .frame(egui::Frame::none().fill(header_bg).inner_margin(40.0))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading(get_rich_text(&self.data.header.name, 40.0, color_white).strong());
                    ui.add_space(8.0);
                    ui.label(get_rich_text(&self.data.header.roles, 20.0, text_muted));
                });
            });

        // 3. FOOTER PANEL
        egui::TopBottomPanel::bottom("footer")
            .frame(egui::Frame::none().fill(header_bg).inner_margin(20.0))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 5.0;
                        ui.label(get_rich_text(format!("© 2026 {} |", self.data.header.name), 13.0, text_muted));
                        ui.hyperlink_to(
                            get_rich_text("Github", 13.0, link_blue),
                            "https://github.com/Dragoenix99cZar"
                        );
                    });
                });
            });

        // 4. MAIN CONTENT AREA
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    let dynamic_width = ui.available_width() * 0.60;

                    ui.allocate_ui_with_layout(
                        Vec2::new(dynamic_width, ui.available_height()),
                        Layout::top_down(egui::Align::Center),
                        |ui| {
                            ui.set_max_width(ui.available_width());
                            
                            let render_section_heading = |ui: &mut egui::Ui, text: &str| {
                                ui.add_space(30.0);
                                ui.heading(get_rich_text(text, 28.0, accent_cyan).strong());
                                ui.separator(); 
                                ui.add_space(10.0);
                            };

                            // --- ABOUT ME ---
                            render_section_heading(ui, "About Me");
                            ui.label(get_rich_text(&self.data.about_me, 20.0, color_white));

                            // --- FEATURED PROJECTS ---
                            render_section_heading(ui, "Featured Projects");
                            for project in &self.data.featured_projects {
                                ui.horizontal(|ui| {
                                    ui.label("💠");
                                    ui.hyperlink_to(get_rich_text(&project.name, 20.0, link_blue), &project.url);
                                });
                            }
                            
                            // --- JAVASCRIPT EXPERIMENTS ---
                            render_section_heading(ui, "JavaScript Experiments");
                            for experiment in &self.data.js_experiments {
                                ui.horizontal(|ui| {
                                    ui.label("💠");
                                    ui.hyperlink_to(get_rich_text(&experiment.name, 20.0, link_blue), &experiment.url);
                                });
                                println!("js: {}",experiment.name);
                            }
                            
                            // --- SKILLS ---
                            render_section_heading(ui, "Skills");
                            for skill in &self.data.skills {
                                ui.horizontal(|ui| {
                                    ui.label("💠");
                                    ui.label(get_rich_text(&skill.category, 14.0, color_white).strong());
                                    ui.label(get_rich_text(&skill.items, 14.0, color_white));
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