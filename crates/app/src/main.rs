use eframe::egui;

struct App;

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Frame::central_panel(ui.style()).show(ui, |_ui| {});
    }
}

fn main() -> anyhow::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Pagurus")
            .with_inner_size([1100.0, 700.0]),
        ..Default::default()
    };
    eframe::run_native("Pagurus", options, Box::new(|_cc| Ok(Box::new(App))))
        .map_err(|e| anyhow::anyhow!("{e}"))
}
