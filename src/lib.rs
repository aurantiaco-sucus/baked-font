#![no_std]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::iter::Peekable;
use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Font {
    pub bitmap: Vec<u8>,
    pub width: u32,
    pub map1: BTreeMap<char, Glyph>,
    pub map2: BTreeMap<[char; 2], Glyph>,
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Glyph {
    pub pos: (u32, u32),
    pub size: (u8, u8),
    pub offset: (i8, i8),
}

impl Font {
    pub fn lookup_single(&self, c: char) -> Option<Glyph> {
        self.map1.get(&c).copied()
    }

    pub fn lookup_double(&self, c1: char, c2: char) -> Option<Glyph> {
        self.map2.get(&[c1, c2]).copied()
    }

    pub fn lookup(&self, str: &[char], pos: usize) -> Option<GlyphResult> {
        if pos < str.len() {
            if pos + 1 < str.len() {
                if let Some(g) = self.map2.get(&[str[pos], str[pos + 1]]) {
                    Some(GlyphResult::Double(*g, [str[pos], str[pos + 1]]))
                } else {
                    Some(self.lookup_single_gr(str[pos]))
                }
            } else {
                Some(self.lookup_single_gr(str[pos]))
            }
        } else {
            None
        }
    }

    fn lookup_single_gr(&self, ch: char) -> GlyphResult {
        if let Some(g) = self.map1.get(&ch) {
            GlyphResult::Single(*g, ch)
        } else {
            GlyphResult::Unknown(ch)
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GlyphResult {
    Unknown(char),
    Single(Glyph, char),
    Double(Glyph, [char; 2]),
}

impl GlyphResult {
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown(_))
    }

    pub const fn is_single(&self) -> bool {
        matches!(self, Self::Single(_, _))
    }

    pub const fn is_double(&self) -> bool {
        matches!(self, Self::Double(_, _))
    }

    pub fn first_char(&self) -> char {
        match self {
            Self::Unknown(c) => *c,
            Self::Single(_, c) => *c,
            Self::Double(_, [c, _]) => *c,
        }
    }

    pub fn char_len(&self) -> usize {
        match self {
            Self::Unknown(_) => 1,
            Self::Single(_, _) => 1,
            Self::Double(_, _) => 2,
        }
    }
}

pub struct CharSliceGlyphIterator<'a> {
    font: &'a Font,
    str: &'a [char],
    pos: usize,
}

impl<'a> CharSliceGlyphIterator<'a> {
    pub fn new(font: &'a Font, str: &'a [char]) -> Self {
        Self { font, str, pos: 0 }
    }
}

impl<'a> Iterator for CharSliceGlyphIterator<'a> {
    type Item = GlyphResult;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.font.lookup(self.str, self.pos)?;
        self.pos += res.char_len();
        Some(res)
    }
}

pub struct CharPeekableGlyphIterator<'a, T: Iterator<Item=char>> {
    font: &'a Font,
    iter: Peekable<T>,
}

impl<'a, T: Iterator<Item=char>> CharPeekableGlyphIterator<'a, T> {
    pub fn new(font: &'a Font, iter: T) -> Self {
        Self { font, iter: iter.peekable() }
    }
}

impl<'a, T: Iterator<Item=char>> Iterator for CharPeekableGlyphIterator<'a, T> {
    type Item = GlyphResult;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.iter.next();
        let next = self.iter.peek().copied();
        if let Some(c) = cur {
            if let Some(n) = next {
                if let Some(g) = self.font.lookup_double(c, n) {
                    Some(GlyphResult::Double(g, [c, n]))
                } else {
                    Some(self.font.lookup_single_gr(c))
                }
            } else {
                Some(self.font.lookup_single_gr(c))
            }
        } else {
            None
        }
    }
}

impl Font {
    pub fn lookup_slice<'a>(&'a self, str: &'a [char]) -> CharSliceGlyphIterator {
        CharSliceGlyphIterator::new(self, str)
    }

    pub fn lookup_peekable<T: Iterator<Item=char>>(&self, iter: T) -> CharPeekableGlyphIterator<T> {
        CharPeekableGlyphIterator::new(self, iter)
    }

    pub fn lookup_string<'a>(&'a self, str: &'a str) -> CharPeekableGlyphIterator<core::str::Chars<'a>> {
        self.lookup_peekable(str.chars())
    }
}