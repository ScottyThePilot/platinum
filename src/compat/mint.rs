use femtovg::{Canvas, Color, FillRule, ImageId, Paint, Path, Renderer, Solidity, Transform2D};
use mint::{Point2, Vector2};



pub trait CanvasExt {
  fn translate_mint(&mut self, point: impl Into<Point2<f32>>);
  fn scale_mint(&mut self, point: impl Into<Point2<f32>>);
  fn scissor_mint(&mut self, point: impl Into<Point2<f32>>, size: impl Into<Vector2<f32>>);
  fn intersect_scissor_mint(&mut self, point: impl Into<Point2<f32>>, size: impl Into<Vector2<f32>>);
  fn contains_point_mint(&self, path: &Path, point: impl Into<Point2<f32>>, fill_rule: FillRule) -> bool;
}

impl<T: Renderer> CanvasExt for Canvas<T> {
  fn translate_mint(&mut self, point: impl Into<Point2<f32>>) {
    let point = point.into();
    self.translate(point.x, point.y);
  }

  fn scale_mint(&mut self, point: impl Into<Point2<f32>>) {
    let point = point.into();
    self.scale(point.x, point.y);
  }

  fn scissor_mint(&mut self, point: impl Into<Point2<f32>>, size: impl Into<Vector2<f32>>) {
    let point = point.into();
    let size = size.into();
    self.scissor(point.x, point.y, size.x, size.y);
  }

  fn intersect_scissor_mint(&mut self, point: impl Into<Point2<f32>>, size: impl Into<Vector2<f32>>) {
    let point = point.into();
    let size = size.into();
    self.intersect_scissor(point.x, point.y, size.x, size.y);
  }

  fn contains_point_mint(&self, path: &Path, point: impl Into<Point2<f32>>, fill_rule: FillRule) -> bool {
    let point = point.into();
    self.contains_point(path, point.x, point.y, fill_rule)
  }
}

pub trait Transform2DExt {
  fn translation_mint(t: impl Into<Vector2<f32>>) -> Self;
  fn scaling_mint(s: impl Into<Vector2<f32>>) -> Self;
  fn translate_mint(&mut self, t: impl Into<Vector2<f32>>);
  fn scale_mint(&mut self, s: impl Into<Vector2<f32>>);
  fn transform_point_mint(&self, s: impl Into<Vector2<f32>>) -> (f32, f32);
}

impl Transform2DExt for Transform2D {
  fn translation_mint(t: impl Into<Vector2<f32>>) -> Self {
    let t = t.into();
    Self::translation(t.x, t.y)
  }

  fn scaling_mint(s: impl Into<Vector2<f32>>) -> Self {
    let s = s.into();
    Self::scaling(s.x, s.y)
  }

  fn translate_mint(&mut self, t: impl Into<Vector2<f32>>) {
    let t = t.into();
    self.translate(t.x, t.y)
  }

  fn scale_mint(&mut self, s: impl Into<Vector2<f32>>) {
    let s = s.into();
    self.scale(s.x, s.y)
  }

  fn transform_point_mint(&self, s: impl Into<Vector2<f32>>) -> (f32, f32) {
    let s = s.into();
    self.transform_point(s.x, s.y)
  }
}

pub trait PaintExt {
  fn image_mint(id: ImageId, c: impl Into<Point2<f32>>, size: impl Into<Vector2<f32>>, angle: f32, alpha: f32) -> Self;
  fn image_tint_mint(id: ImageId, c: impl Into<Point2<f32>>, size: impl Into<Vector2<f32>>, angle: f32, tint: Color) -> Self;
  fn linear_gradient_mint(start: impl Into<Point2<f32>>, end: impl Into<Point2<f32>>, start_color: Color, end_color: Color) -> Self;
  fn linear_gradient_stops_mint(start: impl Into<Point2<f32>>, end: impl Into<Point2<f32>>, stops: impl IntoIterator<Item = (f32, Color)>) -> Self;
  fn box_gradient_mint(point: impl Into<Point2<f32>>, size: impl Into<Vector2<f32>>, radius: f32, feather: f32, inner_color: Color, outer_color: Color) -> Self;
  fn radial_gradient_mint(c: impl Into<Point2<f32>>, in_radius: f32, out_radius: f32, inner_color: Color, outer_color: Color) -> Self;
  fn radial_gradient_stops_mint(c: impl Into<Point2<f32>>, in_radius: f32, out_radius: f32, stops: impl IntoIterator<Item = (f32, Color)>) -> Self;
  fn conic_gradient_stops_mint(c: impl Into<Point2<f32>>, stops: impl IntoIterator<Item = (f32, Color)>) -> Self;
}

impl PaintExt for Paint {
  fn image_mint(id: ImageId, c: impl Into<Point2<f32>>, size: impl Into<Vector2<f32>>, angle: f32, alpha: f32) -> Self {
    let c = c.into();
    let size = size.into();
    Self::image(id, c.x, c.y, size.x, size.y, angle, alpha)
  }

  fn image_tint_mint(id: ImageId, c: impl Into<Point2<f32>>, size: impl Into<Vector2<f32>>, angle: f32, tint: Color) -> Self {
    let c = c.into();
    let size = size.into();
    Self::image_tint(id, c.x, c.y, size.x, size.y, angle, tint)
  }

  fn linear_gradient_mint(start: impl Into<Point2<f32>>, end: impl Into<Point2<f32>>, start_color: Color, end_color: Color) -> Self {
    let start = start.into();
    let end = end.into();
    Self::linear_gradient(start.x, start.y, end.x, end.y, start_color, end_color)
  }

