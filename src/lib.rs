pub mod app;
pub mod input;
pub mod tui;
pub mod helper;
pub mod fps;

use anyhow::Result;
use crossbeam_channel::Sender;
use ratatui::Frame;
use ratatui::layout::Rect;

type CtEvent = ratatui::crossterm::event::Event;

#[derive(Clone)]
pub enum IEvent<E> {
    CT(CtEvent),
    Initialize,
    Exit,
    Any(String),
    Custom(E),
}

#[derive(Clone)]
pub enum IMsg<M> {
    Exit,
    Tick,
    Pause(M),
    Any(String),
    Custom(M),
    None,
}

impl<M> IMsg<M> {
    pub fn is_pause(&self) -> bool {
        if let IMsg::Pause(_) = self {
            true
        } else {
            false
        }
    }
}

pub struct EventCtx<M> {
    sender: Sender<IMsg<M>>,
    pub consumed: bool,
}

impl<M> EventCtx<M> {
    pub fn new(sender: Sender<IMsg<M>>) -> Self {
        Self {
            sender,
            consumed: false,
        }
    }

    pub fn sender(&self) -> Sender<IMsg<M>> {
        self.sender.clone()
    }

    pub fn consumed(&mut self) {
        self.consumed = true
    }
}

#[allow(unused_variables)]
pub trait Component<E, M> {
    fn on(&mut self, event: &IEvent<E>, ctx: &mut EventCtx<M>) -> Result<()> {
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame, rect: Rect) -> Result<()> {
        Ok(())
    }
}
