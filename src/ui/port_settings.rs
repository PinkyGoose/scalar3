use crate::ui::SerialPortSettings;
use eframe::egui;
use std::fmt::Display;
use serialport::SerialPort;

#[derive(Default, PartialEq)]
pub enum InterfaceSettings {
    #[default]
    RS232,
    RS485,
}
impl Display for InterfaceSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str: String = match self {
            InterfaceSettings::RS232 => "RS-232".parse().unwrap(),
            InterfaceSettings::RS485 => "RS-485".parse().unwrap(),
        };
        write!(f, "{}", str)
    }
}
// #[derive(Default, PartialEq)]
// pub enum BaudRate {
//     #[default]
//     _9600(u32),
//     _115200(u32),
// }

// impl Display for BaudRate {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let str: String = match self {
//             BaudRate::_9600 => "9600".parse().unwrap(),
//             BaudRate::_115200 => "115200".parse().unwrap(),
//         };
//         write!(f, "{}", str)
//     }
// }
pub fn render_port_settings(ui: &mut egui::Ui, settings: &mut SerialPortSettings) {
    ui.heading("Настройки порта");
    ui.label("Настройте параметры подключения:");

    ui.horizontal(|ui| {
        ui.push_id("interface", |ui| {
            ui.label("Интерфейс:");
            egui::ComboBox::from_label("")
                .selected_text(settings.interface.to_string()) // Преобразуем в строку
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut settings.interface,
                        InterfaceSettings::RS232,
                        InterfaceSettings::RS232.to_string(),
                    );
                    ui.selectable_value(
                        &mut settings.interface,
                        InterfaceSettings::RS485,
                        InterfaceSettings::RS485.to_string(),
                    );
                });
        })
    });

    ui.horizontal(|ui| {
        ui.push_id("baud", |ui| {
            ui.label("Скорость:");
            egui::ComboBox::from_label("")
                .selected_text(&settings.baud_rate.to_string())
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut settings.baud_rate, 9600, 9600.to_string());
                    ui.selectable_value(
                        &mut settings.baud_rate,
                        115200,
                        115200.to_string(),
                    );
                });
        })
    });
    ui.separator();
    if ui.button("Открыть").clicked() {
    }
}
