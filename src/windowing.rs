use ahash::AHashSet;
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};
use winit::application::ApplicationHandler;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::error::EventLoopError;
use winit::event::{
  AxisId, DeviceEvent, DeviceId, ElementState, Ime, KeyEvent, Modifiers, MouseButton, MouseScrollDelta, StartCause, Touch, TouchPhase, WindowEvent
};
use winit::keyboard::{Key as LogicalKey, NamedKey, PhysicalKey, KeyCode};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Theme, Window, WindowId};

use std::marker::PhantomData;
use std::mem::replace;
use std::ops::Index;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;



/// Handles events emitted by a winit event loop.
#[allow(unused_variables)]
pub trait EventHandler<W: HasWindow, T = ()>: Sized + 'static {
  /// Called upon only the first [`Event::Resumed`][winit::event::Event::Resumed].
  fn init(&mut self, window_state: &WindowState<W>) {}

  /// Called upon [`Event::AboutToWait`][winit::event::Event::AboutToWait].
  fn update(&mut self, window_state: &WindowState<W>);

  /// Called upon [`WindowEvent::RedrawRequested`].
  fn render(&mut self, window_state: &WindowState<W>);

  /// Called when an event is sent from [`EventLoopProxy::send_event`][winit::event_loop::EventLoopProxy::send_event].
  fn on_user_event(&mut self, window_state: &WindowState<W>, event: T) {}

  /// Called upon [`Event::WindowEvent`][winit::event::Event::WindowEvent].
  /// By default, this calls [`WindowState::handle_window_event`] which properly delegates
  /// events from [`WindowEvent`] to their respective handler functions.
  ///
  /// When overriding this function, ensure that [`WindowState::handle_window_event`] is called,
  /// otherwise this [`EventHandler`] will stop working.
  fn on_window_event(&mut self, window_state: &mut WindowState<W>, event: WindowEvent, event_loop: &ActiveEventLoop) {
    window_state.handle_window_event(self, event, event_loop);
  }

  /// Called upon [`Event::DeviceEvent`][winit::event::Event::DeviceEvent].
  fn on_device_event(&mut self, window_state: &WindowState<W>, id: DeviceId, event: DeviceEvent) {}

  /// Called when an event from the keyboard has been received.
  fn on_keyboard_input(&mut self, window_state: &WindowState<W>, event: KeyEvent) {}

  /// Called upon [`WindowEvent::Ime`].
  fn on_text_input(&mut self, window_state: &WindowState<W>, event: Ime) {}

  /// Called when the cursor has moved on the window.
  fn on_cursor_moved(&mut self, window_state: &WindowState<W>, pos: PhysicalPosition<f32>) {}

  /// Called when a mouse button press has been received.
  fn on_mouse_input(&mut self, window_state: &WindowState<W>, state: ElementState, button: MouseButton) {}

  /// Called when a mouse wheel or touchpad scroll occurs.
  fn on_mouse_scroll(&mut self, window_state: &WindowState<W>, delta: MouseScrollDelta) {}

  /// Called upon the following events:
  /// - [`WindowEvent::PinchGesture`] (mapped to [`Gesture::Pinch`])
  /// - [`WindowEvent::PanGesture`] (mapped to [`Gesture::Pan`])
  /// - [`WindowEvent::DoubleTapGesture`] (mapped to [`Gesture::DoubleTap`])
  /// - [`WindowEvent::RotationGesture`] (mapped to [`Gesture::Rotation`])
  /// - [`WindowEvent::TouchpadPressure`] (mapped to [`Gesture::TouchpadPressure`])
  fn on_gesture(&mut self, window_state: &WindowState<W>, gesture: Gesture) {}

  /// Called upon [`WindowEvent::Touch`].
  fn on_touch(&mut self, window_state: &WindowState<W>, touch: Touch) {}

  /// Called upon [`WindowEvent::AxisMotion`].
  fn on_axis_motion(&mut self, window_state: &WindowState<W>, axis_motion: AxisMotion) {}

  /// Called when the application window loses or gains focus. See [WindowEvent::Focused].
  fn on_focus_changed(&mut self, window_state: &WindowState<W>, state: bool) {}

  /// Called when the application becomes entirely occluded or becomes no longer occluded. See [WindowEvent::Occluded],
  fn on_occlusion_changed(&mut self, window_state: &WindowState<W>, state: bool) {}

  /// Called when a file is dropped, hovered, or a file hover is cancelled in the application window.
  fn on_file_over(&mut self, window_state: &WindowState<W>, path: Option<PathBuf>, dropped: bool) {}

  /// Called when either the window has been resized or the scale factor has changed.
  fn on_resized(&mut self, window_state: &WindowState<W>, window_size: PhysicalSize<u32>, scale_factor: f64) {}

  /// Called upon [`Event::Resumed`][winit::event::Event::Resumed].
  fn on_resumed(&mut self, window_state: &WindowState<W>) {}

  /// Called upon [`Event::Suspended`][winit::event::Event::Suspended].
  fn on_suspended(&mut self, window_state: &WindowState<W>) {}

  /// Called when the user attempts to close the application.
  /// A return value of `true` closes the application, while `false` cancels closing it.
  /// Defaults to an 'always `true`' implementation.
  fn on_close_requested(&mut self, window_state: &WindowState<W>) -> bool { true }

  /// Instructs the event dispatcher whether the handler wants the application to exit.
  /// Defaults to an 'always `false`' implementation.
  fn should_exit(&self, window_state: &WindowState<W>) -> bool { false }

  /// Called once the event loop has been destroyed and will no longer dispatch any more events.
  /// This is different from the `close` function in that the handler has no choice over the application state.
  fn on_exited(self) {}
}

