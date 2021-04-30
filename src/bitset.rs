use std::fmt;

pub const WORD_BIT: usize = 32;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct ImmutableBitSet<'a>(pub &'a [u32]);

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct BitSet<'a>(pub &'a mut [u32]);

#[derive(Copy, Clone)]
pub struct UnsafeBitSet(*mut u32);

#[inline(always)]
pub fn ibs(x: &[u32]) -> ImmutableBitSet { ImmutableBitSet(x) }

#[inline(always)]
pub fn bs(x: &mut [u32]) -> BitSet { BitSet(x) }

#[inline(always)]
pub unsafe fn ubs(x: *const [u32]) -> UnsafeBitSet { UnsafeBitSet(x as _) }

#[inline(always)]
pub unsafe fn ubs1(x: *const u32) -> UnsafeBitSet { UnsafeBitSet(x as _) }

#[inline(always)]
pub fn bslen(bits: usize) -> usize { (bits + 31) >> 5 }

#[inline(always)]
pub fn bsmake(bits: usize) -> Box<[u32]> { vec![0; bslen(bits)].into() }

#[inline(always)]
pub fn elem2bs(iter: impl IntoIterator<Item=usize>, bits: usize) -> Box<[u32]> {
  let mut bs = bsmake(bits);
  for x in iter { BitSet(&mut bs).set(x); }
  bs
}

#[inline(always)]
pub fn slice2bs(x: &[bool]) -> Box<[u32]> {
  let mut bs = bsmake(x.len());
  for (i, &x) in x.iter().enumerate() {
    if x { UnsafeBitSet(bs.as_mut_ptr()).set(i) }
  }
  bs
}

impl ImmutableBitSet<'_> {
  #[inline(always)]
  pub fn get(&self, i: usize) -> bool { ((self.0[i >> 5] >> (i & 31)) & 1) != 0 }

  #[inline(always)]
  pub fn any(&self) -> bool { self.0.iter().any(|&x| x != 0) }

  #[inline(always)]
  pub fn ones(self, mut f: impl FnMut(usize)) {
    for (i, &x) in self.0.iter().enumerate() {
      if x != 0 {
        for j in 0..WORD_BIT {
          if ((x >> j) & 1) != 0 { f(i * WORD_BIT + j) }
        }
      }
    }
  }
}

impl fmt::Display for ImmutableBitSet<'_> {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    let mut list = f.debug_list();
    self.ones(|x| { list.entry(&x); });
    list.finish()
  }
}

macro_rules! impl_single {
  ($name: ident $($op: tt)*) => {
    #[inline(always)]
    pub fn $name(&mut self, i: usize) { self.0[i >> 5] $($op)* (1 << (i & 31)); }
  };
}

macro_rules! impl_group {
  ($name: ident) => {
    #[inline(always)]
    pub fn $name<'b>(&mut self, rhs: &'b [u32]) -> bool {
      let len = self.0.len();
      assert_eq!(len, rhs.len());
      UnsafeBitSet(self.0.as_mut_ptr()).$name(rhs.as_ptr(), len)
    }
  }
}

impl BitSet<'_> {
  #[inline(always)]
  pub fn as_imm(&self) -> ImmutableBitSet { ImmutableBitSet(self.0) }

  #[inline(always)]
  pub fn clear(&mut self) { for x in self.0.iter_mut() { *x = 0; } }

  #[inline(always)]
  pub fn inv(&mut self) { for x in self.0.iter_mut() { *x = !*x; } }

  impl_single!(set |=);
  impl_single!(del &= !);
  impl_single!(flip ^=);
  impl_group!(or);
  impl_group!(and);
  impl_group!(andn); // andn = x & ~y = x \ y (x, y are sets)
  impl_group!(xor);
}

macro_rules! impl_single {
  ($name: ident $($op: tt)*) => {
    #[inline(always)]
    pub fn $name(self, i: usize) {
      unsafe { *self.0.add(i >> 5) $($op)* (1 << (i & 31)); }
    }
  };
}

macro_rules! impl_group {
  ($name: ident $($op: tt)*) => {
    #[inline(always)]
    pub fn $name(self, rhs: *const u32, len: usize) -> bool {
      unsafe {
        let (mut x, mut y, mut changed) = (self.0, rhs, false);
        for _ in 0..len {
          let ox = *x;
          *x $($op)* *y;
          changed |= *x != ox;
          x = x.add(1);
          y = y.add(1);
        }
        changed
      }
    }
  }
}

impl UnsafeBitSet {
  pub fn get(self, i: usize) -> bool { unsafe { ((*self.0.add(i >> 5) >> (i & 31)) & 1) != 0 } }

  impl_single!(set |=);
  impl_single!(del &= !);
  impl_single!(flip ^=);
  impl_group!(or |=);
  impl_group!(and &=);
  impl_group!(andn &= !); // andn = x & ~y = x \ y (x, y are sets)
  impl_group!(xor ^=);
}
