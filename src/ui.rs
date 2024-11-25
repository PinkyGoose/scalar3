use crate::messages::{MessageFromUi, MessageToUi};
use crate::ui::bottom_board::render_bottom_tab;
use crate::ui::bottom_board::BottomTab;
use crate::ui::port_settings::render_port_settings;
use crate::ui::port_settings::SerialPortSettings;
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
    serial_port_settings: SerialPortSettings,
    tx: Sender<MessageFromUi>,
    rx: Receiver<MessageToUi>,
    bottom_tab: BottomTab,
}

impl Scalar3 {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        tx: Sender<MessageFromUi>,
        rx: Receiver<MessageToUi>,
    ) -> Self {
        Self {
            rx,
            tx,
            serial_port_settings: SerialPortSettings::default(),
            current_tab: Tab::default(),
            bottom_tab: BottomTab::Temperature,
        }
    }
}

impl eframe::App for Scalar3 {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
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
                    render_port_settings(ui, &mut self.serial_port_settings, self.tx.clone())
                }
                Tab::BottomBoard => {
                    render_bottom_tab(ui);
                    ui.heading("Нижняя плата");
                    ui.label("Содержимое для нижней платы.");
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
    }
}
