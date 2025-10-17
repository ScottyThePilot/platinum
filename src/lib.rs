#![warn(
  absolute_paths_not_starting_with_crate,
  redundant_imports,
  redundant_lifetimes,
  future_incompatible,
  deprecated_in_future,
  missing_copy_implementations,
  missing_debug_implementations,
  unnameable_types,
  unreachable_pub
)]

pub extern crate image;
pub extern crate femtovg;
pub extern crate glutin_winit;
pub extern crate glutin;
pub extern crate winit;

#[macro_use]
pub mod misc;
pub mod engine;
pub mod windowing;
