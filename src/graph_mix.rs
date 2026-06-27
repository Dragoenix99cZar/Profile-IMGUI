use eframe::egui;
use egui::Color32;
use std::f64::consts::PI;

#[derive(Clone, Copy)]
struct UIWave {
    frequency: f32,
    amplitude: f32,
    damping: f32, // <-- Added damping factor (0.0 means no damping)
    enabled: bool,
}

struct GraphDim {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

pub struct GraphingApp {
    waves: Vec<UIWave>,
    graph_dimension: GraphDim,
}

impl Default for GraphingApp {
    fn default() -> Self {
        Self {
            waves: vec![
                UIWave { frequency: 1.5, amplitude: 8.0, damping: 1.0, enabled: true }, // Starts with elegant decay
                UIWave { frequency: 5.0, amplitude: 2.0, damping: 0.0, enabled: false },
            ],
            graph_dimension: GraphDim { left: -0.5, right: 11.0, top: 11.0, bottom: -11.0 },
        }
    }
}

impl GraphingApp {
    pub fn render(&mut self, ui: &mut egui::Ui) {
        // --- PAGE HEADER ---
        ui.vertical_centered(|ui| {
            ui.add_space(10.0);
            ui.heading(
                egui::RichText::new("📊 Signal Convolution Sandbox")
                    .font(egui::FontId::proportional(28.0))
                    .strong(),
            );
            ui.label("Add multiple sine waves together to generate complex, convoluted signals in real-time.");
            ui.add_space(15.0);
        });

        // --- DYNAMIC PROP-WIDTH LAYOUT SYSTEM ---
        let total_width = ui.available_width();
        let left_column_width = total_width * 0.25;  // 35% allocated for controls
        let right_column_width = total_width * 0.72; // 60% allocated for the visual graph
        let gap_spacer = total_width * 0.05;         // 5% margin gap between columns

        let mut combined_waves = vec![];

        ui.horizontal(|ui| {
            // ==========================================
            // LEFT COLUMN: PARAMETER SLIDERS (35% Width)
            // ==========================================
            ui.allocate_ui(egui::vec2(left_column_width, ui.available_height()), |ui| {
                ui.vertical(|ui| {
                    ui.spacing_mut().slider_width = ui.available_width() - 80.0;
                    ui.group(|ui| {
                        ui.set_min_width(ui.available_width());
                        ui.heading("Wave Parameters");
                        ui.separator();
                        ui.add_space(5.0);
                        
                        let mut to_remove = None;
                        
                        // Calculate slider sizing to stretch perfectly within the 35% boundary
                        // Subtracts 90.0 pixels to leave cleanly padded room for text labels
                        let target_slider_width = ui.available_width() - 90.0;

                        for (i, wave) in self.waves.iter_mut().enumerate() {
                            ui.checkbox(&mut wave.enabled, format!("Wave #{}", i + 1));
                            
                            if wave.enabled {
                                // Freq Slider
                                ui.add_sized(
                                    [target_slider_width, 0.0],
                                    egui::Slider::new(&mut wave.frequency, 0.1..=20.0).text("Freq (Hz)")
                                );
                                ui.add_space(2.0);
                                
                                // Amplitude Slider
                                ui.add_sized(
                                    [target_slider_width, 0.0],
                                    egui::Slider::new(&mut wave.amplitude, 0.0..=10.0).text("Amplitude")
                                );
                                ui.add_space(2.0);

                                // --- ADDED: Damping Slider ---
                                ui.add_sized(
                                    [target_slider_width, 0.0],
                                    egui::Slider::new(&mut wave.damping, 0.0..=5.0).text("Damping (λ)")
                                );
                                
                                combined_waves.push(*wave);
                            }
                            
                            ui.add_space(5.0);
                            if ui.button("🗑 Remove Wave").clicked() {
                                to_remove = Some(i);
                            }
                            ui.separator();
                            ui.add_space(5.0);
                        }

                        if let Some(idx) = to_remove {
                            self.waves.remove(idx);
                        }

                        if ui.button("➕ Add New Sine Wave").clicked() {
                            self.waves.push(UIWave { frequency: 2.0, amplitude: 1., damping: 2.0, enabled: true });
                        }
                    });
                    ui.add_space(20.0);
                    ui.add(egui::Slider::new(&mut self.graph_dimension.left, -2.0..=-0.01).text("X Lim left"));
                    ui.add_space(8.0);
                    ui.add(egui::Slider::new(&mut self.graph_dimension.right, 5.0..=20.0).text("X Lim Right"));
                    ui.add_space(15.0);
                    ui.add(egui::Slider::new(&mut self.graph_dimension.top, 5.0..=15.0).text("Y Lim Top"));
                    ui.add_space(8.0);
                    ui.add(egui::Slider::new(&mut self.graph_dimension.bottom, -15.0..=-0.01).text("Y Lim Bottom"));
                });
            });

            // --- SPACING GAP ---
            ui.add_space(gap_spacer);

            // ==========================================
            // RIGHT COLUMN: REAL-TIME PLOT (60% Width)
            // ==========================================
            ui.allocate_ui(egui::vec2(right_column_width, ui.available_height()), |ui| {
                ui.vertical(|ui| {
                    ui.heading("Visualized Superposition");
                    ui.add_space(5.0);
                    
                    // Generate sampling resolution points (Higher number = Smoother wave curve)
                    let n_points = 5000;
                    let line_points: egui_plot::PlotPoints = (0..n_points)
                        .map(|i| {
                            let x = (i as f64 / n_points as f64) * 10.0; // Bumped window to 4 seconds to view the decay trail better
                            let mut y = 0.0;
                            
                            for wave in &combined_waves {
                                let amp = wave.amplitude as f64;
                                let freq = wave.frequency as f64;
                                let decay = wave.damping as f64;
                                
                                // Math: y += Amplitude * e^(-decay * time) * sin(2 * pi * freq * time)
                                y += amp * (-decay * x).exp() * (2.0 * PI * freq * x).sin();
                            }
                            [x, y]
                        })
                        .collect();

                    let line = egui_plot::Line::new(line_points)
                        .color(Color32::from_rgb(0x00, 0xBC, 0xD4)) // Accent Cyan matching your theme
                        .width(2.5);

                    egui::Frame::none()
                        .fill(Color32::from_rgb(0x1A, 0x1A, 0x1A)) // Darker tint to isolate the graph area
                        .inner_margin(20.0)                        // Adjust this number for a wider/narrower margin
                        .rounding(2.0)                             // Soft rounded corners
                        .show(ui, |ui| {
                            egui_plot::Plot::new("convoluted_wave_plot")
                                .view_aspect(1.9) 
                                .include_x(self.graph_dimension.left)
                                .include_x(self.graph_dimension.right)
                                .include_y(self.graph_dimension.top)
                                .include_y(self.graph_dimension.bottom)
                                .allow_zoom(false)
                                .allow_drag(false)
                                .show(ui, |plot_ui| plot_ui.line(line));
                    });
                });
            });
        });
    }
}