#[derive(Debug)]
struct ElesRef<'e, T: 'e> {
    raw: &'e [T],
    start: usize,
    end: usize,
    is_reverse: bool,
}
impl<'e, T: Clone + std::fmt::Debug> ElesRef<'e, T> {
    fn new<'l: 'e>(slice: &'l [T]) -> Self {
        ElesRef {
            raw: slice,
            start: 0,
            end: slice.len(),
            is_reverse: false,
        }
    }
    fn reverse(&mut self) {
        self.is_reverse = !self.is_reverse;
    }
    #[inline]    
    fn as_slice(&self) -> &[T] {
        &self.raw[self.start..self.end]
    }
    #[inline]    
    fn slice<RG: IntoRg>(&self, rg: RG) -> &[T] {
        let (rg0, rg1) = rg.into().into_range(self.start, self.end, self.is_reverse);
        self.check(&rg0, &rg1);
        &self.raw[rg0..rg1]
    }
    fn cut<RG: IntoRg>(&mut self, rg: RG) {
        let (rg0, rg1) = rg.into().into_range(self.start, self.end, self.is_reverse);
        self.check(&rg0, &rg1);
        self.start = rg0;
        self.end = rg1;
    }
    #[inline]
    fn check(&self, rg0: &usize, rg1: &usize) {
        if rg0 > rg1 {
            panic!("slice index starts at {} but ends at {}", rg0, rg1);
        }
        if *rg0 > self.raw.len() {
            panic!("slice index starts at {} but ends at {}",
                   rg0,
                   self.raw.len());
        }
        if *rg1 > self.raw.len() {
            panic!("index {} out of range for slice of length {}",
                   rg1,
                   self.raw.len());
        }
    }
    fn is_empty(&self) -> bool {
        self.start == self.end
    }
    fn len(&self) -> usize {
        self.end - self.start
    }
}
impl<'e, T: Clone + std::fmt::Debug> std::fmt::Display for ElesRef<'e, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.as_slice())
    }
}

use std::ops::{Range, RangeFrom, RangeTo, RangeFull};
#[derive(Debug)]
enum Rg {
    FT(Range<usize>),
    F(RangeFrom<usize>),
    T(RangeTo<usize>),
    Full,
}
impl Rg {
    fn into_range(self, start: usize, end: usize, is_reverse: bool) -> (usize, usize) {
        if is_reverse {
            match self {
                Rg::FT(ft) => (end - ft.end, end - ft.start),
                Rg::F(f) => (start, end - f.start),
                Rg::T(t) => (end - t.end, end),
                Rg::Full => (start, end),
            }
        } else {
            match self {
                Rg::FT(ft) => (ft.start + start, ft.end + start),
                Rg::F(f) => (f.start + start, end),
                Rg::T(t) => (start, start + t.end),
                Rg::Full => (start, end),
            }
        }
    }
}
trait IntoRg {
    fn into(self) -> Rg;
}

impl IntoRg for Range<usize> {
    fn into(self) -> Rg {
        Rg::FT(self)
    }
}
impl IntoRg for RangeFrom<usize> {
    fn into(self) -> Rg {
        Rg::F(self)
    }
}
impl IntoRg for RangeTo<usize> {
    fn into(self) -> Rg {
        Rg::T(self)
    }
}
impl IntoRg for RangeFull {
    fn into(self) -> Rg {
        Rg::Full
    }
}