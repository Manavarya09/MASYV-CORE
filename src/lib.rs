use eframe::egui;

pub struct HeliosApp {
    command_input: String,
    output_messages: Vec<String>,
}

impl Default for HeliosApp {
    fn default() -> Self {
        Self {
            command_input: String::new(),
            output_messages: vec!["HELIOS v0.1.0 initialized.".to_string()],
        }
    }
}

impl eframe::App for HeliosApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.columns(3, |columns| {
                columns[0].vertical(|ui| {
                    ui.heading("COMMANDS");
                    ui.separator();
                    ui.label("System");
                    ui.label("AI");
                    ui.label("Files");
                    ui.label("Network");
                    ui.label("Settings");
                });

                columns[1].vertical(|ui| {
                    ui.heading("INPUT");
                    ui.separator();
                    ui.text_edit_singleline(&mut self.command_input);
                    if ui.button("EXECUTE").clicked() {
                        if !self.command_input.is_empty() {
                            self.output_messages.push(format!("> {}", self.command_input.clone()));
                            self.command_input.clear();
                        }
                    }
                    ui.separator();
                    ui.heading("OUTPUT");
                    ui.separator();
                    for msg in &self.output_messages {
                        ui.label(msg);
                    }
                });

                columns[2].vertical(|ui| {
                    ui.heading("STATUS");
                    ui.separator();
                    ui.label("System: Online");
                    ui.label("AI: Ready");
                    ui.label("Memory: 0MB");
                });
            });
        });
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_fullsize_content_view(true)
            .with_decorations(false)
            .with_transparent(true),
        ..Default::default()
    };

    eframe::run_native(
        "HELIOS",
        native_options,
        Box::new(|_cc| Ok(Box::new(HeliosApp::default()))),
    )
}