use std::{
    cmp::{max, min},
    fmt::Display,
    ops::Range,
};

#[derive(Debug, PartialEq, Eq, Clone, Hash, Default, Copy, PartialOrd, Ord)]
pub struct Span {
    pub lo: u32,
    pub hi: u32,
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("").field(&self.lo).field(&self.hi).finish()
    }
}

impl Span {
    #[inline]
    pub fn with_lo(mut self, lo: u32) -> Span {
        self.lo = lo;
        self
    }
    #[inline]
    pub fn with_hi(mut self, hi: u32) -> Span {
        self.hi = hi;
        self
    }
    /// Returns `true` if this is a dummy span with any hygienic context.
    #[inline]
    pub fn is_dummy(self) -> bool {
        self.lo == 0 && self.hi == 0
    }
    /// Returns `true` if `self` fully encloses `other`.
    pub fn contains(self, other: Self) -> bool {
        self.lo <= other.lo && other.hi <= self.hi
    }

    /// Returns a new span representing an empty span at the beginning of this span.
    #[inline]
    pub fn shrink_to_lo(self) -> Span {
        self.with_hi(self.lo)
    }
    /// Returns a new span representing an empty span at the end of this span.
    #[inline]
    pub fn shrink_to_hi(self) -> Span {
        self.with_lo(self.hi)
    }

    #[inline]
    /// Returns `true` if `hi == lo`.
    pub fn is_empty(self) -> bool {
        self.hi == self.lo
    }

    /// Returns `self` if `self` is not the dummy span, and `other` otherwise.
    pub fn substitute_dummy(self, other: Span) -> Span {
        if self.is_dummy() {
            other
        } else {
            self
        }
    }

    /// Returns `true` if `self` touches `other`.
    pub fn overlaps(self, other: Span) -> bool {
        self.lo < other.hi && other.lo < self.hi
    }

    /// Returns `true` if `self` touches or adjoins `other`.
    pub fn overlaps_or_adjacent(self, other: Span) -> bool {
        self.lo <= other.hi && other.lo <= self.hi
    }

    /// Returns `true` if the spans are equal with regards to the source text.
    ///
    /// Use this instead of `==` when either span could be generated code,
    /// and you only care that they point to the same bytes of source text.
    pub fn source_equal(self, other: Span) -> bool {
        self.lo == other.lo && self.hi == other.hi
    }

    /// Returns `Some(span)`, where the start is trimmed by the end of `other`.
    pub fn trim_start(self, other: Span) -> Option<Span> {
        if self.hi > other.hi {
            Some(self.with_lo(max(self.lo, other.hi)))
        } else {
            None
        }
    }

    /// Returns `Some(span)`, where the end is trimmed by the start of `other`.
    pub fn trim_end(self, other: Span) -> Option<Span> {
        if self.lo < other.lo {
            Some(self.with_hi(min(self.hi, other.lo)))
        } else {
            None
        }
    }
}

impl From<Span> for Range<usize> {
    fn from(val: Span) -> Self {
        Range {
            start: val.lo as usize,
            end: val.hi as usize,
        }
    }
}
