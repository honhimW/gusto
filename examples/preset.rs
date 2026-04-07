use anyhow::Result;
use ratatui::Frame;
use ratatui::layout::Rect;
use gusto::app::AppBuilder;
use gusto::{Component, EventCtx, IEvent};

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = AppBuilder::new(Box::new(TestComponent)).build();
    app.run().await?;
    Ok(())
}

struct TestComponent;

impl Component<E, M> for TestComponent {
    fn on(&mut self, event: &IEvent<E>, ctx: &mut EventCtx<M>) -> Result<()> {
        todo!()
    }

    fn render(&mut self, frame: &mut Frame, rect: Rect) -> Result<()> {
        todo!()
    }
}

#[derive(Clone)]
struct M;

#[derive(Clone)]
struct E;