  fn linear_gradient_stops_mint(start: impl Into<Point2<f32>>, end: impl Into<Point2<f32>>, stops: impl IntoIterator<Item = (f32, Color)>) -> Self {
    let start = start.into();
    let end = end.into();
    Self::linear_gradient_stops(start.x, start.y, end.x, end.y, stops)
  }

  fn box_gradient_mint(point: impl Into<Point2<f32>>, size: impl Into<Vector2<f32>>, radius: f32, feather: f32, inner_color: Color, outer_color: Color) -> Self {
    let point = point.into();
    let size = size.into();
    Self::box_gradient(point.x, point.y, size.x, size.y, radius, feather, inner_color, outer_color)
  }

  fn radial_gradient_mint(c: impl Into<Point2<f32>>, in_radius: f32, out_radius: f32, inner_color: Color, outer_color: Color) -> Self {
    let c = c.into();
    Self::radial_gradient(c.x, c.y, in_radius, out_radius, inner_color, outer_color)
  }

  fn radial_gradient_stops_mint(c: impl Into<Point2<f32>>, in_radius: f32, out_radius: f32, stops: impl IntoIterator<Item = (f32, Color)>) -> Self {
    let c = c.into();
    Self::radial_gradient_stops(c.x, c.y, in_radius, out_radius, stops)
  }

  fn conic_gradient_stops_mint(c: impl Into<Point2<f32>>, stops: impl IntoIterator<Item = (f32, Color)>) -> Self {
    let c = c.into();
    Self::conic_gradient_stops(c.x, c.y, stops)
  }
}

pub trait PathExt {
  fn move_to_mint(&mut self, point: impl Into<Point2<f32>>);
  fn line_to_mint(&mut self, point: impl Into<Point2<f32>>);
  fn bezier_to_mint(&mut self, c1: impl Into<Point2<f32>>, c2: impl Into<Point2<f32>>, point: impl Into<Point2<f32>>);
  fn quad_to_mint(&mut self, c: impl Into<Point2<f32>>, point: impl Into<Point2<f32>>);
  fn arc_mint(&mut self, c: impl Into<Point2<f32>>, r: f32, a0: f32, a1: f32, dir: Solidity);
  fn arc_to_mint(&mut self, point1: impl Into<Point2<f32>>, point2: impl Into<Point2<f32>>, radius: f32);
  fn rect_mint(&mut self, point: impl Into<Point2<f32>>, size: impl Into<Vector2<f32>>);
  fn rounded_rect_mint(&mut self, point: impl Into<Point2<f32>>, size: impl Into<Vector2<f32>>, r: f32);
  fn rounded_rect_varying_mint(&mut self, point: impl Into<Point2<f32>>, size: impl Into<Vector2<f32>>, rad_top_left: f32, rad_top_right: f32, rad_bottom_right: f32, rad_bottom_left: f32);
  fn ellipse_mint(&mut self, c: impl Into<Point2<f32>>, r: impl Into<Vector2<f32>>);
  fn circle_mint(&mut self, c: impl Into<Point2<f32>>, r: f32);
}

impl PathExt for Path {
  fn move_to_mint(&mut self, point: impl Into<Point2<f32>>) {
    let point = point.into();
    self.move_to(point.x, point.y)
  }

  fn line_to_mint(&mut self, point: impl Into<Point2<f32>>) {
    let point = point.into();
    self.line_to(point.x, point.y);
  }

  fn bezier_to_mint(&mut self, c1: impl Into<Point2<f32>>, c2: impl Into<Point2<f32>>, point: impl Into<Point2<f32>>) {
    let c1 = c1.into();
    let c2 = c2.into();
    let point = point.into();
    self.bezier_to(c1.x, c1.y, c2.x, c2.y, point.x, point.y);
  }

  fn quad_to_mint(&mut self, c: impl Into<Point2<f32>>, point: impl Into<Point2<f32>>) {
    let c = c.into();
    let point = point.into();
    self.quad_to(c.x, c.y, point.x, point.y);
  }

  fn arc_mint(&mut self, c: impl Into<Point2<f32>>, r: f32, a0: f32, a1: f32, dir: Solidity) {
    let c = c.into();
    self.arc(c.x, c.y, r, a0, a1, dir);
  }

  fn arc_to_mint(&mut self, point1: impl Into<Point2<f32>>, point2: impl Into<Point2<f32>>, radius: f32) {
    let point1 = point1.into();
    let point2 = point2.into();
    self.arc_to(point1.x, point1.y, point2.x, point2.y, radius);
  }

  fn rect_mint(&mut self, point: impl Into<Point2<f32>>, size: impl Into<Vector2<f32>>) {
    let point = point.into();
    let size = size.into();
    self.rect(point.x, point.y, size.x, size.y);
  }

  fn rounded_rect_mint(&mut self, point: impl Into<Point2<f32>>, size: impl Into<Vector2<f32>>, r: f32) {
    let point = point.into();
    let size = size.into();
    self.rounded_rect(point.x, point.y, size.x, size.y, r);
  }

  fn rounded_rect_varying_mint(&mut self, point: impl Into<Point2<f32>>, size: impl Into<Vector2<f32>>, rad_top_left: f32, rad_top_right: f32, rad_bottom_right: f32, rad_bottom_left: f32) {
    let point = point.into();
    let size = size.into();
    self.rounded_rect_varying(point.x, point.y, size.x, size.y, rad_top_left, rad_top_right, rad_bottom_right, rad_bottom_left);
  }

  fn ellipse_mint(&mut self, c: impl Into<Point2<f32>>, r: impl Into<Vector2<f32>>) {
    let c = c.into();
    let r = r.into();
    self.ellipse(c.x, c.y, r.x, r.y);
  }

  fn circle_mint(&mut self, c: impl Into<Point2<f32>>, r: f32) {
    let c = c.into();
    self.circle(c.x, c.y, r);
  }
}
