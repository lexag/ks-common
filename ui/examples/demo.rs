use egui::CentralPanel;
use ks_common_ui::style;

#[derive(Default)]
struct DemoApp {}

impl DemoApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // cloning here does not feel right but it seems to work
        cc.egui_ctx.set_style(style::style());
        ks_common_ui::style::load_fonts(&mut cc.egui_ctx.clone());

        Self {}
    }
}

impl eframe::App for DemoApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, ks_common_ui::demo::demo_ui);
    }
}

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "KSUI Demo",
        Default::default(),
        Box::new(|cc| Ok(Box::new(DemoApp::new(cc)))),
    )
}
