use std::cell::RefCell;
use std::panic;
use std::rc::Rc;

use color_eyre::owo_colors::OwoColorize;
use iced::{
    button, executor, Align, Application, Button, Clipboard, Column, Command, Container, Element,
    Length, ProgressBar, Row, Settings, Subscription, Text,
};
use tracing::info;

use crate::chain::{ChainEvent, SyncProgress};
use crate::cli::CLI;
use crate::data::{Block, Tip};
use crate::gui::subscription::{progress, SyncProgressEngine};
use crate::synchronization::Engine;

#[derive(Debug)]
pub enum State {
    UiInitialized,
    Loading,
    Loaded,
}

#[derive(Debug)]
pub enum Message {
    UiInitialized,
    Loaded,
    WS(ChainEvent),
}

#[derive(Debug)]
pub struct Explorer {
    flags: Rc<RefCell<SyncProgressEngine>>,
    engine: Box<Engine>,
    sync_progress: f32,
    block: Option<Block>,
    tip: Option<Tip>,
    state: State,
}

impl Application for Explorer {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = CLI;

    fn new(flags: Self::Flags) -> (Explorer, Command<Message>) {
        let uri = flags.ws.clone();
        let (engine, rx) = Engine::new(flags.ws);
        let sync_process_engine = SyncProgressEngine::new(uri, Some(rx));

        // let start_engine = engine.start();
        (
            Self {
                flags: Rc::new(RefCell::new(sync_process_engine)),
                engine: engine,
                sync_progress: 0.0,
                block: None,
                tip: None as Option<Tip>,
                state: State::UiInitialized,
            },
            Command::perform(async {}, |_| Message::UiInitialized), //perform(start_engine, |_| Message::Loaded),
        )
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut Clipboard,
    ) -> Command<Self::Message> {
        // info!("call update");
        match self.state {
            State::UiInitialized => match message {
                Message::UiInitialized => {
                    self.state = State::Loading;
                    let connection = self.engine.connection.clone();
                    return Command::perform(
                        async {
                            tokio::spawn(async move {
                                let _ = connection.run().await;
                            });
                        },
                        |_| Message::Loaded,
                    );
                }
                _ => panic!("oops"),
            },
            State::Loaded => match message {
                Message::WS(m) => {
                    match m {
                        ChainEvent::Collection(_) => (),
                        ChainEvent::Synchronizing(s) => match s {
                            SyncProgress::Synchronizing(progress, block, tip) => {
                                self.sync_progress = progress;
                                self.block = Some(block);
                                self.tip = Some(tip);
                            }
                            SyncProgress::Synchronized(tip) => self.sync_progress = 100.0,
                            SyncProgress::Unsynchronized => self.sync_progress = 0.0,
                        },
                        ChainEvent::RevertFork(_) => (),
                    };
                }
                _ => panic!("Loaded message received when already loaded state"),
            },
            State::Loading => match message {
                Message::Loaded => {
                    self.state = State::Loaded;
                }
                _ => {
                    panic!("we received tick before UI is ready")
                }
            },
        }

        Command::none()
    }

    fn title(&self) -> String {
        String::from("Mini Cardano Explorer")
    }

    fn subscription(&self) -> Subscription<Message> {
        // info!("call subscription");
        let flags = self.flags.take();
        if flags.s.is_some() {
            let s = flags.s;

            self.flags
                .replace(SyncProgressEngine::new(flags.uri.clone(), None));
            progress(flags.uri, s)
        } else {
            self.flags.replace(flags);
            progress(self.flags.borrow().uri.clone(), None)
        }
        .map(Message::WS)
    }

    fn view(&mut self) -> Element<Message> {
        let progress_bar = ProgressBar::new(0.0..=100.0, self.sync_progress);
        let tip_epoch = match &self.tip {
            Some(tip) => tip.epoch(),
            None => 0,
        };
        let (block_epoch, block_era) = match &self.block {
            Some(block) => (block.epoch(), format!("Current Era: {:?}", block.era())),
            None => (0, "".to_string()),
        };
        // let Some(block_epoch) = self.block.map(|t| t.epoch());

        let row: Element<_> = Row::new()
            .spacing(10)
            .padding(10)
            .align_items(Align::Center)
            .push(Text::new("Synchronization progress"))
            .spacing(10)
            .push(progress_bar)
            .push(Text::new(format!("{:.2}", self.sync_progress)).size(50))
            .spacing(10)
            .into();

        let control: Element<_> = Column::new()
            .spacing(10)
            .padding(10)
            .align_items(Align::Center)
            .push(row)
            .push(Text::new(format!("Epoch Progress {} / {}", block_epoch, tip_epoch)).size(50))
            .push(Text::new(block_era).size(50))
            .into();

        Container::new(control)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .padding(20)
            .into()
    }
}
