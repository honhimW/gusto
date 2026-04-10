use anyhow::Result;
use ratatui::crossterm::event::KeyCode;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::text::Text;
use gusto::app::{App, AppBuilder, AppState, Context, DefaultMsgHandler, Root, MsgHandler};
use gusto::{Component, EventCtx, IEvent, IMsg};
use gusto::helper::MouseHelper;
use gusto::tui::Features;

fn main() -> Result<()> {
    let mut app = AppBuilder::new(Box::new(TestComponent {
        content: "ab".to_string(),
    }))
        .fps(120, 10, true)
        .features(Features::all())
        .message_handler(Box::new(ExtendMsgHandler {counter: 0}))
        .build();
    app.prepare()?;

    while app.health() {
        app.looping()?;
        if app.state() == AppState::Pausing {
            let reason = app.take_pause_reason();
            if let Some(reason) = reason {
                let result = edit::edit_with_builder(reason.0, &edit::Builder::new().suffix(".json"))?;
                app.resume(Some(E(result)))?;
            } else {
                app.resume(None)?;
            }
        }
    }
    app.close()?;
    Ok(())
}

struct TestComponent {
    content: String,
}

#[derive(Clone)]
struct E(String);

#[derive(Clone)]
struct M(String);

impl Component<E, M> for TestComponent {
    fn on(&mut self, event: &IEvent<E>, ctx: &mut EventCtx<M>) -> anyhow::Result<()> {
        if let IEvent::CT(ct) = event && let Some(k) = ct.as_key_press_event() {
            if let KeyCode::Char(c) = k.code && c == 'e' {
                ctx.sender().send(IMsg::Pause(M(self.content.clone())))?;
                ctx.consumed();
            }
            if let KeyCode::F(5) = k.code {
                ctx.consumed();
            }
        }
        if let IEvent::CT(ct) = event && let Some(m) = ct.as_mouse_event() && m.is_n_left_down() {
            ctx.consumed();
        }
        if let IEvent::Custom(e) = event {
            self.content = e.0.clone();
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame, rect: Rect) -> anyhow::Result<()> {
        frame.render_widget(Text::raw(&self.content), rect);
        Ok(())
    }
}

struct ExtendMsgHandler {
    counter: usize,
}

impl MsgHandler<E, M> for ExtendMsgHandler
{
    fn handle(&mut self, ctx: &mut Context<M>, component: &mut Root<E, M>, m: IMsg<M>) -> Result<()> {
        self.counter = self.counter.saturating_add(1);
        DefaultMsgHandler.handle(ctx, component, m)
    }
}
