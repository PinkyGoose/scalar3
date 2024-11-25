use crate::messages::{MessageFromUi, MessageToUi};
use eframe::egui;
use serialport::{available_ports, SerialPort};
use std::fmt::Display;
use std::time::Duration;
use tokio::sync::broadcast::{Receiver, Sender};

#[derive(Clone, Debug)]
pub struct SerialPortSettings {
    pub interface: InterfaceSettings,
    pub baud_rate: u32,
    pub current_port: String,
    pub open: bool,
}
#[derive(Default, PartialEq, Clone, Debug)]
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
#[derive(Default)]
pub struct SerialP {
    port_settings: SerialPortSettings,
    serial_port: Option<Box<dyn SerialPort>>,
}
impl Default for SerialPortSettings {
    fn default() -> Self {
        Self {
            open: false,
            interface: Default::default(),
            baud_rate: 9600,
            current_port: available_ports()
                .unwrap()
                .first()
                .unwrap()
                .port_name
                .to_string(),
        }
    }
}

pub fn render_port_settings(
    ui: &mut egui::Ui,
    settings: &mut SerialPortSettings,
    tx: Sender<MessageFromUi>,
) {
    ui.heading("Настройки порта");
    ui.label("Настройте параметры подключения:");

    ui.horizontal(|ui| {
        ui.push_id("port_name", |ui| {
            ui.label("Выбранный порт:");
            if settings.open {
                ui.disable();
            }
            egui::ComboBox::from_label("")
                .selected_text(settings.current_port.to_string()) // Преобразуем в строку
                .show_ui(ui, |ui| {
                    for i in available_ports().unwrap() {
                        ui.selectable_value(
                            &mut settings.current_port,
                            i.port_name.clone(),
                            i.port_name.to_string(),
                        );
                    }
                });
        })
    });
    ui.horizontal(|ui| {
        ui.push_id("interface", |ui| {
            ui.label("Интерфейс:");
            if settings.open {
                ui.disable();
            }
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
            if settings.open {
                ui.disable();
            }
            egui::ComboBox::from_label("")
                .selected_text(&settings.baud_rate.to_string())
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut settings.baud_rate, 9600, 9600.to_string());
                    ui.selectable_value(&mut settings.baud_rate, 115200, 115200.to_string());
                });
        })
    });
    ui.separator();
    if settings.open {
        if ui.button("Закрыть").clicked() {
            settings.open = false;
            let _ = tx.send(MessageFromUi::SetPortSettings(settings.clone()));
        }
    } else {
        if ui.button("Открыть").clicked() {
            settings.open = true;
            let _ = tx.send(MessageFromUi::SetPortSettings(settings.clone()));
        }
    }
}
