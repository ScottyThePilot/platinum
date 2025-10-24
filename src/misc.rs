use femtovg::rgb::AsPixels;
use femtovg::imgref::{ImgRef, ImgRefMut};
use image::{Pixel, ImageBuffer};



pub(crate) trait OptionExt<T> {
  #[allow(unused)]
  fn unwrap_unreachable(self) -> T;
  #[allow(unused)]
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



pub trait ImageBufferExt {
  type Pixel;

  fn as_imgref(&self) -> ImgRef<'_, Self::Pixel>;
  fn as_imgref_mut(&mut self) -> ImgRefMut<'_, Self::Pixel>;
}

impl<T> ImageBufferExt for ImageBuffer<image::Rgb<T>, Vec<T>>
where image::Rgb<T>: Pixel<Subpixel = T> {
  type Pixel = femtovg::rgb::Rgb<T>;

  fn as_imgref(&self) -> ImgRef<'_, Self::Pixel> {
    let (width, height) = self.dimensions();
    ImgRef::new(self.as_pixels(), width as usize, height as usize)
  }

  fn as_imgref_mut(&mut self) -> ImgRefMut<'_, Self::Pixel> {
    let (width, height) = self.dimensions();
    ImgRefMut::new(self.as_pixels_mut(), width as usize, height as usize)
  }
}

impl<T> ImageBufferExt for ImageBuffer<image::Rgba<T>, Vec<T>>
where image::Rgba<T>: Pixel<Subpixel = T> {
  type Pixel = femtovg::rgb::Rgba<T>;

  fn as_imgref(&self) -> ImgRef<'_, Self::Pixel> {
    let (width, height) = self.dimensions();
    ImgRef::new(self.as_pixels(), width as usize, height as usize)
  }

  fn as_imgref_mut(&mut self) -> ImgRefMut<'_, Self::Pixel> {
    let (width, height) = self.dimensions();
    ImgRefMut::new(self.as_pixels_mut(), width as usize, height as usize)
  }
}

impl<T> ImageBufferExt for ImageBuffer<image::Luma<T>, Vec<T>>
where image::Luma<T>: Pixel<Subpixel = T> {
  type Pixel = femtovg::rgb::Gray<T>;

  fn as_imgref(&self) -> ImgRef<'_, Self::Pixel> {
    let (width, height) = self.dimensions();
    ImgRef::new(self.as_pixels(), width as usize, height as usize)
  }

  fn as_imgref_mut(&mut self) -> ImgRefMut<'_, Self::Pixel> {
    let (width, height) = self.dimensions();
    ImgRefMut::new(self.as_pixels_mut(), width as usize, height as usize)
  }
}

impl<T> ImageBufferExt for ImageBuffer<image::LumaA<T>, Vec<T>>
where image::LumaA<T>: Pixel<Subpixel = T> {
  type Pixel = femtovg::rgb::GrayAlpha<T>;

  fn as_imgref(&self) -> ImgRef<'_, Self::Pixel> {
    let (width, height) = self.dimensions();
    ImgRef::new(self.as_pixels(), width as usize, height as usize)
  }

  fn as_imgref_mut(&mut self) -> ImgRefMut<'_, Self::Pixel> {
    let (width, height) = self.dimensions();
    ImgRefMut::new(self.as_pixels_mut(), width as usize, height as usize)
  }
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
