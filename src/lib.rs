pub mod app;
pub mod input;
pub mod tui;
pub mod helper;
pub mod fps;

use anyhow::Result;
use crossbeam_channel::Sender;
use ratatui::crossterm::event::{KeyEvent, MouseEvent};
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

impl<E> IEvent<E> {
    pub fn as_key(&self) -> Option<KeyEvent> {
        if let IEvent::CT(ct) = self && let CtEvent::Key(key) = ct {
            Some(*key)
        } else {
            None
        }
    }
    
    pub fn as_mouse(&self) -> Option<MouseEvent> {
        if let IEvent::CT(ct) = self && let CtEvent::Mouse(mouse) = ct {
            Some(*mouse)
        } else {
            None
        }
    }
    
    // columns, rows
    pub fn as_resize(&self) -> Option<(u16, u16)> {
        if let IEvent::CT(ct) = self && let CtEvent::Resize(columns, rows) = ct {
            Some((*columns, *rows))
        } else {
            None
        }
    }
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
