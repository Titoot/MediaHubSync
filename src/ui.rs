use eframe::{egui::{self, RichText, FontId}, epaint::Pos2};
use egui_extras::{Column, TableBuilder};
use win_msgbox::Okay;
use widestring::U16CString;

use crate::{CONFIG, config::Path};
use crate::config;
use crate::requests::UI_STATE;

#[derive(Debug, PartialEq)]
enum Enum {
    Game,
    Series,
    Movie,
    Anime
}

pub struct MyApp {
    system_path: String,
    server_path: String,
    jwt: String,
    popup: bool,
    folder_type: crate::ui::Enum,
    is_enabled: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { 
            system_path: String::new(),
            server_path: String::new(),
            jwt: CONFIG.lock().unwrap().jwt.clone(),
            popup: false,
            folder_type: Enum::Game,
            is_enabled: true,
        }
    }
}

impl MyApp {
    fn show_popup(&mut self, ctx: &egui::Context, show_popup: &mut bool) {
        egui::Window::new("New Entry")
            .open(show_popup)
            .fixed_pos(Pos2::new(330.0/4.0, 500.0/4.0))
            .resizable(false)
            .max_size([150.0, 100.0])
            .collapsible(false)
            .show(ctx, |ui| {
                ui.add_enabled_ui(self.is_enabled, |ui| {
                    self.popup_window(ui);
                });
            });
    }

    fn popup_window(&mut self, ui: &mut egui::Ui) {
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
                        log::debug!("{}", last_two_components);
                    }
                }
            });
        });

        ui.add(
            egui::TextEdit::singleline(&mut self.server_path)
                .desired_width(150.0)
                .hint_text("Add Server Path"),
        );

        egui::ComboBox::from_label("Select one!")
        .selected_text(format!("{:?}", self.folder_type))
        .show_ui(ui, |ui| {
            ui.selectable_value(&mut self.folder_type, Enum::Game, "Game");
            ui.selectable_value(&mut self.folder_type, Enum::Movie, "Movie");
            ui.selectable_value(&mut self.folder_type, Enum::Series, "Series");
            ui.selectable_value(&mut self.folder_type, Enum::Anime, "Anime");
        });

        ui.add_space(15.0);
        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
            if ui.button("Save").clicked() {
                if config::check_path(&CONFIG.lock().unwrap(), self.server_path.to_string(), self.system_path.to_string()) {
                    show_error_message("Can't put duplicate Paths");
                    return;
                }
                if !self.system_path.is_empty() && !self.server_path.is_empty()
                {
                    config::append_path(Path::from((self.system_path.to_string(), self.server_path.to_string(), format!("{:?}", self.folder_type))));
                    log::debug!("{}\n{}", self.system_path, self.server_path);
                    self.popup = false;
                    self.server_path = String::from("");
                    self.system_path = String::from("");
                }
                else {
                    show_error_message("Can't put empty Paths");
                }
            }
        });
    }

    fn show_table(&mut self, ui: &mut egui::Ui)
    {
        let table = TableBuilder::new(ui)
                                .striped(true)
                                .resizable(false)
                                .cell_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown))
                                .column(Column::auto())
                                .column(Column::auto())
                                .column(Column::initial(100.0).range(40.0..=300.0))
                                .column(Column::initial(0.0).at_least(25.0).clip(true))
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
                header.col(|ui| {
                    ui.strong("Del");
            });
            })
            .body(|mut body| {
                let mut row_index = 1;
                let config = &mut *CONFIG.lock().unwrap();
                let paths = config.paths.clone();
                for path in &paths {
                        let row_height = 18.0;
                        body.row(row_height, |mut row| {
                            row.col(|ui| {
                                ui.label(row_index.to_string());
                            });
                            row.col(|ui| {
                            ui.add(egui::Label::new(&path.srv_path).truncate(true));
                        });
                            row.col(|ui| {
                                ui.add(egui::Label::new(&path.path).truncate(true));
                            });
                            row.col(|ui| {
                                if ui.add(egui::Button::image(egui::include_image!("../data/delete.png")).small()).clicked() 
                                {
                                    config::delete_path(&mut *config,path.srv_path.to_string(), path.path.to_string());
                                    log::debug!("{} deleted", path.srv_path);
                                }
                            });
                        });
                        row_index += 1;
                }
            });
    }

    fn show_main_ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.label(RichText::new("MediaHubSync").font(FontId::proportional(26.0)));

            ui.add_space(15.0);

            self.show_table(ui);

        });

        ui.add_space(15.0);

        ui.vertical_centered(|ui| {

            if ui.add(egui::Button::new("Add New Entry").min_size([150.0, 0.0].into())).clicked() {
                self.popup = true;
            }
        });

        ui.add_space(35.0);

        ui.label("JWT Token");
        ui.horizontal(|ui| {
            ui.add(
                egui::TextEdit::singleline(&mut self.jwt)
                    .desired_width(150.0)
                    .hint_text("Jwt Token"),
            );
            if ui.button("Save").clicked() {
                if self.jwt != String::from("")
                {
                    config::set_jwt(self.jwt.to_string());
                    log::debug!("{}", self.jwt);
                }
            }
        });
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut show_popup = self.popup;
        if self.popup {
            self.show_popup(ctx, &mut show_popup);
           // self.popup = show_popup;
        }

        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            let ui_state = UI_STATE.lock().unwrap();
            self.is_enabled = !ui_state.is_loading;
            if ui_state.is_loading {
                ui.horizontal(|ui| {
                    ui.label("Loading... Please wait.");
                    ui.spinner();
                });
            }
            ui.add_enabled_ui(self.is_enabled, |ui| {
                self.show_main_ui(ui);
            });
        });
    }
}

fn show_error_message(message: &str) {
    let message = U16CString::from_str(message).unwrap();
    let _ = win_msgbox::error::<Okay>(message.as_ptr()).show().unwrap();
}

fn get_last_two_components(path: &std::path::PathBuf) -> String {
    if let (Some(parent), Some(file_name)) = (path.parent(), path.file_name()) {
        let parent_str = match parent.file_name() {
            Some(file_name) => file_name.to_string_lossy(),
            None => return file_name.to_string_lossy().to_string(),
         };
        let file_name_str = file_name.to_string_lossy();
        return format!("\\{}\\{}", parent_str, file_name_str);
    }
    path.display().to_string()
}