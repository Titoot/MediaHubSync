use std::collections::HashMap;

use eframe::{egui::{self, RichText, FontId}, epaint::Pos2};
use egui_extras::{Column, TableBuilder};

use crate::CONFIG;
use crate::config;

#[derive(Default)]
pub struct MyApp {
    system_path: String,
    server_path: String,
    popup: bool
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut show_popup = self.popup;
        if self.popup {
            egui::Window::new("New Entry")
            .open(&mut show_popup)
            .fixed_pos(Pos2::new(330.0/4.0, 500.0/4.0))
            .resizable(false)
            .max_size([150.0, 100.0])
            .collapsible(false)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {

                    ui.horizontal(|ui| {
                        let system_path_buf = std::path::PathBuf::from(self.system_path.clone());
                        ui.add(
                            egui::TextEdit::singleline(&mut get_last_two_components(&system_path_buf))
                                .desired_width(100.0)
                                .interactive(false)
                                .hint_text("Choose folder"),
                        );
                
                        if ui.button("Browse").clicked() {
                            if let Some(path) = rfd::FileDialog::new().pick_folder() {
                                let last_two_components = get_last_two_components(&path);
                
                                self.system_path = path.to_string_lossy().to_string();
                                println!("{}", last_two_components);
                            }
                        }
                    });
                });

                ui.add(
                    egui::TextEdit::singleline(&mut self.server_path)
                        .desired_width(150.0)
                        .hint_text("Add Server Path"),
                );
                ui.add_space(15.0);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    if ui.button("Save").clicked() {
                        if self.system_path != String::from("") && self.server_path != String::from("")
                        {
                            config::append_path(HashMap::from([(self.server_path.to_string(), self.system_path.to_string())]));
                            println!("{}\n{}", self.system_path, self.server_path);
                            self.popup = false;
                            self.server_path = String::from("");
                            self.system_path = String::from("");
                        }
                    }
                });
            });
            self.popup = show_popup;
    }

        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            ui.vertical_centered(|ui| {
                ui.label(RichText::new("MediaSubSync").font(FontId::proportional(26.0)));

                ui.add_space(15.0);

                let table = TableBuilder::new(ui)
                                .striped(true)
                                .resizable(false)
                                .cell_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown))
                                .column(Column::auto())
                                .column(Column::auto())
                                .column(Column::initial(100.0).range(40.0..=300.0))
                                .column(Column::initial(100.0).at_least(40.0).clip(true))
                                .column(Column::remainder())
                                .min_scrolled_height(0.0)
                                .max_scroll_height(130.0);

                table.header(20.0, |mut header| {
                        
                        header.col(|ui| {
                                ui.strong("Row");
                        });
                        header.col(|ui| {
                                ui.strong("Server Path");
                        });
                        header.col(|ui| {
                                ui.strong("System Path");
                        });
                    })
                    .body(|mut body| {
                        let mut row_index = 1;
                        for path in &CONFIG.lock().unwrap().paths {
                            for (key, value) in path {
                                let row_height = 18.0;
                                body.row(row_height, |mut row| {
                                   row.col(|ui| {
                                       ui.label(row_index.to_string());
                                   });
                                   row.col(|ui| {
                                    ui.add(egui::Label::new(key).truncate(true));
                                });
                                   row.col(|ui| {
                                       ui.add(egui::Label::new(value).truncate(true));
                                   });
                                });
                                row_index += 1;
                            }
                        }
                    });
                });

            ui.add_space(15.0);

            ui.vertical_centered(|ui| {

                if ui.add(egui::Button::new("Add New Entry").min_size([150.0, 0.0].into())).clicked() {
                        println!("{}", self.server_path);
                        self.popup = true;

                }
            });
        });
    }
}

fn get_last_two_components(path: &std::path::PathBuf) -> String {
    if let (Some(parent), Some(file_name)) = (path.parent(), path.file_name()) {
        let parent_str = parent.file_name().unwrap().to_string_lossy();
        let file_name_str = file_name.to_string_lossy();
        return format!("\\{}\\{}", parent_str, file_name_str);
    }
    path.display().to_string()
}