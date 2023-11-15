use eframe::egui;
use egui::{Visuals, Style};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        ..Default::default()
    };

    eframe::run_native("practice egui", options, Box::new(|cc| {
        let style = Style {visuals: Visuals::dark(), ..Style::default() };
        cc.egui_ctx.set_style(style);
        Box::new(EnJaDictionay::new())
    }))
}

struct EnJaDictionay {
    text: String
}

impl EnJaDictionay {
    fn new() -> Self { 
        EnJaDictionay { text: String::from("hogehoghe") }
    }
}

impl eframe::App for EnJaDictionay {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        egui::TopBottomPanel::top("text").resizable(true).show(ctx, |ui| {
            let mut text = self.text.to_owned();
            let text_edit = egui::TextEdit::multiline(&mut text);
            let response = ui.add_sized(ui.available_size(), text_edit);

            if response.changed() {
                self.text = text.to_string();
            }
        });
        egui::SidePanel::left("word_list").resizable(true).show(ctx, |ui| {
            ui.label("word list");
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("means");
        });
    }
}
