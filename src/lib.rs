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

    pub fn lookup(&self, str: &[char], pos: usize) -> Option<(Glyph, bool)> {
        if pos < str.len() {
            if pos + 1 < str.len() {
                self.map2.get(&[str[pos], str[pos + 1]]).copied()
                    .map(|g| (g, true))
                    .or_else(|| self.map1.get(&str[pos]).copied()
                        .map(|g| (g, false)))
            } else {
                self.map1.get(&str[pos]).copied().map(|g| (g, false))
            }
        } else {
            None
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
    type Item = Glyph;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.str.len() {
            return None;
        }
        let mut res = self.font.lookup(self.str, self.pos);
        while res.is_none() && self.pos < self.str.len() {
            self.pos += 1;
            res = self.font.lookup(self.str, self.pos);
        }
        if let Some((g, b)) = res {
            self.pos += if b { 2 } else { 1 };
            Some(g)
        } else {
            None
        }
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
    type Item = Glyph;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.iter.next();
        let next = self.iter.peek().copied();
        if let Some(cc) = cur {
            if let Some(cn) = next {
                let res = self.font.lookup_double(cc, cn);
                if res.is_none() {
                    let res = self.font.lookup_single(cc);
                    if res.is_some() {
                        return res;
                    }
                    self.next()
                } else {
                    res
                }
            } else {
                let res = self.font.lookup_single(cc);
                if res.is_some() {
                    return res;
                }
                self.next()
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

    pub fn lookup_string<'a>(&'a self, str: &'a str) -> CharPeekableGlyphIterator<impl Iterator<Item=char> + 'a> {
        self.lookup_peekable(str.chars())
    }
}