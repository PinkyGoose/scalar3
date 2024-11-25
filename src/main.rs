mod messages;
mod ui;

use crate::messages::{MessageFromUi, MessageToUi};
use crate::ui::Scalar3;
use std::io::Read;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::runtime::Runtime;
use tokio::sync::broadcast;
use tokio::sync::broadcast::{Receiver, Sender};

fn main() {
    let native_options = eframe::NativeOptions::default();
    let (tx_from_ui, rx_to_back) = broadcast::channel(36);
    let (tx_from_back, rx_to_ui) = broadcast::channel(36);
    let rt = Runtime::new().unwrap();
    // Флаг для завершения event_loop

    let stop_flag = Arc::new(AtomicBool::new(false));
    let stop_flag_clone = Arc::clone(&stop_flag);

    let handle = rt.spawn(event_loop(tx_from_back, rx_to_back, stop_flag_clone));

    let _ = eframe::run_native(
        "Scalar3",
        native_options,
        Box::new(|cc| Ok(Box::new(Scalar3::new(cc, tx_from_ui, rx_to_ui)))),
    );

    stop_flag.store(true, Ordering::SeqCst);
    let _ = rt.block_on(handle);
}
#[derive(Default, Clone, Debug)]
struct Regime {
    // buf:&mut [u8],
    bytes_need: u32,
}
pub async fn event_loop(
    tx: Sender<MessageToUi>,
    mut rx: Receiver<MessageFromUi>,
    stop_flag: Arc<AtomicBool>,
) {
    let mut sp = None;
    let mut regime = Regime::default();
    regime.bytes_need = 17;
    while !stop_flag.load(Ordering::SeqCst) {
        if let Ok(msg) = rx.try_recv() {
            match msg {
                MessageFromUi::SetPortSettings(sets) => {
                    if sets.open {
                        if let Ok(port) = serialport::new(&sets.current_port, sets.baud_rate)
                            .timeout(Duration::from_secs(1))
                            .open()
                        {
                            sp = Some(port);
                            println!("Порт {} успешно открыт!", sets.current_port);
                        } else {
                            println!("Ошибка при открытии порта {}", sets.current_port);
                        }
                    } else {
                        println!("Зануляем порт");
                        sp = None;
                    }
                }

                MessageFromUi::SetRegime(reg) => {
                    regime = reg;
                }
            }
            //TODO обрабатываем остальные сообщения от юи
        }
        if let Some(ref mut serial) = sp {
            let bytes_available = serial
                .bytes_to_read()
                .map_err(|error| {
                    eprintln!("Ошибочка вышла {}", error);
                })
                .unwrap();
            if bytes_available >= regime.bytes_need {
                let mut buf = vec![0; regime.bytes_need as usize]; // Создаем буфер необходимой длины

                serial.read_exact(&mut buf).unwrap(); // Читаем ровно regime.bytes_need байтов
                println!("Считанные данные: {:?}", buf);
                println!("Считанные данные: {:?}", String::from_utf8(buf));
            }
        }
    }
}

// if let Ok(port) = serialport::new(&settings.current_port, settings.baud_rate)
// .timeout(Duration::from_secs(1))
// .open()
// {
// settings.serial_port = Some(port);
// println!("Порт {} успешно открыт!", settings.current_port);
// } else {
// println!("Ошибка при открытии порта {}", settings.current_port);
// }
