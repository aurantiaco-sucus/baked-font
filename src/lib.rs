#![no_std]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
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