//! A bare-bones wrapper library around `femtovg` and `winit`
//! for quickly setting up graphics programs.

#![warn(
  absolute_paths_not_starting_with_crate,
  redundant_imports,
  redundant_lifetimes,
  future_incompatible,
  deprecated_in_future,
  missing_copy_implementations,
  missing_debug_implementations,
  missing_docs,
  unnameable_types,
  unreachable_pub
)]

pub extern crate image;
pub extern crate femtovg;
pub extern crate glutin_winit;
pub extern crate glutin;
pub extern crate mint;
pub extern crate winit;

macro_rules! delegate {
  ($delegate:ident: $vis:vis fn $name:ident(&self $(, $arg:ident : $Arg:ty)* $(,)?) $(-> $Ret:ty)?) => (
    #[inline] $vis fn $name(&self, $($arg: $Arg),*) $(-> $Ret)? { self.$delegate.$name($($arg),*) }
  );
  ($delegate:ident: $vis:vis fn $name:ident(&mut self $(, $arg:ident : $Arg:ty)* $(,)?) $(-> $Ret:ty)?) => (
    #[inline] $vis fn $name(&mut self, $($arg: $Arg),*) $(-> $Ret)? { self.$delegate.$name($($arg),*) }
  );
  ($delegate:ident: $vis:vis fn $name:ident(self $(, $arg:ident : $Arg:ty)* $(,)?) $(-> $Ret:ty)?) => (
    #[inline] $vis fn $name(self, $($arg: $Arg),*) $(-> $Ret)? { self.$delegate.$name($($arg),*) }
  );
}

pub mod compat;
pub mod engine;
pub mod windowing;
