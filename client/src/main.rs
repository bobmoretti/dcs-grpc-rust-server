use std::sync::mpsc::{channel, Receiver, Sender};
use std::time;
use tonic::transport::Channel;

use client::DcsGrpcClient;
use std::sync::atomic::{AtomicBool, Ordering};

pub mod client;
pub mod dcs;
pub mod gui;

struct WorkerThread {
    done: AtomicBool,
    tx: Sender<gui::Message>,
    rpc_client: DcsGrpcClient<Channel>,
}

fn background_thread_entry(mut state: WorkerThread) {
    loop {
        if state.done.load(Ordering::Acquire) {
            return;
        };

        let result = state.rpc_client.get_aircraft_name();
        match result {
            Ok(name) => {
                let aircraft = dcs::aircraft_by_name(name);
                state.notify_gui(gui::Message::SetConnectionState(
                    gui::ConnectionState::Connected,
                ));
                state.notify_gui(gui::Message::SetAircraftType(aircraft));
            }
            Err(_) => state.notify_gui(gui::Message::SetConnectionState(
                gui::ConnectionState::Disconnected,
            )),
        }
        let delay = time::Duration::from_millis(500);
        std::thread::sleep(delay);
    }
}

impl WorkerThread {
    fn notify_gui(&mut self, message: gui::Message) {
        self.tx
            .send(message)
            .expect("Should be able to send a message to GUI thread");
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rpc_client = DcsGrpcClient::new();
    let indication = rpc_client.list_indication(4)?;
    println!("Indication:\n{}", indication);
    println!("Aircraft name: {}", rpc_client.get_aircraft_name()?);

    let (tx, rx): (Sender<gui::Message>, Receiver<gui::Message>) = channel();

    let worker = WorkerThread {
        done: AtomicBool::new(false),
        tx: tx,
        rpc_client: rpc_client,
    };

    std::thread::spawn(move || {
        background_thread_entry(worker);
    });

    let gui_opts = eframe::NativeOptions::default();
    let state = Box::new(gui::GuiState {
        channel_rx: rx,
        ..gui::GuiState::default()
    });

    eframe::run_native(
        "gui app",
        gui_opts,
        Box::new(|_cc| Box::new(gui::Gui::new(_cc, state))),
    );

    Ok(())
}
