use crate::ui::port_settings::{render_port_settings,  InterfaceSettings};
use eframe::egui;
use serialport::{SerialPort, SerialPortInfo};

mod bottom_board;
mod general_message;
mod port_settings;
mod upper_board;

#[derive(PartialEq, Default)]
enum Tab {
    #[default]
    Port,
    BottomBoard,
    UpperBoard,
    General,
}
#[derive(Default)]
pub struct Scalar3 {
    current_tab: Tab,
    port_settings: SerialPortSettings,
}
pub struct SerialPortSettings {
    interface: InterfaceSettings,
    baud_rate: u32
}
impl Default for SerialPortSettings{
    fn default() -> Self {
        Self{
            interface: Default::default(),
            baud_rate: 9600,
        }
    }
}
impl Scalar3 {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
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
                Tab::Port => render_port_settings(ui, &mut self.port_settings),
                Tab::BottomBoard => {
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