#[derive(Debug, Clone, PartialEq)]
pub struct InputState {
  cursor_pos: Option<PhysicalPosition<f32>>,
  cursor_pos_prev: Option<PhysicalPosition<f32>>,
  mouse_actions: Vec<MouseAction>,
  mouse_left_held: bool,
  mouse_right_held: bool,
  mouse_middle_held: bool,
  mouse_back_held: bool,
  mouse_forward_held: bool,
  has_cursor_not_moved: bool,
  key_actions: Vec<KeyAction>,
  keys_held_physical: AHashSet<KeyCode>,
  keys_held_logical: AHashSet<NamedKey>,
  modifiers: Modifiers,
  gestures: Vec<Gesture>,
  touches: Vec<Touch>,
  axis_motions: Vec<AxisMotion>
}

impl InputState {
  #[inline]
  pub fn cursor_pos(&self) -> Option<PhysicalPosition<f32>> {
    self.cursor_pos
  }

  #[inline]
  pub fn cursor_pos_prev(&self) -> Option<PhysicalPosition<f32>> {
    self.cursor_pos_prev
  }

  #[inline]
  pub fn cursor_pos_rel(&self) -> Option<PhysicalPosition<f32>> {
    if let (Some(pos), Some(pos_prev)) = (self.cursor_pos, self.cursor_pos_prev) {
      Some(PhysicalPosition {
        x: pos.x - pos_prev.x,
        y: pos.y - pos_prev.y
      })
    } else {
      None
    }
  }

  /// Returns a list of mouse button actions performed during the current frame.
  #[inline]
  pub fn mouse_actions(&self) -> &[MouseAction] {
    &self.mouse_actions
  }

  /// Checks whether or not the given mouse button is currently pressed.
  #[inline]
  pub fn is_button_held(&self, button: MouseButton) -> bool {
    self[button]
  }

  /// Returns a list of key actions performed during the current frame.
  #[inline]
  pub fn key_actions(&self) -> &[KeyAction] {
    &self.key_actions
  }

  /// Checks whether or not the given named key is currently pressed.
  #[inline]
  pub fn is_key_held_logical(&self, named_key: &NamedKey) -> bool {
    self.keys_held_logical.contains(named_key)
  }

  /// Checks whether or not the given key code is currently pressed.
  #[inline]
  pub fn is_key_held_physical(&self, key_code: &KeyCode) -> bool {
    self.keys_held_physical.contains(key_code)
  }

