use crate::{components::SQUARE_BUTTON_SIZE, graphics::SEGMENTED_CHAR_WIDTH, style};
use core::str::FromStr;
use egui::{Key, Sense, Vec2, Widget};

pub struct Numpad<'a, T>
where
    T: FromStr,
{
    side_keys: Option<[Key; 4]>,
    keys: [Option<Key>; 12],
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
                Some(Key::Num7),
                Some(Key::Num8),
                Some(Key::Num9),
                Some(Key::Num4),
                Some(Key::Num5),
                Some(Key::Num6),
                Some(Key::Num1),
                Some(Key::Num2),
                Some(Key::Num3),
                None,
                Some(Key::Num0),
                None,
            ],
            val,
            row_width: 3,
        }
    }

    pub fn with_sign(mut self) -> Self {
        self.keys[11] = Some(Key::Minus);
        self
    }

    pub fn with_decimal(mut self, separator: Key) -> Self {
        self.keys[9] = Some(separator);
        self
    }

    pub fn with_side_keys(mut self, keys: [Key; 4]) -> Self {
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

    fn digits_grid(&mut self, ui: &mut egui::Ui)
    where
        T: FromStr,
    {
        egui::Grid::new("ksui.numpad.grid")
            .striped(false)
            .spacing(SPACING)
            .show(ui, |ui| {
                for (i, &button_char) in self.keys.clone().iter().enumerate() {
                    match button_char {
                        Some(c) => {
                            self.onscreen_key(ui, c);
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

    fn onscreen_key(&mut self, ui: &mut egui::Ui, c: Key) {
        let mut button = egui::Button::new(c.symbol_or_name())
            .min_size(BUTTON_SIZE)
            .sense(Sense::click_and_drag());
        if c == Key::Escape {
            button = button.fill(style::ERROR_COLOR);
        } else if c == Key::Enter {
            button = button.fill(style::CUED_COLOR);
        };

        if button.ui(ui).clicked() {
            self.press_key(ui, c);
        }
    }

    fn press_key(&mut self, ui: &egui::Ui, key: Key) -> Option<()> {
        match key {
            Key::Minus => {
                let mut memstr = self.get_memstr(ui);
                if memstr.starts_with('-') {
                    memstr = memstr.strip_prefix('-').unwrap_or(&memstr).to_string();
                } else {
                    memstr.insert(0, '-');
                }
                self.set_memstr(ui, &memstr);
            }
            Key::Backspace => {
                self.pop_char_from_memstr(ui);
            }
            Key::Enter => {
                if let Ok(val) = self.get_memstr(ui).parse::<T>() {
                    *self.val = val;
                    self.clear_memstr(ui);
                }
            }
            Key::Escape => {
                self.clear_memstr(ui);
            }
            _ => {
                self.push_char_to_memstr(
                    ui,
                    key.symbol_or_name().chars().next()?.to_ascii_lowercase(),
                );
            }
        }
        Some(())
    }

    fn control_grid(&mut self, ui: &mut egui::Ui)
    where
        T: FromStr,
    {
        egui::Grid::new("ksui.numpad.controls")
            .striped(false)
            .spacing(SPACING)
            .show(ui, |ui| {
                self.onscreen_key(ui, Key::Backspace);
                ui.end_row();
                self.onscreen_key(ui, Key::Escape);
                ui.end_row();
                self.onscreen_key(ui, Key::Enter);
                ui.end_row();
            });
    }

    fn check_for_external_keyboard_input(&mut self, ui: &egui::Ui)
    where
        T: FromStr,
    {
        ui.memory_mut(|w| w.request_focus(ui.id()));
        for &key in self
            .keys
            .clone()
            .iter()
            .flatten()
            .chain(self.side_keys.clone().iter().flatten())
            .chain([Key::Backspace, Key::Escape, Key::Enter].iter())
        {
            if ui.input(|i| i.key_pressed(key)) {
                self.press_key(ui, key);
            }
        }
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

        self.check_for_external_keyboard_input(ui);

        egui::Grid::new("ksui.numpad.sections")
            .striped(false)
            .show(ui, |ui| {
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
                        .striped(false)
                        .spacing(SPACING)
                        .show(ui, |ui| {
                            for key in keys {
                                self.onscreen_key(ui, key);
                                ui.end_row();
                            }
                        });
                }
                self.digits_grid(ui);
                self.control_grid(ui);
            })
            .response
    }
}
