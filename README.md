# gusto

lightweight ratatui framework

### Quick start

```rust
async fn main() -> Result<()> {
    let mut app = AppBuilder::new(Box::new(TestComponent)).build();
    app.run()?;
    Ok(())
}

struct TestComponent;

impl Component<E, M> for TestComponent {
    fn on(&mut self, event: &IEvent<E>, ctx: &mut EventCtx<M>) -> Result<()> { todo!() }

    fn render(&mut self, frame: &mut Frame, rect: Rect) -> Result<()> { todo!() }
}

#[derive(Clone)]
struct M;

#[derive(Clone)]
struct E;
```

### Examples

- [open system editor](./examples/open_system_editor.rs)
