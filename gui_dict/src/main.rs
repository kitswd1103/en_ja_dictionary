use std::{io::{Read, self}, fs, path::Path};

use eframe::egui;
use egui::{Visuals, Style};

const FONT_DIR: &str = "./fonts";

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
    font_index: usize,
    font_list: Vec<(String, String)>,
    text: String,
}

impl EnJaDictionay {
    fn new() -> Self { 
        let mut font_list = vec![("default".to_owned(), String::new())];
        let local_font = get_font_files();
        if local_font.is_ok() {
            font_list.extend(local_font.unwrap())
        } else {
            println!("フォント一覧の取得に失敗しました");
        }

        // TODO: フォントの設定の保存及び読み込み機能を作成する;
        EnJaDictionay {
            font_index: 0,
            font_list,
            text: String::from("")
        }
    }
    
    fn show_font_list(&self, ui: &mut egui::Ui) -> Option<usize> {
        let mut ret = None;
        for (i, font) in self.font_list.iter().enumerate() {
            let font_name = font.0.to_owned();

            let radio = egui::RadioButton::new(i == self.font_index, font_name);
            let response = ui.add(radio);
            if response.clicked() {
                ret = Some(i);
            }
        }
        ret
    }

    fn update_font(&self, ctx: &egui::Context, index: usize) -> Option<()>{
        let mut fonts = egui::FontDefinitions::default();
        if index == 0 {
            ctx.set_fonts(fonts);
            return Some(());
        }
        let font_name = self.font_list.get(index);
        if font_name.is_none() {
            return None;
        }
        let font_path = get_font_path(&font_name.unwrap());
        let font_file = read_font_file(&font_path).unwrap_or_default();
        if font_file.is_empty() {
            return None;
        }
        fonts.font_data.insert("my_font".to_owned(),
            egui::FontData::from_owned(font_file));

        fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap()
            .insert(0, "my_font".to_owned());
        ctx.set_fonts(fonts);
        Some(())
    }
}

impl eframe::App for EnJaDictionay {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("text").resizable(true).show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Fonts", |ui| {
                    if let Some(select_index) = self.show_font_list(ui) {
                        if self.update_font(&ctx, select_index).is_some() {
                            self.font_index = select_index;
                        } else {
                            println!("Fontの変更に失敗しました")
                        }
                        ui.close_menu();
                    }
                })
            });

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

fn get_font_files() -> io::Result<impl Iterator<Item = (String, String)>> {
    Ok(fs::read_dir(FONT_DIR)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.file_type().ok()?.is_file() {
                get_font_file_stem_and_extension(&entry.file_name().to_string_lossy().into_owned())
            } else {
                None
            }
        })
    )
}

fn get_font_file_stem_and_extension(file_name: &str) -> Option<(String, String)> {
    let pos = file_name.rfind(".")?;
    let stem = file_name[0..pos].to_owned();
    let extension = file_name[(pos + 1)..].to_owned();

    match extension.to_lowercase().as_str() {
        "ttf" | "otf" => { Some((stem, extension)) }
        _ => None
    }
}

fn get_font_path(file_name: &(String, String)) -> String {
    let file_name = [file_name.0.clone(), file_name.1.clone()].join(".");
    Path::new(FONT_DIR).join(file_name).to_string_lossy().to_string()
}

fn read_font_file(path: &String) -> io::Result<Vec<u8>> {
    let mut buf = Vec::new();
    fs::File::open(path)?.read_to_end(&mut buf)?;
    Ok(buf)
}

#[cfg(test)]
mod test {
    use crate::get_font_file_stem_and_extension;

    #[test]
    fn test_get_font_file_stem_and_extension() {
        assert_eq!(get_font_file_stem_and_extension("hello world"), None);
        assert_eq!(get_font_file_stem_and_extension("hello .world"), None);
        assert_eq!(get_font_file_stem_and_extension("hello world.ttf"), Some(("hello world".to_owned(), "ttf".to_owned())));
        assert_eq!(get_font_file_stem_and_extension("hello world.otf"), Some(("hello world".to_owned(), "otf".to_owned())));
        assert_eq!(get_font_file_stem_and_extension("hello .world.ttf"), Some(("hello .world".to_owned(), "ttf".to_owned())));
        assert_eq!(get_font_file_stem_and_extension("hello .world.otf"), Some(("hello .world".to_owned(), "otf".to_owned())));
        assert_eq!(get_font_file_stem_and_extension("hello world.Otf"), Some(("hello world".to_owned(), "Otf".to_owned())));
        assert_eq!(get_font_file_stem_and_extension("hello world.TTF"), Some(("hello world".to_owned(), "TTF".to_owned()))); 
    }
}
