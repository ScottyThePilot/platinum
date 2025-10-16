use femtovg::renderer::OpenGl;
use femtovg::Canvas;
use glutin::surface::{Surface, SurfaceAttributesBuilder, WindowSurface};
use glutin_winit::{ApiPreference, DisplayBuilder, GlWindow};
use glutin::config::{Config, ConfigTemplateBuilder};
use glutin::context::{ContextApi, ContextAttributesBuilder, NotCurrentContext, PossiblyCurrentContext, Version};
use glutin::display::{Display, GetGlDisplay};
use glutin::prelude::*;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{DeviceEvent, DeviceId, ElementState, Ime, KeyEvent, MouseButton, MouseScrollDelta, Touch};
use winit::event_loop::{EventLoop, EventLoopBuilder};
use winit::raw_window_handle::HasWindowHandle;
use winit::window::{Window, WindowAttributes};

use crate::windowing::{Application, AxisMotion, EventHandler, Gesture, WindowState};

use std::fmt;
use std::num::NonZero;
use std::path::PathBuf;
use std::rc::Rc;



pub fn default_gl_config_picker(configs: Box<dyn Iterator<Item = Config> + '_>) -> Config {
  configs
    .reduce(|prev_config, config| {
      let transparency_check = config.supports_transparency().unwrap_or(false)
        && !prev_config.supports_transparency().unwrap_or(false);

      if transparency_check || config.num_samples() > prev_config.num_samples() {
        config
      } else {
        prev_config
      }
    })
    .unwrap()
}

pub type WindowRef = Rc<Window>;

pub struct EngineBuilder<T: 'static = ()> {
  event_loop_builder: EventLoopBuilder<T>,
  window_attributes: Option<WindowAttributes>,
  gl_api_preference: ApiPreference,
  gl_config_template_builder: ConfigTemplateBuilder,
  gl_config_picker: fn(Box<dyn Iterator<Item = Config> + '_>) -> Config
}

impl EngineBuilder {
  pub fn new_without_user_event() -> Self {
    Self::default()
  }
}

impl<T: 'static> EngineBuilder<T> {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn with_event_loop_builder(mut self, operate: impl FnOnce(&mut EventLoopBuilder<T>) -> &mut EventLoopBuilder<T>) -> Self {
    operate(&mut self.event_loop_builder);
    self
  }

  pub fn with_window_attributes(mut self, window_attributes: Option<WindowAttributes>) -> Self {
    self.window_attributes = window_attributes;
    self
  }

  pub fn with_gl_api_preference(mut self, gl_api_preference: ApiPreference) -> Self {
    self.gl_api_preference = gl_api_preference;
    self
  }

  pub fn with_gl_config_template_builder(mut self, operate: impl FnOnce(ConfigTemplateBuilder) -> ConfigTemplateBuilder) -> Self {
    self.gl_config_template_builder = operate(self.gl_config_template_builder);
    self
  }

  pub fn with_gl_config_picker(mut self, gl_config_picker: fn(Box<dyn Iterator<Item = Config> + '_>) -> Config) -> Self {
    self.gl_config_picker = gl_config_picker;
    self
  }

  pub fn create<H: EngineEventHandler<T>>(self, handler: H) -> Engine<H, T> {
    let (event_loop, current_gl_context, gl_display, gl_window_surface, window) = self.build_parts();
    let window = Rc::new(window);
    let canvas = create_canvas(&gl_display);

    Engine {
      event_loop,
      application: Application::new(
        window.clone(),
        EngineHandlerWrapper {
          current_gl_context,
          gl_display,
          gl_window_surface,
          canvas,
          handler
        }
      ),
      window
    }
  }

  fn build_parts(mut self) -> (EventLoop<T>, PossiblyCurrentContext, Display, Surface<WindowSurface>, Window) {
    let event_loop = self.event_loop_builder.build()
      .expect("failed to build event loop");

    let gl_config_template_builder = self.gl_config_template_builder.with_alpha_size(8);
    let gl_display_builder = DisplayBuilder::new()
      .with_preference(self.gl_api_preference)
      .with_window_attributes(self.window_attributes);
    let (window, gl_config) = gl_display_builder
      .build(&event_loop, gl_config_template_builder, self.gl_config_picker)
      .expect("failed to build display");

    let window = window.expect("display builder produced no window");

    let not_current_gl_context = create_gl_context(&window, &gl_config);

    let gl_surface_attributes = window.build_surface_attributes(SurfaceAttributesBuilder::default())
      .expect("failed to build window surface attributes");

    let gl_display = gl_config.display();

    let gl_window_surface = unsafe {
      gl_display.create_window_surface(&gl_config, &gl_surface_attributes)
        .expect("failed to create opengl window surface")
    };

    let current_gl_context = not_current_gl_context.make_current(&gl_window_surface)
      .expect("failed to make opengl context current");

    (event_loop, current_gl_context, gl_display, gl_window_surface, window)
  }
}

