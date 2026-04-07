use anyhow::Result;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use std::io::{Stdout, stdout};
use bitflags::bitflags;
use crate::input::Input;

pub type TerminalBackEnd = Terminal<CrosstermBackend<Stdout>>;

pub struct Tui {
    features: Features,
    pub input: Input,
    pub terminal: Option<TerminalBackEnd>,
    state: State,
}

bitflags! {
    #[derive(Copy, Clone)]
    pub struct Features: u8 {
        const RAW_MODE = 0b0000_0001;
        const ALTERNATE_SCREEN = 0b0000_0010;
        const MOUSE_CAPTURE = 0b0000_0100;
    }
}

bitflags! {
    #[derive(Copy, Clone)]
    struct State: u8 {
        const INITIALIZED = 0b0000_0001;
        const PAUSED = 0b0000_0010;
    }
}

impl Features {
    fn init(&self) -> Result<()> {
        if self.contains(Features::RAW_MODE) {
            enable_raw_mode()?;
        }
        if self.contains(Features::ALTERNATE_SCREEN) {
            execute!(stdout(), EnterAlternateScreen)?;
        }
        if self.contains(Features::MOUSE_CAPTURE) {
            execute!(stdout(), EnableMouseCapture)?;
        }
        Ok(())
    }

    fn restore(&self) -> Result<()> {
        if self.contains(Features::MOUSE_CAPTURE) {
            execute!(stdout(), DisableMouseCapture)?;
        }
        if self.contains(Features::ALTERNATE_SCREEN) {
            execute!(stdout(), LeaveAlternateScreen)?;
        }
        if self.contains(Features::RAW_MODE) {
            disable_raw_mode()?;
        }
        Ok(())
    }

}

impl Default for Features {
    fn default() -> Features {
        Features::RAW_MODE | Features::ALTERNATE_SCREEN
    }
}

impl Tui {
    
    pub fn new(
        features: Features,
    ) -> Self {
        Self {
            features,
            input: Input::new(),
            terminal: None,
            state: State::empty(),
        }
    }
    
    pub fn init(&mut self) -> Result<()> {
        if !self.state.contains(State::INITIALIZED) {
            self.state.insert(State::INITIALIZED);
            self.features.init()?;
            let backend = CrosstermBackend::new(stdout());
            let terminal = Terminal::new(backend)?;
            self.terminal = Some(terminal);
            set_panic_hook(self.features);
        }
        Ok(())
    }

    pub fn restore(&mut self) -> Result<()> {
        if self.state.contains(State::INITIALIZED) {
            self.state.remove(State::INITIALIZED);
            self.input.set_polling(false);
            self.features.restore()?;
            self.terminal = None;
        }
        Ok(())
    }

    pub fn pause(&mut self) -> Result<()> {
        if !self.state.contains(State::PAUSED) {
            self.state.insert(State::PAUSED);
            self.input.set_polling(false);
            self.features.restore()?;
        }
        Ok(())
    }

    pub fn resume(&mut self) -> Result<()> {
        if self.state.contains(State::PAUSED) {
            self.state.remove(State::PAUSED);
            self.features.init()?;
            self.input.set_polling(true);
            if let Some(terminal) = &mut self.terminal {
                terminal.clear()?;

            }
        }
        Ok(())
    }

}

fn set_panic_hook(features: Features) {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = features.restore();
        hook(panic_info);
    }));
}
