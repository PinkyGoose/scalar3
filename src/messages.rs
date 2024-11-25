use crate::ui::port_settings::SerialPortSettings;
use crate::Regime;

#[derive(Clone, Debug)]

pub enum MessageFromUi {
    OpenPort(SerialPortSettings),
    ClosePort,
    SetRegime(Regime),
}
#[derive(Clone)]

pub enum MessageToUi {
    PortClosed,
    PortOpened,
    PortError(String),
}