impl<T: 'static> Default for EngineBuilder<T> {
  fn default() -> Self {
    EngineBuilder {
      event_loop_builder: EventLoop::with_user_event(),
      window_attributes: None,
      gl_api_preference: ApiPreference::default(),
      gl_config_template_builder: ConfigTemplateBuilder::new(),
      gl_config_picker: default_gl_config_picker
    }
  }
}

fn create_canvas(gl_display: &Display) -> EngineCanvas {
  let renderer = unsafe {
    OpenGl::new_from_function_cstr(|s| gl_display.get_proc_address(s).cast())
      .expect("failed to create femtovg opengl renderer")
  };

  let canvas = Canvas::new(renderer)
    .expect("failed to create femtovg canvas");

  canvas
}

fn create_gl_context(window: &Window, gl_config: &Config) -> NotCurrentContext {
  let raw_window_handle = window.window_handle()
    .expect("could not get window handle from window")
    .as_raw();

  let gl_context_attributes = ContextAttributesBuilder::new()
    .build(Some(raw_window_handle));

  let fallback_gl_context_attributes = ContextAttributesBuilder::new()
    .with_context_api(ContextApi::Gles(None))
    .build(Some(raw_window_handle));

  let legacy_gl_context_attributes = ContextAttributesBuilder::new()
    .with_context_api(ContextApi::OpenGl(Some(Version::new(2, 1))))
    .build(Some(raw_window_handle));

  let gl_display = gl_config.display();

  let not_current_gl_context = unsafe {
    gl_display.create_context(&gl_config, &gl_context_attributes)
      .or_else(|_| gl_display.create_context(&gl_config, &fallback_gl_context_attributes))
      .or_else(|_| gl_display.create_context(&gl_config, &legacy_gl_context_attributes))
      .expect("failed to create opengl context")
  };

  not_current_gl_context
}

impl<T: 'static> fmt::Debug for EngineBuilder<T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("EngineBuilder")
      .field("event_loop_builder", &format_args!("EventLoopBuilder"))
      .field("window_attributes", &self.window_attributes)
      .field("gl_api_preference", &self.gl_api_preference)
      .field("gl_config_template_builder", &self.gl_config_template_builder)
      .field("gl_config_picker", &self.gl_config_picker)
      .finish()
  }
}



#[derive(Debug)]
pub struct Engine<H: EngineEventHandler<T>, T: 'static = ()> {
  event_loop: EventLoop<T>,
  application: Application<WindowRef, EngineHandlerWrapper<H>, T>,
  window: WindowRef
}

impl<H: EngineEventHandler<T>, T: 'static> Engine<H, T> {
  pub fn window(&self) -> &WindowRef {
    &self.window
  }

  pub fn event_loop(&self) -> &EventLoop<T> {
    &self.event_loop
  }

  pub fn run(self) {
    let Engine { event_loop, mut application, window } = self;
    event_loop.run_app(&mut application)
      .expect("failed to run event loop");
    drop(application);
    drop(window);
  }
}



struct EngineHandlerWrapper<H> {
  current_gl_context: PossiblyCurrentContext,
  #[allow(unused)]
  gl_display: Display,
  gl_window_surface: Surface<WindowSurface>,
  canvas: EngineCanvas,
  handler: H
}

impl<H: EngineEventHandler<T>, T: 'static> EventHandler<WindowRef, T> for EngineHandlerWrapper<H> {
  delegate!(handler: fn init(&mut self, window_state: &EngineWindowState));
  delegate!(handler: fn update(&mut self, window_state: &EngineWindowState));

  fn render(&mut self, window_state: &EngineWindowState) {
    let window = window_state.window();
    let size = window.inner_size();
    self.canvas.set_size(size.width, size.height, window.scale_factor() as f32);
    self.handler.render(window_state, &mut self.canvas);
    self.canvas.flush();
    window.pre_present_notify();
    self.gl_window_surface.swap_buffers(&self.current_gl_context)
      .expect("failed to swap opengl window surface buffers");
  }

  fn on_resized(&mut self, window_state: &EngineWindowState, window_size: PhysicalSize<u32>, scale_factor: f64) {
    let PhysicalSize { width, height } = window_size;
    if let Some(width) = NonZero::new(width) && let Some(height) = NonZero::new(height) {
      self.gl_window_surface.resize(&self.current_gl_context, width, height);
    };

    self.handler.on_resized(window_state, window_size, scale_factor);
  }

  delegate!(handler: fn on_user_event(&mut self, window_state: &EngineWindowState, event: T));
  delegate!(handler: fn on_device_event(&mut self, window_state: &EngineWindowState, id: DeviceId, event: DeviceEvent));
  delegate!(handler: fn on_keyboard_input(&mut self, window_state: &EngineWindowState, event: KeyEvent));
  delegate!(handler: fn on_text_input(&mut self, window_state: &EngineWindowState, event: Ime));
  delegate!(handler: fn on_cursor_moved(&mut self, window_state: &EngineWindowState, pos: PhysicalPosition<f32>));
  delegate!(handler: fn on_mouse_input(&mut self, window_state: &EngineWindowState, state: ElementState, button: MouseButton));
  delegate!(handler: fn on_mouse_scroll(&mut self, window_state: &EngineWindowState, delta: MouseScrollDelta));
  delegate!(handler: fn on_gesture(&mut self, window_state: &EngineWindowState, gesture: Gesture));
  delegate!(handler: fn on_touch(&mut self, window_state: &EngineWindowState, touch: Touch));
  delegate!(handler: fn on_axis_motion(&mut self, window_state: &EngineWindowState, axis_motion: AxisMotion));
  delegate!(handler: fn on_focus_changed(&mut self, window_state: &EngineWindowState, state: bool));
  delegate!(handler: fn on_occlusion_changed(&mut self, window_state: &EngineWindowState, state: bool));
  delegate!(handler: fn on_file_over(&mut self, window_state: &EngineWindowState, path: Option<PathBuf>, dropped: bool));
  delegate!(handler: fn on_resumed(&mut self, window_state: &EngineWindowState));
  delegate!(handler: fn on_suspended(&mut self, window_state: &EngineWindowState));
  delegate!(handler: fn on_close_requested(&mut self, window_state: &EngineWindowState) -> bool);
  delegate!(handler: fn should_exit(&self, window_state: &EngineWindowState) -> bool);
  delegate!(handler: fn on_exited(self));
}