  /// Checks whether or not the given physical key was operated in the given method during the current frame.
  pub fn was_key_operated_physical(&self, physical_key: &PhysicalKey, state: KeyActionState) -> bool {
    self.key_actions.iter().find(|&action| action.is_physical(&physical_key, state)).is_some()
  }

  /// Checks whether or not the given logical key was operated in the given method during the current frame.
  pub fn was_key_operated_logical(&self, logical_key: &LogicalKey, state: KeyActionState) -> bool {
    self.key_actions.iter().find(|&action| action.is_logical(&logical_key, state)).is_some()
  }

  /// Checks whether or not the given physical key was pressed during the current frame.
  pub fn was_key_pressed_physical(&self, physical_key: &PhysicalKey) -> bool {
    self.was_key_operated_physical(physical_key, KeyActionState::Pressed)
  }

  /// Checks whether or not the given logical key was pressed during the current frame.
  pub fn was_key_pressed_logical(&self, logical_key: &LogicalKey) -> bool {
    self.was_key_operated_logical(logical_key, KeyActionState::Pressed)
  }

  /// Checks whether or not the given physical key was repeating during the current frame.
  pub fn was_key_repeating_physical(&self, physical_key: &PhysicalKey) -> bool {
    self.was_key_operated_physical(physical_key, KeyActionState::Repeating)
  }

  /// Checks whether or not the given logical key was repeating during the current frame.
  pub fn was_key_repeating_logical(&self, logical_key: &LogicalKey) -> bool {
    self.was_key_operated_logical(logical_key, KeyActionState::Repeating)
  }

  /// Checks whether or not the given physical key was released during the current frame.
  pub fn was_key_released_physical(&self, physical_key: &PhysicalKey) -> bool {
    self.was_key_operated_physical(physical_key, KeyActionState::Released)
  }

  /// Checks whether or not the given logical key was released during the current frame.
  pub fn was_key_released_logical(&self, logical_key: &LogicalKey) -> bool {
    self.was_key_operated_logical(logical_key, KeyActionState::Released)
  }

  /// Whether the mouse moved during the current frame.
  pub fn was_cursor_moving(&self) -> bool {
    self.cursor_pos != self.cursor_pos_prev
  }

  /// Whether the mouse has remained stationary since the mouse was pressed.
  /// This can be useful for filtering between user intention in the event you
  /// want to have different functionality between click + drag and click.
  pub fn has_cursor_not_moved(&self) -> bool {
    self.has_cursor_not_moved && !self.was_cursor_moving()
  }

  #[inline]
  pub fn modifiers(&self) -> Modifiers {
    self.modifiers
  }

  #[inline]
  pub fn gestures(&self) -> &[Gesture] {
    &self.gestures
  }

  #[inline]
  pub fn touches(&self) -> &[Touch] {
    &self.touches
  }

  #[inline]
  pub fn axis_motions(&self) -> &[AxisMotion] {
    &self.axis_motions
  }

  pub fn axis_motion_for(&self, axis: AxisId, device_id: Option<DeviceId>) -> f64 {
    if let Some(device_id) = device_id {
      self.axis_motions.iter()
        .filter(|axis_motion| axis_motion.device_id == device_id)
        .filter(|axis_motion| axis_motion.axis == axis)
        .map(|axis| axis.value)
        .sum::<f64>()
    } else {
      self.axis_motions.iter()
        .filter(|axis_motion| axis_motion.axis == axis)
        .map(|axis| axis.value)
        .sum::<f64>()
    }
  }

  pub fn was_axis_moving(&self, axis: AxisId, device_id: Option<DeviceId>) -> bool {
    if let Some(device_id) = device_id {
      self.axis_motions.iter()
        .filter(|axis_motion| axis_motion.device_id == device_id)
        .find(|axis_motion| axis_motion.axis == axis)
        .is_some()
    } else {
      self.axis_motions.iter()
        .find(|axis_motion| axis_motion.axis == axis)
        .is_some()
    }
  }

