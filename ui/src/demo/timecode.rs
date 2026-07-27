use crate::interface::{AutoInlineWidgetMenu, InlineWidget};
use egui::{Key, Widget};
use ks_common_generic::smpte::{Timecode, TimecodeOffset};

pub(crate) fn demo_timecode(ui: &mut egui::Ui) {
    ui.vertical(|ui| {
        let (mut val, mut offs) = ui.memory(|r| {
            r.data
                .get_temp::<(Timecode, TimecodeOffset)>(ui.id())
                .unwrap_or((Timecode::default(), TimecodeOffset::default()))
        });

        val.auto_inline_widget_menu(ui);

        offs.auto_inline_widget_menu(ui);

        (val + offs).unwrap_or_default().inline_widget(ui);

        ui.memory_mut(|w| {
            *w.data
                .get_temp_mut_or(ui.id(), (Timecode::default(), TimecodeOffset::default())) =
                (val, offs)
        });
    });
}
