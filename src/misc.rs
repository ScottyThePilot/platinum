use femtovg::rgb::AsPixels;
use femtovg::imgref::{ImgRef, ImgRefMut};
use image::{Pixel, ImageBuffer};



#[allow(unused)]
pub(crate) trait OptionExt<T> {
  fn unwrap_unreachable(self) -> T;
  fn expect_unreachable(self, msg: &str) -> T;
}

impl<T> OptionExt<T> for Option<T> {
  fn unwrap_unreachable(self) -> T {
    self.unwrap_or_else(|| unreachable!())
  }

  fn expect_unreachable(self, msg: &str) -> T {
    self.unwrap_or_else(|| unreachable!("{msg}"))
  }
}



pub fn rgb_imgref_from_image<'i, T>(image: &'i ImageBuffer<image::Rgb<T>, Vec<T>>) -> ImgRef<'i, femtovg::rgb::Rgb<T>>
where image::Rgb<T>: Pixel<Subpixel = T> {
  let (width, height) = image.dimensions();
  ImgRef::new(image.as_pixels(), width as usize, height as usize)
}

pub fn rgb_imgrefmut_from_image<'i, T>(image: &'i mut ImageBuffer<image::Rgb<T>, Vec<T>>) -> ImgRefMut<'i, femtovg::rgb::Rgb<T>>
where image::Rgb<T>: Pixel<Subpixel = T> {
  let (width, height) = image.dimensions();
  ImgRefMut::new(image.as_pixels_mut(), width as usize, height as usize)
}

pub fn rgb_alpha_imgref_from_image<'i, T>(image: &'i ImageBuffer<image::Rgba<T>, Vec<T>>) -> ImgRef<'i, femtovg::rgb::Rgba<T>>
where image::Rgba<T>: Pixel<Subpixel = T> {
  let (width, height) = image.dimensions();
  ImgRef::new(image.as_pixels(), width as usize, height as usize)
}

pub fn rgb_alpha_imgrefmut_from_image<'i, T>(image: &'i mut ImageBuffer<image::Rgba<T>, Vec<T>>) -> ImgRefMut<'i, femtovg::rgb::Rgba<T>>
where image::Rgba<T>: Pixel<Subpixel = T> {
  let (width, height) = image.dimensions();
  ImgRefMut::new(image.as_pixels_mut(), width as usize, height as usize)
}

pub fn gray_imgref_from_image<'i, T>(image: &'i ImageBuffer<image::Luma<T>, Vec<T>>) -> ImgRef<'i, femtovg::rgb::Gray<T>>
where image::Luma<T>: Pixel<Subpixel = T> {
  let (width, height) = image.dimensions();
  ImgRef::new(image.as_pixels(), width as usize, height as usize)
}

pub fn gray_imgrefmut_from_image<'i, T>(image: &'i mut ImageBuffer<image::Luma<T>, Vec<T>>) -> ImgRefMut<'i, femtovg::rgb::Gray<T>>
where image::Luma<T>: Pixel<Subpixel = T> {
  let (width, height) = image.dimensions();
  ImgRefMut::new(image.as_pixels_mut(), width as usize, height as usize)
}

pub fn gray_alpha_imgref_from_image<'i, T>(image: &'i ImageBuffer<image::LumaA<T>, Vec<T>>) -> ImgRef<'i, femtovg::rgb::GrayAlpha<T>>
where image::LumaA<T>: Pixel<Subpixel = T> {
  let (width, height) = image.dimensions();
  ImgRef::new(image.as_pixels(), width as usize, height as usize)
}

pub fn gray_alpha_imgrefmut_from_image<'i, T>(image: &'i mut ImageBuffer<image::LumaA<T>, Vec<T>>) -> ImgRefMut<'i, femtovg::rgb::GrayAlpha<T>>
where image::LumaA<T>: Pixel<Subpixel = T> {
  let (width, height) = image.dimensions();
  ImgRefMut::new(image.as_pixels_mut(), width as usize, height as usize)
}



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