  fn set_button_value(&mut self, button: MouseButton, value: bool) {
    match button {
      MouseButton::Left => self.mouse_left_held = value,
      MouseButton::Right => self.mouse_right_held = value,
      MouseButton::Middle => self.mouse_middle_held = value,
      MouseButton::Back => self.mouse_back_held = value,
      MouseButton::Forward => self.mouse_forward_held = value,
      MouseButton::Other(_) => ()
    };
  }

  fn reset(&mut self) {
    self.cursor_pos_prev = self.cursor_pos;
    self.mouse_actions.clear();
    self.key_actions.clear();
    self.keys_held_physical.clear();
    self.keys_held_logical.clear();
    self.has_cursor_not_moved = false;
    self.gestures.clear();
    self.touches.clear();
    self.axis_motions.clear();
  }

  fn handle_keyboard_input(&mut self, event: &KeyEvent) {
    self.key_actions.push(KeyAction {
      physical_key: event.physical_key,
      logical_key: event.logical_key.clone(),
      state: match event.state {
        ElementState::Pressed if event.repeat => KeyActionState::Repeating,
        ElementState::Pressed => KeyActionState::Pressed,
        ElementState::Released => KeyActionState::Released
      }
    });

    if let PhysicalKey::Code(key_code) = event.physical_key {
      match event.state {
        ElementState::Pressed => self.keys_held_physical.insert(key_code),
        ElementState::Released => self.keys_held_physical.remove(&key_code)
      };
    };

    if let LogicalKey::Named(named_key) = event.logical_key {
      match event.state {
        ElementState::Pressed => self.keys_held_logical.insert(named_key),
        ElementState::Released => self.keys_held_logical.remove(&named_key)
      };
    };
  }

  fn handle_mouse_input(&mut self, state: ElementState, button: MouseButton) {
    let condition = state.is_pressed();

    self.set_button_value(button, condition);
    self.mouse_actions.push(MouseAction { button, state });
    self.has_cursor_not_moved = condition;
  }

  fn handle_gesture(&mut self, gesture: Gesture) {
    self.gestures.push(gesture);
  }

  fn handle_touch(&mut self, touch: Touch) {
    self.touches.push(touch);
  }

  fn handle_axis_motion(&mut self, axis_motion: AxisMotion) {
    self.axis_motions.push(axis_motion);
  }
}

impl Default for InputState {
  #[inline]
  fn default() -> Self {
    InputState {
      cursor_pos: None,
      cursor_pos_prev: None,
      mouse_actions: Vec::new(),
      mouse_left_held: false,
      mouse_right_held: false,
      mouse_middle_held: false,
      mouse_back_held: false,
      mouse_forward_held: false,
      has_cursor_not_moved: false,
      key_actions: Vec::new(),
      keys_held_physical: AHashSet::new(),
      keys_held_logical: AHashSet::new(),
      modifiers: Modifiers::default(),
      gestures: Vec::new(),
      touches: Vec::new(),
      axis_motions: Vec::new()
    }
  }
}

impl Index<KeyCode> for InputState {
  type Output = bool;

  #[inline]
  fn index(&self, key_code: KeyCode) -> &bool {
    if self.keys_held_physical.contains(&key_code) { &true } else { &false }
  }
}

impl Index<NamedKey> for InputState {
  type Output = bool;

  #[inline]
  fn index(&self, named_key: NamedKey) -> &bool {
    if self.keys_held_logical.contains(&named_key) { &true } else { &false }
  }
}

impl Index<MouseButton> for InputState {
  type Output = bool;

  #[inline]
  fn index(&self, button: MouseButton) -> &bool {
    match button {
      MouseButton::Left => &self.mouse_left_held,
      MouseButton::Right => &self.mouse_right_held,
      MouseButton::Middle => &self.mouse_middle_held,
      MouseButton::Back => &self.mouse_back_held,
      MouseButton::Forward => &self.mouse_forward_held,
      MouseButton::Other(_) => &false
    }
  }
}

/// Equivalent to [`ElementState`] but with an additional `Repeating` variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum KeyActionState {
  Pressed,
  Repeating,
  Released
}

