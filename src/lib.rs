use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Font {
    pub bitmap: Vec<u8>,
    pub width: u32,
    pub map16: Vec<Glyph>,
    pub dict32: BTreeMap<[u16; 2], Glyph>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Glyph {
    pub pos: (u32, u32),
    pub size: (u8, u8),
    pub offset: (i8, i8),
}

impl Font {
    pub fn lookup(&self, str: &[u16], pos: usize) -> Option<(Glyph, bool)> {
        if pos < str.len() {
            if pos + 1 < str.len() {
                self.dict32.get(&[str[pos], str[pos + 1]]).copied()
                    .map(|g| (g, true))
                    .or_else(|| self.map16.get(str[pos] as usize).copied()
                        .map(|g| (g, false)))
            } else {
                self.map16.get(str[pos] as usize).copied().map(|g| (g, false))
            }
        } else {
            None
        }
    }
}