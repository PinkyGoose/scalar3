use crate::ui::port_settings::SerialPortSettings;
use crate::Regime;

#[derive(Clone, Debug)]

pub enum MessageFromUi {
    SetPortSettings(SerialPortSettings),
    SetRegime(Regime),
}
#[derive(Clone)]

pub enum MessageToUi {}
