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
