use eframe::egui::{self, RichText, Ui, Window};
use crate::AIPersonality;
use crate::theme::CyberTheme;

pub struct PersonalityModal {
    pub show: bool,
    pub personality: AIPersonality,
    editing_catchphrase: String,
    theme: CyberTheme,
}

impl PersonalityModal {
    pub fn new(personality: AIPersonality) -> Self {
        Self {
            show: false,
            personality,
            editing_catchphrase: String::new(),
            theme: CyberTheme::default(),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context) -> Option<AIPersonality> {
        let mut result = None;
        
        if self.show {
            Window::new("AI Personality Settings")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .show(ctx, |ui| {
                    self.show_content(ui, &mut result);
                });
        }
        
        result
    }

    fn show_content(&mut self, ui: &mut Ui, result: &mut Option<AIPersonality>) {
        ui.vertical_centered(|ui| {
            ui.add_space(8.0);
            ui.heading(RichText::new("Voice Settings").color(self.theme.foreground));
            ui.add_space(4.0);
        });

        // Voice Settings Section
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("Voice Type:").color(self.theme.foreground));
                ui.text_edit_singleline(&mut self.personality.voice_type);
            });

            // Audio Controls
            ui.horizontal(|ui| {
                if ui.button(if self.personality.audio_enabled { "🔊 Mute" } else { "🔈 Unmute" }).clicked() {
                    self.personality.toggle_audio();
                }
                if ui.button("🔄 Reset Audio").clicked() {
                    self.personality.reset_audio();
                }
            });

            ui.add(egui::Slider::new(&mut self.personality.volume, 0.0..=1.0)
                .text("Volume")
                .clamp_to_range(true));
            ui.add(egui::Slider::new(&mut self.personality.speech_rate, 0.5..=2.0)
                .text("Speech Rate")
                .clamp_to_range(true));
        });

        ui.add_space(8.0);
        ui.vertical_centered(|ui| {
            ui.heading(RichText::new("Personality Traits").color(self.theme.foreground));
        });
        ui.add_space(4.0);

        // Personality Traits Section
        ui.group(|ui| {
            let traits = [
                ("Drunk Level", &mut self.personality.drunk_level),
                ("Sass Level", &mut self.personality.sass_level),
                ("Tech Expertise", &mut self.personality.tech_expertise),
                ("Grand Pappi References", &mut self.personality.grand_pappi_references),
                ("Enthusiasm", &mut self.personality.enthusiasm),
                ("Anxiety Level", &mut self.personality.anxiety_level),
            ];

            for (label, value) in traits {
                ui.horizontal(|ui| {
                    ui.label(RichText::new(label).color(self.theme.foreground));
                    ui.add(egui::Slider::new(value, 0.0..=1.0)
                        .clamp_to_range(true));
                });
            }
        });

        ui.add_space(8.0);
        ui.vertical_centered(|ui| {
            ui.heading(RichText::new("Catchphrases").color(self.theme.foreground));
        });
        ui.add_space(4.0);

        // Catchphrases Section
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.editing_catchphrase);
                if ui.button("Add").clicked() && !self.editing_catchphrase.is_empty() {
                    self.personality.catchphrases.push(self.editing_catchphrase.clone());
                    self.editing_catchphrase.clear();
                }
            });

            ui.add_space(4.0);
            
            let mut to_remove = None;
            for (idx, catchphrase) in self.personality.catchphrases.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(RichText::new(catchphrase).color(self.theme.foreground_dim));
                    if ui.small_button("❌").clicked() {
                        to_remove = Some(idx);
                    }
                });
            }

            if let Some(idx) = to_remove {
                self.personality.catchphrases.remove(idx);
            }
        });

        ui.add_space(16.0);

        // Bottom Buttons
        ui.horizontal(|ui| {
            if ui.button("Apply").clicked() {
                self.personality.clamp_values();
                *result = Some(self.personality.clone());
                self.show = false;
            }
            if ui.button("Cancel").clicked() {
                self.show = false;
            }
        });
    }
} 