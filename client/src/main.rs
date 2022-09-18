use std::sync::mpsc::{channel, Receiver, Sender};
use std::time;
use tonic::transport::Channel;

use client::DcsGrpcClient;
use std::sync::atomic::{AtomicBool, Ordering};

pub mod client;
pub mod dcs;
pub mod gui;

struct WorkerThread {
    gui: gui::GuiWorkerInterface,
    rpc_client: DcsGrpcClient<Channel>,
}

fn background_thread_entry(mut worker: WorkerThread, done: &AtomicBool) {
    // let indication = rpc_client.list_indication(4);
    // if indication
    // println!("Indication:\n{}", indication);
    // println!("Aircraft name: {}", rpc_client.get_aircraft_name());

    loop {
        if done.load(Ordering::Acquire) {
            return;
        };

        let result = worker.rpc_client.get_aircraft_name();
        match result {
            Ok(name) => {
                let aircraft = dcs::aircraft_by_name(name);
                worker.notify_gui(gui::Message::SetConnectionState(
                    gui::ConnectionState::Connected,
                ));
                worker.notify_gui(gui::Message::SetAircraftType(aircraft));
            }
            Err(_) => worker.notify_gui(gui::Message::SetConnectionState(
                gui::ConnectionState::Disconnected,
            )),
        }
        let delay = time::Duration::from_millis(500);
        std::thread::sleep(delay);
    }
}

impl WorkerThread {
    fn notify_gui(&mut self, message: gui::Message) {
        self.gui.notify(message);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    static DONE: AtomicBool = AtomicBool::new(false);

    let gui_opts = eframe::NativeOptions::default();
    eframe::run_native(
        "gui app",
        gui_opts,
        Box::new(|_cc| {
            let (tx, rx): (Sender<gui::Message>, Receiver<gui::Message>) = channel();

            let (gui, gui_context) = gui::Gui::new(_cc, rx);
            let rpc_client = DcsGrpcClient::new();

            let worker = WorkerThread {
                rpc_client: rpc_client,
                gui: gui::GuiWorkerInterface::new(gui_context, tx),
            };

            std::thread::spawn(|| {
                background_thread_entry(worker, &DONE);
            });

            Box::new(gui)
        }),
    );

    DONE.store(true, Ordering::Release);

    Ok(())
}
