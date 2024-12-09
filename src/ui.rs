use crate::messages::{MessageFromUi, MessageToUi};
use crate::ui::bottom_board::render_bottom_tab;
use crate::ui::bottom_board::BottomTab;
use crate::ui::port_settings::SerialPortSettings;
use crate::ui::port_settings::{render_port_settings, SerialPortParams};
use eframe::egui;
use serialport::{available_ports, SerialPort};
use std::fmt::Display;
use std::sync::Arc;
use tokio::sync::broadcast::{Receiver, Sender};

pub mod bottom_board;
pub mod general_message;
pub mod port_settings;
pub mod upper_board;

#[derive(PartialEq, Default)]
enum Tab {
    #[default]
    Port,
    BottomBoard,
    UpperBoard,
    General,
}

pub struct Scalar3 {
    current_tab: Tab,
    serial_port_settings: SerialPortParams,
    tx: Sender<MessageFromUi>,
    rx: Receiver<MessageToUi>,
    bottom_tab: BottomTab,
    error: Option<String>
}
// pub struct  ErrorMessageBox{
//     message: String,
//     show: bool,
// }
impl Scalar3 {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        tx: Sender<MessageFromUi>,
        rx: Receiver<MessageToUi>,
    ) -> Self {
        Self {
            rx,
            tx,
            serial_port_settings: SerialPortParams::default(),
            current_tab: Tab::default(),
            bottom_tab: BottomTab::Temperature,
            error:None
        }
    }
}

impl eframe::App for Scalar3 {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        while let Ok(msg) = self.rx.try_recv() {
            match msg {
                MessageToUi::PortClosed => {
                    self.serial_port_settings.opened = false;
                }
                MessageToUi::PortOpened => {
                    self.serial_port_settings.opened = true;
                }
                MessageToUi::PortError(error)=>{
                    println!("вроде должно вывестись сообщение");
                    self.serial_port_settings.opened = false;
                    self.error = Some(error);
                }
            }
        }
        if let Some(error_message) = self.error.clone() {
            egui::Window::new("Error")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0]) // Центрируем окно
                .frame(egui::Frame::popup(&ctx.style()).inner_margin(egui::Margin::symmetric(10.0, 10.0))) // Уменьшаем внутренние отступы
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        // Заголовок ошибки
                        ui.label(
                            egui::RichText::new("⚠️ Error")
                                .color(egui::Color32::RED)
                                .size(18.0), // Уменьшаем размер текста
                        );

                        // Текст ошибки
                        ui.add_space(5.0); // Меньший отступ
                        ui.label(
                            egui::RichText::new(error_message)
                                .color(egui::Color32::LIGHT_RED)
                                .size(14.0), // Уменьшенный шрифт текста
                        );

                        // Кнопки управления
                        ui.add_space(10.0);
                        ui.horizontal(|ui| {
                            if ui.button("OK").clicked() {
                                self.error = None; // Закрыть окно
                            }
                            if ui.button("Details").clicked() {
                                println!("Error details requested"); // Обработка "Details"
                            }
                        });
                    });
                });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    if ui.button("Порт").clicked() {
                        self.current_tab = Tab::Port;
                    }
                    if ui.button("Нижняя плата").clicked() {
                        self.current_tab = Tab::BottomBoard;
                    }
                    if ui.button("Верхняя плата").clicked() {
                        self.current_tab = Tab::UpperBoard;
                    }
                    if ui.button("Общие").clicked() {
                        self.current_tab = Tab::General;
                    }
                });

                // Содержимое текущей вкладки

                ui.separator();
                match self.current_tab {
                    Tab::Port => {
                        render_port_settings(ui, &mut self.serial_port_settings, self.tx.clone(), self.rx.resubscribe())
                    }
                    Tab::BottomBoard => {
                        render_bottom_tab(ui, self);
                        // ui.heading("Нижняя плата");
                        // ui.label("Содержимое для нижней платы.");
                    }
                    Tab::UpperBoard => {
                        ui.heading("Верхняя плата");
                        ui.label("Содержимое для верхней платы.");
                    }
                    Tab::General => {
                        ui.heading("Общее сообщение");
                        ui.label("Общие настройки приложения.");
                    }
                }
            });
        });
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let le = 0.93_f32.to_le_bytes();
        let be = 0.93_f32.to_be_bytes();
        let ne = 0.93_f32.to_ne_bytes();
        let le = String::from_utf8(Vec::from(le));
        let be = String::from_utf8(Vec::from(be));
        let ne = String::from_utf8(Vec::from(ne));
        println!("le {le:?} be {be:?} ne {ne:?}");
    }
}
