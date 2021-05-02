use core::fmt::*;

#[inline(always)]
pub fn fn2display(f: impl Fn(&mut Formatter) -> Result) -> impl Display {
  struct S<F>(F);
  impl<F: Fn(&mut Formatter) -> Result> Display for S<F> {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter) -> Result { (self.0)(f) }
  }
  S(f)
}

pub fn sep<'a, T: Display>(it: impl Iterator<Item=T> + Clone + 'a, sep: &'a str) -> impl Display + 'a {
  fn2display(move |f| {
    let mut seen = false;
    for t in it.clone() {
      if seen { f.write_str(sep)?; }
      t.fmt(f)?;
      seen = true;
    }
    Ok(())
  })
}

pub fn opt<'a, T: Display>(x: &'a Option<T>) -> impl Display + 'a {
  fn2display(move |f| if let Some(x) = x { x.fmt(f) } else { f.write_str("None") })
}
