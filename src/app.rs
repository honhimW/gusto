use crate::input::InputEvent;
use crate::tui::{Features, Tui};
use crate::{Component, CtEvent, EventCtx, IEvent, IMsg};
use anyhow::Result;
use crossbeam_channel::{select, Receiver, Sender, TryRecvError};
use log::trace;
use ratatui::crossterm::event::Event;
use ratatui::layout::Rect;
use ratatui::Frame;
use std::time::Duration;
use crate::helper::KeyHelper;

pub struct App<E, M>
where
    E: Clone,
    M: Clone,
{
    msg_handler: Box<dyn MsgHandler<E, M>>,

    context: Context<M>,
    component: Root<E, M>,

    fps: (u8, u8, bool),
}

pub struct AppBuilder<E, M>
where
    E: Clone,
    M: Clone,
{
    component: Box<dyn Component<E, M>>,
    features: Features,
    // limitation(maximum) & minimal & visibleness
    fps: (u8, u8, bool),
    message_handler: Box<dyn MsgHandler<E, M>>,
}

impl<E, M> AppBuilder<E, M>
where
    E: Clone,
    M: Clone,
{
    pub fn new(component: Box<dyn Component<E, M>>) -> Self {
        Self {
            component,
            features: Features::default(),
            fps: (30, 1, false),
            message_handler: Box::new(DefaultMsgHandler),
        }
    }

    pub fn features(mut self, features: Features) -> Self {
        self.features = features;
        self
    }

    pub fn fps(mut self, maximum: u8, minimal: u8, visibility: bool) -> Self {
        self.fps = (maximum, minimal, visibility);
        self
    }

    pub fn message_handler(mut self, handler: Box<dyn MsgHandler<E, M>>) -> Self {
        self.message_handler = handler;
        self
    }

    pub fn build(self) -> App<E, M> {
        let interval = crossbeam_channel::tick(delay_duration(self.fps.0 as usize));
        App {
            msg_handler: self.message_handler,
            context: Context {
                tui: Tui::new(self.features),
                state: AppState::Preparing,
                msg_channel: crossbeam_channel::unbounded(),
                pause_reason: None,
                fps_state: if self.fps.2 { Some(crate::fps::FpsState::default()) } else { None },
                interval,
            },
            component: Root { inner: Some(self.component) },
            fps: self.fps,
        }
    }
}

impl<E, M> App<E, M>
where
    E: Clone,
    M: Clone,
{
    pub fn state(&self) -> AppState {
        self.context.state
    }

    pub fn health(&self) -> bool {
        self.context.state == AppState::Running
    }

    pub fn run(&mut self) -> Result<()> {
        self.prepare()?;
        self.looping()?;
        self.close()?;
        Ok(())
    }

    pub fn prepare(&mut self) -> Result<()> {
        if self.context.state == AppState::Preparing {
            self.context.tui.init()?;
            self.component.on(&IEvent::Initialize, &mut EventCtx::new(self.context.msg_channel.0.clone()))?;
        }
        self.context.state = AppState::Running;
        Ok(())
    }

    pub fn looping(&mut self) -> Result<()> {
        let _ = self.context.msg_channel.0.send(IMsg::Tick);
        let minimal_ticker = crossbeam_channel::tick(delay_duration(self.fps.1 as usize));
        let update_ticker = crossbeam_channel::bounded::<bool>(1);
        while self.health() {
            let _ = self.handle_input_loop(&self.context.tui.input.receiver());
            let mut messages = self.msg_handler.poll(&self.context.msg_channel.1);
            if !messages.is_empty() {
                messages.retain_mut(|m| !matches!(m, IMsg::Tick));
                for msg in messages {
                    let _ = self.msg_handler.handle(&mut self.context, &mut self.component, msg);
                }
                let _ = update_ticker.0.send(true);
            }

            let redraw = select! {
                recv(minimal_ticker) -> _ => true,
                recv(update_ticker.1) -> _ => true,
                default => false,
            };
            if redraw {
                let _ = self.msg_handler.handle(&mut self.context, &mut self.component, IMsg::Tick);
            }
        }
        Ok(())
    }

    pub fn take_pause_reason(&mut self) -> Option<M> {
        self.context.pause_reason.take()
    }

    pub fn resume(&mut self, custom_event: Option<E>) -> Result<()> {
        self.context.state = AppState::Running;
        self.context.tui.resume()?;
        if let Some(ce) = custom_event {
            self.component.on(&IEvent::Custom(ce), &mut EventCtx::new(self.context.msg_channel.0.clone()))?;
        }
        Ok(())
    }

    pub fn close(&mut self) -> Result<()> {
        if let Err(e) = self.context.tui.restore() {
            eprintln!(
                "failed to restore terminal. Run `reset` or restart your terminal to recover: {}",
                e,
            );
        }
        self.context.state = AppState::Closed;
        Ok(())
    }

    fn handle_input_loop(
        &mut self,
        input_channel: &Receiver<InputEvent>,
    ) -> Result<()> {
        let mut input_event_counter = 0;
        loop {
            if let Ok(input_event) = input_channel.try_recv()
                && let InputEvent::Input(input) = input_event
            {
                if let Event::Key(key) = &input
                    && !key.is_press()
                {
                    input_event_counter += 1;
                    continue;
                }

                #[cfg(debug_assertions)]
                trace!("Input: {:?}", input);
                let mut ctx = EventCtx::new(self.context.msg_channel.0.clone());
                let _ = self.component.on(&IEvent::CT(input), &mut ctx);
            } else {
                break;
            }
            input_event_counter += 1;
            if input_event_counter > 50 {
                break;
            }
        }
        Ok(())
    }
}

