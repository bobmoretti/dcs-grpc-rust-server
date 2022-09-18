use eframe;
use eframe::egui;
use std::sync::mpsc::{channel, Receiver, Sender};

use egui::FontFamily::Proportional;
use egui::FontId;
use egui::TextStyle;

use crate::dcs;

#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionState {
    Disconnected,
    Connected,
}

pub enum Message {
    SetConnectionState(ConnectionState),
    SetAircraftType(dcs::Aircraft),
}

pub struct GuiWorkerInterface {
    context: egui::Context,
    channel_tx: Sender<Message>,
}
impl GuiWorkerInterface {
    pub fn notify(&mut self, message: Message) {
        self.channel_tx
            .send(message)
            .expect("Should be able to send a message to GUI thread");
        self.context.request_repaint();
    }

    pub fn new(context: egui::Context, channel_tx: Sender<Message>) -> Self {
        Self {
            context: context,
            channel_tx: channel_tx,
        }
    }
}
struct GuiState {
    connection_state: ConnectionState,
    channel_rx: Receiver<Message>,
    aircraft_type: dcs::Aircraft,
}

impl Default for GuiState {
    fn default() -> GuiState {
        GuiState {
            connection_state: ConnectionState::Disconnected,
            channel_rx: channel().1,
            aircraft_type: dcs::Aircraft::Unknown,
        }
    }
}

impl GuiState {
    fn handle_message(&mut self, message: Message) {
        match message {
            Message::SetConnectionState(state) => {
                self.connection_state = state;
            }
            Message::SetAircraftType(aircraft_type) => {
                self.aircraft_type = aircraft_type;
            }
        }
    }

    fn is_connected(&self) -> bool {
        self.connection_state == ConnectionState::Connected
    }
}

#[derive(Default)]
pub struct Gui {
    state: GuiState,
}

impl Gui {
    pub fn new(cc: &eframe::CreationContext<'_>, rx: Receiver<Message>) -> (Self, egui::Context) {
        let ctx = cc.egui_ctx.clone();
        let mut style = (*cc.egui_ctx.style()).clone();

        // Redefine text_styles
        style.text_styles = [
            (TextStyle::Heading, FontId::new(40.0, Proportional)),
            (
                TextStyle::Name("Heading2".into()),
                FontId::new(35.0, Proportional),
            ),
            (
                TextStyle::Name("Context".into()),
                FontId::new(30.0, Proportional),
            ),
            (TextStyle::Body, FontId::new(25.0, Proportional)),
            (TextStyle::Monospace, FontId::new(20.0, Proportional)),
            (TextStyle::Button, FontId::new(20.0, Proportional)),
            (TextStyle::Small, FontId::new(14.0, Proportional)),
        ]
        .into();

        // Mutate global style with above changes
        ctx.set_style(style);
        (
            Self {
                state: GuiState {
                    channel_rx: rx,
                    ..GuiState::default()
                },
            },
            ctx,
        )
    }
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            while let Ok(message) = self.state.channel_rx.try_recv() {
                self.state.handle_message(message);
            }

            ui.heading("DTC");

            let connection_string = if self.state.is_connected() {
                "Connected"
            } else {
                "Disconnected"
            };

            egui::Grid::new("main_grid").show(ui, |ui| {
                ui.label("DCS connection:");
                ui.label(connection_string);
                ui.end_row();
                ui.label("Aircraft:");
                ui.label(self.state.aircraft_type.get_friendly_name());
                ui.end_row();

                // ui.label("Second row, first column");
                // ui.label("Second row, second column");
                // ui.label("Second row, third column");
                // ui.end_row();

                // ui.horizontal(|ui| {
                //     ui.label("Same");
                //     ui.label(format!("cell: {}", self.state.test));
                // });
                // ui.label("Third row, second column");
                // ui.end_row();
            });
        });
    }
}
