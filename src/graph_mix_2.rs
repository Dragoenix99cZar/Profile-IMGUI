use eframe::egui;
use egui::Color32;
use std::f64::consts::PI;

#[derive(Clone, Copy)]
struct UIWave {
    frequency: f32,
    amplitude: f32,
    damping: f32,
    enabled: bool,
}

pub struct GraphingApp {
    waves: Vec<UIWave>
}

impl Default for GraphingApp {
    fn default() -> Self {
        Self {
            waves: vec![
                UIWave { frequency: 1.5, amplitude: 8.0, damping: 0.0, enabled: true },
                UIWave { frequency: 5.0, amplitude: 2.0, damping: 0.0, enabled: false },
            ],
        }
    }
}

impl GraphingApp {
    pub fn render(&mut self, ui: &mut egui::Ui) {
        // Capture initial configuration state beforehand to avoid reference borrowing issues in closures
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
        let left_column_width = total_width * 0.25;  
        let right_column_width = total_width * 0.72; 
        let gap_spacer = total_width * 0.02;         

        let mut combined_waves = vec![];

        ui.horizontal(|ui| {
            // ==========================================
            // LEFT COLUMN: PARAMETER SLIDERS (25% Width)
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
                        let target_slider_width = ui.available_width() - 90.0;

                        for (i, wave) in self.waves.iter_mut().enumerate() {
                            ui.checkbox(&mut wave.enabled, format!("Wave #{}", i + 1));
                            
                            if wave.enabled {
                                ui.horizontal(|ui| {
                                    ui.add(egui::Label::new("Freq (Hz)"));
                                    ui.add_sized(
                                        [target_slider_width, 0.0],
                                        // egui::Slider::new(&mut wave.frequency, 0.1..=20.0).text("Freq (Hz)")
                                        egui::DragValue::new(&mut wave.frequency).speed(0.1)
                                    );
                                });
                                
                                ui.add_space(2.0);
                                
                                ui.horizontal(|ui| {
                                    ui.add(egui::Label::new("Amplitude"));
                                    ui.add_sized(
                                        [target_slider_width, 0.0],
                                        // egui::Slider::new(&mut wave.amplitude, 0.0..=10.0).text("Amplitude")
                                        egui::DragValue::new(&mut wave.amplitude).speed(0.25)
                                    );
                                });
                                
                                ui.add_space(2.0);

                                ui.add_sized(
                                    [target_slider_width, 0.0],
                                    egui::Slider::new(&mut wave.damping, 0.0..=3.0).text("Damping (λ)")
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
                            self.waves.push(UIWave { frequency: 2.0, amplitude: 1.0, damping: 2.0, enabled: true });
                        }
                    });
                });
            });

            // --- SPACING GAP ---
            ui.add_space(gap_spacer);

            // ==========================================
            // RIGHT COLUMN: REAL-TIME PLOT (72% Width)
            // ==========================================
            ui.allocate_ui(egui::vec2(right_column_width, ui.available_height()), |ui| {
                ui.vertical(|ui| {
                    ui.heading("Visualized Superposition");
                    ui.add_space(5.0);
                    
                    let n_points = 5000;
                    let mut individual_lines_points: Vec<Vec<[f64; 2]>> = vec![vec![]; combined_waves.len()];
                    let mut combined_points: Vec<[f64; 2]> = Vec::with_capacity(n_points);

                    for i in 0..n_points {
                        let x = (i as f64 / n_points as f64) * 10.0;
                        let mut y_combined = 0.0;
                        
                        for (idx, wave) in combined_waves.iter().enumerate() {
                            let amp = wave.amplitude as f64;
                            let freq = wave.frequency as f64;
                            let decay = wave.damping as f64;
                            
                            let y_val = amp * (-decay * x).exp() * (2.0 * PI * freq * x).sin();
                            y_combined += y_val;
                            
                            let offset = (idx + 1) as f64 * 10.0;
                            individual_lines_points[idx].push([x, y_val + offset]);
                        }
                        combined_points.push([x, y_combined]);
                    }

                    egui::Frame::none()
                        .fill(Color32::from_rgb(0x1A, 0x1A, 0x1A)) 
                        .inner_margin(20.0)                        
                        .rounding(2.0)                             
                        .show(ui, |ui| {
                            egui_plot::Plot::new("Mixed_wave_plot")
                                .view_aspect(1.9) 
                                .allow_zoom(true) // Now true to support manual wheel-zoom interactions
                                .allow_drag(true) // Now true to support dynamic drag-to-pan actions
                                .legend(egui_plot::Legend::default().position(egui_plot::Corner::LeftTop))
                                .show(ui, |plot_ui| {
                                    let colors = [
                                        Color32::from_rgb(244, 67, 54),   
                                        Color32::from_rgb(76, 175, 80),   
                                        Color32::from_rgb(255, 152, 0),  
                                        Color32::from_rgb(156, 39, 176), 
                                        Color32::from_rgb(233, 30, 99),  
                                    ];

                                    for (idx, points) in individual_lines_points.into_iter().enumerate() {
                                        let color = colors[idx % colors.len()];
                                        let line = egui_plot::Line::new(points.into_iter().collect::<egui_plot::PlotPoints>())
                                            .color(color)
                                            .width(1.5)
                                            .name(format!("Wave #{}", idx));
                                        plot_ui.line(line);
                                    }

                                    let combined_line = egui_plot::Line::new(combined_points.into_iter().collect::<egui_plot::PlotPoints>())
                                        .color(Color32::from_rgb(0x00, 0xBC, 0xD4)) 
                                        .width(2.5);
                                    plot_ui.line(combined_line);
                                });
                        });
                });
            });
        });
    }
}