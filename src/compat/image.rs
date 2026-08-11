use femtovg::rgb::AsPixels;
use femtovg::imgref::{ImgRef, ImgRefMut};
use image::{Pixel, ImageBuffer};



/// A utility trait for performing conversion of [`ImageBuffer`]s into [`ImgRef`]s or [`ImgRefMut`]s.
pub trait ImageBufferExt {
  /// The `Pixel` type for any resultant [`ImgRef`]s or [`ImgRefMut`]s.
  type Pixel;

  /// Gets this [`ImageBuffer`] as an [`ImgRef`].
  fn as_imgref(&self) -> ImgRef<'_, Self::Pixel>;

  /// Gets this [`ImageBuffer`] as an [`ImgRefMut`].
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
