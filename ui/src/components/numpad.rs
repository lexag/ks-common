use crate::{components::SQUARE_BUTTON_SIZE, graphics::SEGMENTED_CHAR_WIDTH, style};
use core::str::FromStr;
use egui::{Vec2, Widget};

pub struct Numpad<'a, T>
where
    T: FromStr,
{
    side_keys: Option<[char; 4]>,
    keys: [Option<char>; 12],
    row_width: usize,

    val: &'a mut T,
}

impl<'a, T> Numpad<'a, T>
where
    T: FromStr,
{
    pub fn new(val: &'a mut T) -> Self {
        Self {
            side_keys: None,
            keys: [
                Some('7'),
                Some('8'),
                Some('9'),
                Some('4'),
                Some('5'),
                Some('6'),
                Some('1'),
                Some('2'),
                Some('3'),
                None,
                Some('0'),
                None,
            ],
            val,
            row_width: 3,
        }
    }

    pub fn with_sign(mut self) -> Self {
        self.keys[11] = Some('-');
        self
    }

    pub fn with_decimal(mut self, separator: char) -> Self {
        self.keys[9] = Some(separator);
        self
    }

    pub fn with_side_keys(mut self, keys: [char; 4]) -> Self {
        self.side_keys = Some(keys);
        self
    }

    fn push_char_to_memstr(&self, ui: &egui::Ui, c: char) {
        ui.memory_mut(|w| {
            w.data.get_temp_mut_or(memstr_id(), String::new()).push(c);
        })
    }

    fn pop_char_from_memstr(&self, ui: &egui::Ui) {
        ui.memory_mut(|w| {
            w.data.get_temp_mut_or(memstr_id(), String::new()).pop();
        })
    }

    fn clear_memstr(&self, ui: &egui::Ui) {
        ui.memory_mut(|w| {
            w.data.get_temp_mut_or(memstr_id(), String::new()).clear();
        })
    }

    fn set_memstr(&self, ui: &egui::Ui, s: &str) {
        ui.memory_mut(|w| *w.data.get_temp_mut_or(memstr_id(), String::new()) = s.to_string())
    }

    fn get_memstr(&self, ui: &egui::Ui) -> String {
        ui.memory_mut(|w| w.data.get_temp(memstr_id()).unwrap_or(String::new()))
    }

    fn digits_grid(&self, mut memstr: String, ui: &mut egui::Ui)
    where
        T: FromStr,
    {
        egui::Grid::new("ksui.numpad.grid")
            .spacing(SPACING)
            .show(ui, |ui| {
                for (i, &button_char) in self.keys.iter().enumerate() {
                    match button_char {
                        Some(c) => {
                            let button = egui::Button::new(c.to_string())
                                .min_size(BUTTON_SIZE)
                                .ui(ui);
                            if button.clicked() {
                                if c == '-' {
                                    if memstr.starts_with('-') {
                                        memstr =
                                            memstr.strip_prefix('-').unwrap_or(&memstr).to_string();
                                    } else {
                                        memstr.insert(0, '-');
                                    }
                                    self.set_memstr(ui, &memstr);
                                } else {
                                    self.push_char_to_memstr(ui, c);
                                }
                            }
                        }
                        None => {
                            ui.allocate_space(BUTTON_SIZE);
                        }
                    }
                    if (i + 1) % self.row_width == 0 {
                        ui.end_row();
                    }
                }
            });
    }

    fn control_grid(&mut self, ui: &mut egui::Ui)
    where
        T: FromStr,
    {
        egui::Grid::new("ksui.numpad.controls")
            .spacing(SPACING)
            .show(ui, |ui| {
                let backspace = egui::Button::new("Back").min_size(BUTTON_SIZE).ui(ui);
                ui.end_row();
                let clear = egui::Button::new("Clear")
                    .min_size(BUTTON_SIZE)
                    .fill(style::ERROR_COLOR)
                    .ui(ui);
                ui.end_row();
                let confirm = egui::Button::new("Confirm")
                    .min_size(BUTTON_SIZE)
                    .fill(style::CUED_COLOR)
                    .ui(ui);
                ui.end_row();

                if backspace.clicked() {
                    self.pop_char_from_memstr(ui);
                }

                if clear.clicked() {
                    self.clear_memstr(ui);
                }

                if confirm.clicked()
                    && let Ok(val) = self.get_memstr(ui).parse::<T>()
                {
                    *self.val = val;
                    self.clear_memstr(ui);
                }
            });
    }
}

fn memstr_id() -> egui::Id {
    egui::Id::new("ksui.numpad.memstr")
}

const BUTTON_SIZE: Vec2 = Vec2::splat(SQUARE_BUTTON_SIZE);
const SPACING: Vec2 = Vec2::splat(8.0);

impl<'a, T> Widget for Numpad<'a, T>
where
    T: FromStr,
{
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        let memstr = self.get_memstr(ui);
        let parsed_val = memstr.parse::<T>();

        egui::Frame::group(ui.style())
            .show(ui, |ui| {
                egui::Grid::new("ksui.numpad.sections").show(ui, |ui| {
                    if self.side_keys.is_some() {
                        ui.allocate_space(Vec2::ZERO);
                    }
                    crate::graphics::draw_segmented_display(
                        ui,
                        &[SEGMENTED_CHAR_WIDTH; 16],
                        &memstr,
                        if parsed_val.is_ok() {
                            ui.visuals().text_color()
                        } else {
                            style::ERROR_COLOR
                        },
                        1.4,
                    );
                    ui.end_row();

                    if let Some(keys) = self.side_keys {
                        egui::Grid::new("ksui.numpad.sidekeys")
                            .spacing(SPACING)
                            .show(ui, |ui| {
                                for key in keys {
                                    if egui::Button::new(key.to_string())
                                        .min_size(BUTTON_SIZE)
                                        .ui(ui)
                                        .clicked()
                                    {
                                        self.push_char_to_memstr(ui, key);
                                    }
                                    ui.end_row();
                                }
                            });
                    }
                    self.digits_grid(memstr, ui);
                    self.control_grid(ui);
                });
            })
            .response
    }
}