impl<H: fmt::Debug> fmt::Debug for EngineHandlerWrapper<H> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("EngineHandlerWrapper")
      .field("current_gl_context", &self.current_gl_context)
      .field("gl_display", &self.gl_display)
      .field("gl_window_surface", &self.gl_window_surface)
      .field("canvas", &format_args!("Canvas"))
      .field("handler", &self.handler)
      .finish()
  }
}

pub type EngineCanvas = Canvas<OpenGl>;
pub type EngineWindowState = WindowState<WindowRef>;

#[allow(unused_variables)]
pub trait EngineEventHandler<T: 'static = ()>: Sized + 'static {
  /// See [`EventHandler::init`].
  fn init(&mut self, window_state: &EngineWindowState) {}
  /// See [`EventHandler::update`].
  fn update(&mut self, window_state: &EngineWindowState);
  /// See [`EventHandler::render`].
  fn render(&mut self, window_state: &EngineWindowState, canvas: &mut EngineCanvas);
  /// See [`EventHandler::on_user_event`].
  fn on_user_event(&mut self, window_state: &EngineWindowState, event: T) {}
  /// See [`EventHandler::on_device_event`].
  fn on_device_event(&mut self, window_state: &EngineWindowState, id: DeviceId, event: DeviceEvent) {}
  /// See [`EventHandler::on_keyboard_input`].
  fn on_keyboard_input(&mut self, window_state: &EngineWindowState, event: KeyEvent) {}
  /// See [`EventHandler::on_text_input`].
  fn on_text_input(&mut self, window_state: &EngineWindowState, event: Ime) {}
  /// See [`EventHandler::on_cursor_moved`].
  fn on_cursor_moved(&mut self, window_state: &EngineWindowState, pos: PhysicalPosition<f32>) {}
  /// See [`EventHandler::on_mouse_input`].
  fn on_mouse_input(&mut self, window_state: &EngineWindowState, state: ElementState, button: MouseButton) {}
  /// See [`EventHandler::on_mouse_scroll`].
  fn on_mouse_scroll(&mut self, window_state: &EngineWindowState, delta: MouseScrollDelta) {}
  /// See [`EventHandler::on_gesture`].
  fn on_gesture(&mut self, window_state: &EngineWindowState, gesture: Gesture) {}
  /// See [`EventHandler::on_touch`].
  fn on_touch(&mut self, window_state: &EngineWindowState, touch: Touch) {}
  /// See [`EventHandler::on_axis_motion`].
  fn on_axis_motion(&mut self, window_state: &EngineWindowState, axis_motion: AxisMotion) {}
  /// See [`EventHandler::on_focus_changed`].
  fn on_focus_changed(&mut self, window_state: &EngineWindowState, state: bool) {}
  /// See [`EventHandler::on_occlusion_changed`].
  fn on_occlusion_changed(&mut self, window_state: &EngineWindowState, state: bool) {}
  /// See [`EventHandler::on_file_dropped`].
  fn on_file_over(&mut self, window_state: &EngineWindowState, path: Option<PathBuf>, dropped: bool) {}
  /// See [`EventHandler::on_resized`].
  fn on_resized(&mut self, window_state: &EngineWindowState, window_size: PhysicalSize<u32>, scale_factor: f64) {}
  /// See [`EventHandler::on_resumed`].
  fn on_resumed(&mut self, window_state: &EngineWindowState) {}
  /// See [`EventHandler::on_suspended`].
  fn on_suspended(&mut self, window_state: &EngineWindowState) {}
  /// See [`EventHandler::on_close_requested`].
  fn on_close_requested(&mut self, window_state: &EngineWindowState) -> bool { true }
  /// See [`EventHandler::should_exit`].
  fn should_exit(&self, window_state: &EngineWindowState) -> bool { false }
  /// See [`EventHandler::on_exited`].
  fn on_exited(self) {}
}
