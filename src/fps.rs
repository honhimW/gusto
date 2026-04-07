use std::time::{Duration, Instant};
use ratatui::widgets::StatefulWidget;
use ratatui::text::Text;
use ratatui::widgets::Widget;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;

#[derive(Debug, Clone)]
pub struct Fps;

#[derive(Debug, Clone)]
pub struct FpsState {
    /// The number of elapsed frames that have passed - used to calculate the fps
    frame_count: usize,

    /// The last instant that the fps was calculated
    last_instant: Instant,

    /// The current frames per second
    pub fps: f32,
}

/// Default impl for `FpsWidget`
///
/// Manual impl is required because we need to initialize the `last_instant` field to the current
/// instant.
impl Default for FpsState {
    fn default() -> Self {
        Self {
            frame_count: 0,
            last_instant: Instant::now(),
            fps: 0f32,
        }
    }
}

/// Widget impl for `FpsWidget`
///
/// This is implemented on a mutable reference so that we can update the frame count and fps
/// calculation while rendering.
impl Widget for &mut FpsState {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.calculate_fps();
        let fps = self.fps;
        let text = format!("{fps:.1} fps");
        Text::from(text).render(area, buf);
    }
}

impl StatefulWidget for Fps {
    type State = FpsState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        state.calculate_fps();
        let fps = state.fps;
        let text = format!("{fps:.1} fps");
        Text::from(text).right_aligned().render(area, buf);
    }
}

impl FpsState {
    /// Update the fps calculation.
    ///
    /// This updates the fps once a second, but only if the widget has rendered at least 2 frames
    /// since the last calculation. This avoids noise in the fps calculation when rendering on slow
    /// machines that can't render at least 2 frames per second.
    #[allow(clippy::cast_precision_loss)]
    pub fn calculate_fps(&mut self) {
        self.frame_count += 1;
        let elapsed = self.last_instant.elapsed();
        if elapsed > Duration::from_secs(1) && self.frame_count > 2 {
            self.fps = self.frame_count as f32 / elapsed.as_secs_f32();
            self.frame_count = 0;
            self.last_instant = Instant::now();
        }
    }
}
