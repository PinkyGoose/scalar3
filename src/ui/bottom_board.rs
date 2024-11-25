use crate::messages::MessageFromUi;
use crate::ui::port_settings::SerialPortSettings;
use eframe::egui;
use tokio::sync::broadcast::Sender;

#[derive(PartialEq, Default)]
pub enum BottomTab {
    #[default]
    Temperature,
    Pressure,
    Voltage,
}
pub fn render_bottom_tab(ui: &mut egui::Ui) {}