impl<E, M> Drop for App<E, M>
where
    E: Clone,
    M: Clone,
{
    fn drop(&mut self) {
        let _ = self.close();
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub enum AppState {
    Preparing,
    Running,
    Pausing,
    Closing,
    Closed,
}

fn delay_duration(fps: usize) -> Duration {
    Duration::from_secs_f32(1f32 / fps as f32)
}

pub struct Root<E, M> {
    inner: Option<Box<dyn Component<E, M>>>,
}

pub struct Context<M> {
    pub state: AppState,
    pub msg_channel: (Sender<IMsg<M>>, Receiver<IMsg<M>>),
    pub tui: Tui,
    pause_reason: Option<M>,
    fps_state: Option<crate::fps::FpsState>,
    interval: Receiver<std::time::Instant>,
}

impl<E, M> Component<E, M> for Root<E, M> {
    fn on(&mut self, event: &IEvent<E>, ctx: &mut EventCtx<M>) -> Result<()> {
        if let IEvent::CT(ct) = event && let CtEvent::Key(key) = ct && key.is_c_char('q') {
            let _ = ctx.sender.send(IMsg::Exit);
            return Ok(());
        }
        match &mut self.inner {
            Some(root) => root.on(&event, ctx),
            None => Ok(()),
        }
    }

    fn render(&mut self, frame: &mut Frame, rect: Rect) -> Result<()> {
        match &mut self.inner {
            Some(root) => root.render(frame, rect),
            None => Ok(()),
        }
    }
}

pub trait MsgHandler<E, M> {
    fn poll(&self, rx: &Receiver<IMsg<M>>) -> Vec<IMsg<M>> {
        let mut vec = vec![];
        for _ in 0..20 {
            match rx.try_recv() {
                Ok(msg) => vec.push(msg),
                Err(_) => break,
            }
        }
        vec
    }
    fn handle(&mut self, ctx: &mut Context<M>, component: &mut Root<E, M>, m: IMsg<M>) -> Result<()>;
}

pub struct DefaultMsgHandler;

impl<E, M> MsgHandler<E, M> for DefaultMsgHandler
where
    E: Clone,
    M: Clone,
{
    fn handle(&mut self, ctx: &mut Context<M>, component: &mut Root<E, M>, msg: IMsg<M>) -> Result<()> {
        match msg {
            IMsg::Exit => {
                ctx.state = AppState::Closing;
                let _ = component.on(&IEvent::Exit, &mut EventCtx::new(ctx.msg_channel.0.clone()));
            }
            IMsg::Tick => {
                let _ = ctx.interval.recv();
                if let Some(terminal) = &mut ctx.tui.terminal {
                    let _ = terminal.draw(|f| {
                        if let Some(fps_state) = &mut ctx.fps_state {
                            let rect = f.area();
                            let rect = Rect {
                                x: rect.x + rect.width.saturating_sub(9),
                                y: rect.y,
                                width: rect.width.min(9),
                                height: rect.height.min(1),
                            };
                            f.render_stateful_widget(crate::fps::Fps, rect, fps_state);
                        }
                        let _ = component.render(f, f.area());
                    });
                }
            }
            IMsg::Pause(m) => {
                ctx.state = AppState::Pausing;
                ctx.pause_reason = Some(m);
                ctx.tui.pause()?;
            }
            _ => {}
        }
        Ok(())
    }
}
