use egui::{Align2, Rect, Vec2, pos2};

pub struct Popup<'a> {
    id: egui::Id,
    window: Option<egui::Window<'a>>,
    parent_rect: Option<Rect>,
}

impl<'a> Popup<'a> {
    pub fn new(id: egui::Id) -> Self {
        let id = id.with("ksui.popup.window");
        let window = Some(
            egui::Window::new(id.short_debug_format())
                .title_bar(false)
                .auto_sized()
                .movable(false)
                .collapsible(false),
        );
        Self {
            id,
            window,
            parent_rect: None,
        }
    }

    pub fn pos_parent(mut self, parent_rect: Rect) -> Self {
        self.parent_rect = Some(parent_rect);
        self
    }

    pub fn show(
        &mut self,
        ui: &egui::Ui,
        add_contents: impl FnOnce(&mut egui::Ui),
    ) -> Option<egui::Response> {
        let mut open = self.is_open(ui);
        let mut window = self.window.take()?.open(&mut open);

        if let Some(rect) = self.parent_rect {
            let screen = ui.ctx().viewport_rect();
            let space_ul = rect.min - screen.min;
            let space_br = screen.max - rect.max;

            // FIXME: there is probably a nicer way to do this
            let (anchor, pos) = match (space_ul.x > space_br.x, space_ul.y > space_br.y) {
                (true, true) => (
                    Align2::RIGHT_BOTTOM,
                    if space_br.x > space_br.y {
                        rect.left_bottom()
                    } else {
                        rect.right_top()
                    },
                ),
                (true, false) => (
                    Align2::RIGHT_TOP,
                    if space_br.x > space_ul.y {
                        rect.right_bottom()
                    } else {
                        rect.left_top()
                    },
                ),
                (false, true) => (
                    Align2::LEFT_BOTTOM,
                    if space_ul.x > space_br.y {
                        rect.left_top()
                    } else {
                        rect.right_bottom()
                    },
                ),
                (false, false) => (
                    Align2::LEFT_TOP,
                    if space_ul.x > space_ul.y {
                        rect.right_top()
                    } else {
                        rect.left_bottom()
                    },
                ),
            };

            window = window.pivot(anchor).fixed_pos(pos);
        }

        window.show(ui.ctx(), add_contents).map(|r| r.response)
    }

    pub fn is_open(&self, ui: &egui::Ui) -> bool {
        ui.memory(|r| r.data.get_temp::<bool>(self.id).unwrap_or_default())
    }

    pub fn set_open(&self, ui: &egui::Ui, open: bool) {
        ui.memory_mut(|w| *w.data.get_temp_mut_or::<bool>(self.id, false) = open);
    }

    pub fn toggle_open(&self, ui: &egui::Ui) {
        self.set_open(ui, !self.is_open(ui));
    }
}
