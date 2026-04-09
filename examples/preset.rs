use anyhow::Result;
use ratatui::Frame;
use ratatui::layout::Rect;
use gusto::app::AppBuilder;
use gusto::{Component, EventCtx, IEvent, IMsg};
use gusto::helper::KeyHelper;

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = AppBuilder::new(Box::new(TestComponent)).fps(120, 120, true).build();
    app.run()?;
    Ok(())
}

struct TestComponent;

impl Component<E, M> for TestComponent {
    fn on(&mut self, event: &IEvent<E>, ctx: &mut EventCtx<M>) -> Result<()> {
        match event {
            IEvent::CT(ct) => {
                if let Some(e) = ct.as_key_event() {
                    if e.is_n_fn(5) {
                        let _ = ctx.sender().send(IMsg::Tick);
                    } else if e.is_c_char('c') {
                        let _ = ctx.sender().send(IMsg::Exit);
                    } else if e.is_n_enter() {
                        let _ = ctx.sender().send(IMsg::None);
                    }
                }
            }
            IEvent::Initialize => {}
            IEvent::Exit => {}
            IEvent::Any(_) => {}
            IEvent::Custom(_) => {}
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame, rect: Rect) -> Result<()> {
        Ok(())
    }
}

#[derive(Clone)]
struct M;

#[derive(Clone)]
struct E;