impl From<KeyActionState> for ElementState {
  #[inline]
  fn from(state: KeyActionState) -> Self {
    match state {
      KeyActionState::Pressed | KeyActionState::Repeating => ElementState::Pressed,
      KeyActionState::Released => ElementState::Released
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct KeyAction {
  pub physical_key: PhysicalKey,
  pub logical_key: LogicalKey,
  pub state: KeyActionState
}

impl KeyAction {
  pub fn is_physical(&self, physical_key: &PhysicalKey, state: KeyActionState) -> bool {
    self.physical_key == *physical_key && self.state == state
  }

  pub fn is_logical(&self, logical_key: &LogicalKey, state: KeyActionState) -> bool {
    self.logical_key == *logical_key && self.state == state
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MouseAction {
  pub button: MouseButton,
  pub state: ElementState
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Gesture {
  Pinch {
    device_id: DeviceId,
    delta: f64,
    phase: TouchPhase
  },
  Pan {
    device_id: DeviceId,
    delta: PhysicalPosition<f32>,
    phase: TouchPhase
  },
  DoubleTap {
    device_id: DeviceId
  },
  Rotation {
    device_id: DeviceId,
    delta: f32,
    phase: TouchPhase
  },
  TouchpadPressure {
    device_id: DeviceId,
    pressure: f32,
    stage: i64
  }
}

impl Gesture {
  pub const fn device_id(&self) -> DeviceId {
    match *self {
      Self::Pinch { device_id, .. } => device_id,
      Self::Pan { device_id, .. } => device_id,
      Self::DoubleTap { device_id, .. } => device_id,
      Self::Rotation { device_id, .. } => device_id,
      Self::TouchpadPressure { device_id, .. } => device_id
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AxisMotion {
  device_id: DeviceId,
  axis: AxisId,
  value: f64
}



pub trait HasWindow {
  fn get_window(&self) -> &Window;

  fn get_window_id(&self) -> WindowId {
    self.get_window().id()
  }
}

impl HasWindow for Window {
  #[inline]
  fn get_window(&self) -> &Window {
    self
  }
}

macro_rules! impl_has_window_deref {
  ($(for $Type:ty where $T:ident),* $(,)?) => ($(
    impl<$T: HasWindow> HasWindow for $Type {
      #[inline]
      fn get_window(&self) -> &Window {
        $T::get_window(self)
      }
    }
  )*);
}

impl_has_window_deref!(
  for &T where T,
  for &mut T where T,
  for Rc<T> where T,
  for Arc<T> where T,
  for Box<T> where T
);



#[derive(Debug, Clone)]
pub struct WindowState<W: HasWindow> {
  initialized: bool,
  input_state: InputState,
  dropped_file: Option<PathBuf>,
  scale_factor: f64,
  window_size: PhysicalSize<u32>,
  window_holder: W
}

impl<W: HasWindow> WindowState<W> {
  fn new(window_holder: W) -> Self {
    let window = window_holder.get_window();
    WindowState {
      initialized: false,
      input_state: InputState::default(),
      dropped_file: None,
      scale_factor: window.scale_factor(),
      window_size: window.inner_size().into(),
      window_holder
    }
  }

  fn reset(&mut self) {
    self.input_state.reset();
    self.dropped_file = None;
  }

  #[inline]
  pub fn input(&self) -> &InputState {
    &self.input_state
  }

  #[inline]
  pub fn dropped_file(&self) -> Option<&Path> {
    self.dropped_file.as_deref()
  }

  #[inline]
  pub fn scale_factor(&self) -> f64 {
    self.scale_factor
  }

  /// Shortcut to [`Window::theme`].
  pub fn theme(&self) -> Option<Theme> {
    self.window().theme()
  }

  /// Shortcut to [`Window::has_focus`].
  pub fn is_focused(&self) -> bool {
    self.window().has_focus()
  }

  /// Shortcut to [`Window::is_visible`].
  pub fn is_visible(&self) -> Option<bool> {
    self.window().is_visible()
  }

  /// Shortcut to [`Window::is_maximized`].
  pub fn is_maximized(&self) -> bool {
    self.window().is_maximized()
  }

  /// Shortcut to [`Window::is_minimized`].
  pub fn is_minimized(&self) -> Option<bool> {
    self.window().is_minimized()
  }

  #[inline]
  pub fn window_size(&self) -> PhysicalSize<u32> {
    self.window_size
  }

  #[inline]
  pub fn window(&self) -> &Window {
    self.window_holder.get_window()
  }

  #[inline]
  pub fn window_holder(&self) -> &W {
    &self.window_holder
  }

  /// Only returns `Some` when the given position is within the window frame.
  pub fn clip_pos_in_frame(&self, position: PhysicalPosition<f32>) -> Option<PhysicalPosition<f32>> {
    let PhysicalSize { width, height } = self.window_size.cast::<f32>();
    if position.x >= 0.0 && position.x <= width && position.y >= 0.0 && position.y <= height {
      Some(position)
    } else {
      None
    }
  }

  pub fn handle_window_event<T, H: EventHandler<W, T>>(&mut self, handler: &mut H, event: WindowEvent, event_loop: &ActiveEventLoop) {
    match event {
      WindowEvent::CloseRequested => {
        if handler.on_close_requested(self) {
          event_loop.exit();
        };
      },
      WindowEvent::Destroyed => (),
      WindowEvent::RedrawRequested => {
        handler.render(self);
      },
      WindowEvent::ActivationTokenDone { .. } => (),
      WindowEvent::Focused(focused_state) => {
        if !focused_state {
          self.input_state = InputState::default();
        };
        handler.on_focus_changed(self, focused_state);
      },
      WindowEvent::Occluded(occluded_state) => {
        handler.on_occlusion_changed(self, occluded_state);
      },
      WindowEvent::HoveredFileCancelled => {
        handler.on_file_over(self, None, false);
      },
      WindowEvent::HoveredFile(path) => {
        handler.on_file_over(self, Some(path), false);
      },
      WindowEvent::DroppedFile(path) => {
        self.dropped_file = Some(path.clone());
        handler.on_file_over(self, Some(path), true);
      },
      WindowEvent::Moved(..) => (),
      WindowEvent::Resized(window_physical_inner_size) => {
        self.window_size = window_physical_inner_size;
        handler.on_resized(self, self.window_size, self.scale_factor);
      },
      WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
        self.scale_factor = scale_factor;
        handler.on_resized(self, self.window_size, self.scale_factor);
      },
      WindowEvent::KeyboardInput { event, .. } => {
        self.input_state.handle_keyboard_input(&event);
        handler.on_keyboard_input(self, event);
      },
      WindowEvent::Ime(event) => {
        handler.on_text_input(self, event);
      },
      WindowEvent::CursorEntered { .. } | WindowEvent::CursorLeft { .. } => (),
      WindowEvent::CursorMoved { position, .. } => {
        if let Some(position) = self.clip_pos_in_frame(position.cast()) {
          self.input_state.cursor_pos = Some(position);
          self.input_state.has_cursor_not_moved = false;
          handler.on_cursor_moved(self, position);
        } else {
          self.input_state.cursor_pos = None;
        };
      },
      WindowEvent::MouseInput { state, button, .. } => {
        self.input_state.handle_mouse_input(state, button);
        handler.on_mouse_input(self, state, button);
      },
      WindowEvent::MouseWheel { delta, .. } => {
        handler.on_mouse_scroll(self, delta);
      },
      WindowEvent::ModifiersChanged(modifiers) => {
        self.input_state.modifiers = modifiers;
      },
      WindowEvent::PinchGesture { device_id, delta, phase } => {
        let gesture = Gesture::Pinch { device_id, delta, phase };
        self.input_state.handle_gesture(gesture);
        handler.on_gesture(self, gesture);
      },
      WindowEvent::PanGesture { device_id, delta, phase } => {
        let gesture = Gesture::Pan { device_id, delta, phase };
        self.input_state.handle_gesture(gesture);
        handler.on_gesture(self, gesture);
      },
      WindowEvent::DoubleTapGesture { device_id } => {
        let gesture = Gesture::DoubleTap { device_id };
        self.input_state.handle_gesture(gesture);
        handler.on_gesture(self, gesture);
      },
      WindowEvent::RotationGesture { device_id, delta, phase } => {
        let gesture = Gesture::Rotation { device_id, delta, phase };
        self.input_state.handle_gesture(gesture);
        handler.on_gesture(self, gesture);
      },
      WindowEvent::TouchpadPressure { device_id, pressure, stage } => {
        let gesture = Gesture::TouchpadPressure { device_id, pressure, stage };
        self.input_state.handle_gesture(gesture);
        handler.on_gesture(self, gesture);
      },
      WindowEvent::Touch(touch) => {
        self.input_state.handle_touch(touch);
        handler.on_touch(self, touch);
      },
      WindowEvent::AxisMotion { device_id, axis, value } => {
        let axis_motion = AxisMotion { device_id, axis, value };
        self.input_state.handle_axis_motion(axis_motion);
        handler.on_axis_motion(self, axis_motion);
      },
      WindowEvent::ThemeChanged(..) => ()
    }
  }
}

macro_rules! unwrap_unreachable {
  ($expr:expr $(,)?) => (match $expr {
    Some(v) => v,
    None => unreachable!()
  });
  ($expr:expr, $($arg:tt)*) => (match $expr {
    Some(v) => v,
    None => unreachable!($($arg)*)
  });
}

#[derive(Debug)]
pub struct Application<W: HasWindow, H: EventHandler<W, T>, T: 'static = ()> {
  handler: Option<H>,
  window_state: WindowState<W>,
  phantom_data: PhantomData<T>
}

impl<W, H, T: 'static> Application<W, H, T>
where W: HasWindow, H: EventHandler<W, T> {
  pub fn new(window: W, handler: H) -> Self {
    Application {
      handler: Some(handler),
      window_state: WindowState::new(window.into()),
      phantom_data: PhantomData
    }
  }

  pub fn run(&mut self, event_loop: EventLoop<T>) -> Result<(), EventLoopError> {
    event_loop.run_app(self)
  }

  #[inline]
  fn decompose_mut(&mut self) -> (&mut H, &mut WindowState<W>) {
    (unwrap_unreachable!(self.handler.as_mut()), &mut self.window_state)
  }
}

macro_rules! application_handler_functions {
  (let $decomposed:pat, $event_loop:ident; $(fn $name:ident($($arg:ident: $Arg:ty),* $(,)?) $block:block)*) => ($(
    fn $name(&mut self, #[allow(unused)] $event_loop: &ActiveEventLoop, $(#[allow(unused)] $arg: $Arg),*) {
      #[allow(unused)]
      let $decomposed = self.decompose_mut();
      $block
    }
  )*);
}

impl<W, H, T: 'static> ApplicationHandler<T> for Application<W, H, T>
where W: HasWindow, H: EventHandler<W, T> {
  application_handler_functions!{
    let (handler, window_state), event_loop;

    fn new_events(start_cause: StartCause) {
      window_state.reset();
    }

    fn resumed() {
      if !replace(&mut window_state.initialized, true) {
        handler.init(window_state);
      };
      handler.on_resumed(window_state);
    }

    fn suspended() {
      handler.on_suspended(window_state);
    }

    fn window_event(window_id: WindowId, event: WindowEvent) {
      if window_state.window_holder.get_window().id() == window_id {
        handler.on_window_event(window_state, event, event_loop);
      };
    }

    fn user_event(event: T) {
      handler.on_user_event(window_state, event);
    }

    fn device_event(device_id: DeviceId, event: DeviceEvent) {
      handler.on_device_event(window_state, device_id, event);
    }

    fn about_to_wait() {
      handler.update(window_state);
      if handler.should_exit(window_state) {
        event_loop.exit();
      } else {
        window_state.window().request_redraw();
      };
    }
  }

  #[allow(unused)]
  fn exiting(&mut self, event_loop: &ActiveEventLoop) {
    unwrap_unreachable!(self.handler.take()).on_exited();
  }
}
