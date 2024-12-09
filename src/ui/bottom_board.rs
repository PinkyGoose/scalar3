mod voltage;

use crate::messages::MessageFromUi;
use crate::ui::port_settings::{render_port_settings, SerialPortSettings};
use crate::ui::{Scalar3, Tab};
use eframe::egui;
use tokio::sync::broadcast::Sender;

#[derive(PartialEq, Default)]
pub enum BottomTab {
    #[default]
    Temperature,
    Pressure,
    Voltage,
}
pub fn render_bottom_tab(ui: &mut egui::Ui, scalar3: &mut Scalar3) {
    ui.horizontal(|ui| {
        if ui.button("Температура").clicked() {
            scalar3.bottom_tab = BottomTab::Temperature;
        }
        if ui.button("Давление").clicked() {
            scalar3.bottom_tab = BottomTab::Pressure;
        }
        if ui.button("Напряжение").clicked() {
            scalar3.bottom_tab = BottomTab::Voltage;
        }
    });
    ui.separator();
    match scalar3.bottom_tab {
        // Tab::Port => {
        //     render_port_settings(ui,  &mut self.serial_port_settings, self.tx.clone(), self.rx.resubscribe())
        // }
        // Tab::BottomBoard => {
        //     render_bottom_tab(ui);
        //     ui.heading("Нижняя плата");
        //     ui.label("Содержимое для нижней платы.");
        // }
        // Tab::UpperBoard => {
        //     ui.heading("Верхняя плата");
        //     ui.label("Содержимое для верхней платы.");
        // }
        // Tab::General => {
        //     ui.heading("Общее сообщение");
        //     ui.label("Общие настройки приложения.");
        // }
        BottomTab::Temperature => {
            ui.heading("Температура");
            ui.label("Вкладка с температурой");
        }
        BottomTab::Pressure => {
            ui.heading("Давление");
            ui.label("Вкладка с давлением");
        }
        BottomTab::Voltage => {
            voltage::render(ui, scalar3);
        }
    }
}
