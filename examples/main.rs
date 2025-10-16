extern crate platinum;

use std::time::{Duration, Instant};

use platinum::engine::{EngineBuilder, EngineCanvas, EngineEventHandler, EngineWindowState};
use platinum::femtovg::{Color, Paint, Path};
use platinum::winit::window::Window;
use platinum::winit::dpi::PhysicalSize;
use platinum::winit::window::Theme;



fn main() {
  let window_attributes = Window::default_attributes()
    .with_inner_size(PhysicalSize::new(1280, 720))
    .with_theme(Some(Theme::Dark));

  EngineBuilder::new_without_user_event()
    .with_window_attributes(Some(window_attributes))
    .create(Handler { last_update: None, i: 0.0 }).run();
}

struct Handler {
  last_update: Option<Instant>,
  i: f32
}

impl Handler {
  pub fn swap_delta_time(&mut self, now: Instant) -> Duration {
    self.last_update.replace(now)
      .map_or(Duration::ZERO, |last_update| now.duration_since(last_update))
  }
}

impl EngineEventHandler for Handler {
  fn update(&mut self, window_state: &EngineWindowState) {
    let dt = self.swap_delta_time(Instant::now()).as_secs_f32();
    let (width, height) = window_state.window_size().into();
    let p = u32::min(width, height);

    self.i += dt * 10.0;
    self.i %= p as f32;
  }

  fn render(&mut self, window_state: &EngineWindowState, canvas: &mut EngineCanvas) {
    let (width, height) = window_state.window_size().into();
    let p = u32::min(width, height);

    canvas.clear_rect(0, 0, width, height, Color::rgb(77, 22, 88));
    canvas.clear_rect(32, 32, width - 64, height - 64, Color::rgb(22, 33, 44));

    let mut path = Path::new();
    let paint = Paint::color(Color::rgb(63, 127, 255));
    for i in [self.i as f32, self.i as f32 - p as f32] {
      path.rect(i, i, 128.0, 128.0);
    };
    canvas.fill_path(&mut path, &paint);
  }
}
