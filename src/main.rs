use std::sync::Arc;

use color_eyre::eyre::Result;
use gui::subscription::SyncProgressEngine;
use iced::{Application, Settings};
use structopt::StructOpt;
use synchronization::Engine;

mod chain;
mod cli;
mod data;
mod gui;
mod storage;
mod synchronization;
mod ws;

// #[tokio::main]
pub fn main() -> iced::Result {
    let opt = init();

    // let uri = opt.ws.clone();
    // let (engine, rx) = Engine::new(opt.ws);
    // let sync_process_engine = SyncProgressEngine::new(uri, Some(rx));
    // let settings = Settings::with_flags(sync_process_engine);
    // engine.start();
    let settings = Settings::with_flags(opt);
    gui::ui::Explorer::run(settings)
}

// #[derive(Default)]
// struct Counter {
//     value: i32,
//     increment_button: button::State,
//     decrement_button: button::State,
// }

// #[derive(Debug, Clone, Copy)]
// enum Message {
//     IncrementPressed,
//     DecrementPressed,
// }

// impl Sandbox for Counter {
//     type Message = Message;

//     fn new() -> Self {
//         Self::default()
//     }

//     fn title(&self) -> String {
//         String::from("Counter - Iced")
//     }

//     fn update(&mut self, message: Message) {
//         match message {
//             Message::IncrementPressed => {
//                 self.value += 1;
//             }
//             Message::DecrementPressed => {
//                 self.value -= 1;
//             }
//         }
//     }

//     fn view(&mut self) -> Element<Message> {
//         Column::new()
//             .padding(20)
//             .align_items(Align::Center)
//             .push(
//                 Button::new(&mut self.increment_button, Text::new("Increment"))
//                     .on_press(Message::IncrementPressed),
//             )
//             .push(Text::new(self.value.to_string()).size(50))
//             .push(
//                 Button::new(&mut self.decrement_button, Text::new("Decrement"))
//                     .on_press(Message::DecrementPressed),
//             )
//             .into()
//     }
// }

// #[tokio::main]
// async fn main() -> Result<()> {
//     let opt = init();

//     let (engine, mut rx) = Engine::new(opt.ws);

//     engine.start();

//     while let Some(e) = rx.next().await {
//         info!("{:?}", e);
//         let c = engine.chain.lock().await;
//         info!("{:?}", c.sync());
//     }

//     Ok(())
// }

fn init() -> cli::CLI {
    let _ = color_eyre::install();
    dotenv::dotenv().unwrap();
    // let file_appender = tracing_appender::rolling::hourly("./", "prefix.log");
    // let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    // tracing_subscriber::fmt().with_writer(non_blocking).init();
    tracing_subscriber::fmt().init();
    let opt = cli::CLI::from_args();
    opt
}
