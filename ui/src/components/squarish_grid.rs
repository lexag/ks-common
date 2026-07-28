use egui::Response;

/// How many times longer the long side is allowed to be before wrapping should occur
const ASPECT_LIMIT: f32 = 2.5;

pub struct SquarishGrid {
    expected_count: usize,
    live_count: usize,
    horizontal: bool,
}

impl SquarishGrid {
    pub fn new(num_items: usize) -> Self {
        Self {
            expected_count: num_items,
            live_count: 0,
            horizontal: false,
        }
    }

    pub fn add(&mut self, ui: &mut egui::Ui, add_child: impl FnOnce(&mut egui::Ui)) {
        let child_container = ui.horizontal(|ui| (add_child)(ui));

        // We measure how much we grew and
        // assume all other children are the same size.
        let child_size = child_container.response.rect.size();

        let total_primary_axis = self.expected_count as f32
            * if self.horizontal {
                child_size.x
            } else {
                child_size.y
            };
        let single_secondary_axis = if self.horizontal {
            child_size.y
        } else {
            child_size.x
        };

        let num_on_secondary_axis =
            (total_primary_axis / single_secondary_axis / ASPECT_LIMIT).ceil() as usize;

        if self.expected_count > 4 && (self.live_count + 1).is_multiple_of(num_on_secondary_axis) {
            ui.end_row();
        }
        self.live_count += 1;
    }

    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        add_contents: impl FnOnce(&mut egui::Ui, &mut Self),
    ) -> Response {
        egui::Grid::new(ui.id().with("ksui.selectorlist"))
            .show(ui, |ui| (add_contents)(ui, self))
            .response
    }

    pub fn horizontal(mut self) -> Self {
        self.horizontal = true;
        self
    }
}
