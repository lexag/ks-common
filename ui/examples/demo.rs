use egui::CentralPanel;

#[derive(Default)]
struct DemoApp {}

impl DemoApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let a = Self {};

        // cloning here does not feel right but it seems to work
        ks_common_ui::style::load_fonts(&mut cc.egui_ctx.clone());
        a
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
