use std::{
    cmp::Ordering,
    ops::{Add, Deref, DerefMut},
};

use ratatui::layout::Position;

#[derive(Debug, Clone)]
pub struct Coordinate(pub Position);

impl Add for Coordinate {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Coordinate {
            0: Position {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            },
        }
    }
}

impl Into<Position> for Coordinate {
    fn into(self) -> Position {
        self.0
    }
}

impl From<(u16, u16)> for Coordinate {
    fn from(c: (u16, u16)) -> Self {
        Coordinate {
            0: Position { x: c.0, y: c.1 },
        }
    }
}

impl From<(usize, usize)> for Coordinate {
    fn from(c: (usize, usize)) -> Self {
        Coordinate {
            0: Position {
                x: c.0 as u16,
                y: c.1 as u16,
            },
        }
    }
}

impl Deref for Coordinate {
    type Target = Position;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Coordinate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl PartialOrd for Coordinate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Coordinate {}

impl Ord for Coordinate {
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare y first. If they are equal, cmp() falls back to comparing x.
        self.y.cmp(&other.y).then_with(|| self.x.cmp(&other.x))
    }
}

impl Default for Coordinate {
    fn default() -> Self {
        Self {
            0: Position::default(),
        }
    }
}

#[derive(Debug)]
pub struct CursorPos {
    line_number: usize,
    pub max_line_number: usize,
    character: usize,
    pub max_character: usize,
    pos: Coordinate,
}

#[derive(Debug)]
pub struct Highlight {
    pub p1: Coordinate,
    pub p2: Coordinate,
}

#[derive(Debug)]
pub struct DynamicText {
    pub position: CursorPos,
    pub raw_text: Vec<Vec<char>>,
    pub highlight: Highlight, // Highlight beginning coordinate (start, end) (y, x)
}

impl CursorPos {
    fn default() -> Self {
        Self {
            line_number: 0,
            max_line_number: 1,
            character: 0,
            max_character: 1,
            pos: Coordinate {
                0: Position { x: 0, y: 0 },
            },
        }
    }
    fn can_move_left(&self) -> bool {
        if self.character == 0 { false } else { true }
    }
    fn can_move_down(&self) -> bool {
        if self.line_number == self.max_line_number - 1 {
            false
        } else {
            true
        }
    }
    fn can_move_up(&self) -> bool {
        if self.line_number == 0 { false } else { true }
    }
    fn can_move_right(&self) -> bool {
        if self.character == self.max_character - 1 {
            false
        } else {
            true
        }
    }
    fn left(&mut self, prev_line_len: usize) {
        if self.can_move_left() {
            self.character -= 1;
            self.pos.x = self.character as u16;
        } else if self.can_move_up() {
            self.line_number -= 1;
            self.max_character = prev_line_len + 1;
            self.character = prev_line_len;
            self.pos = Coordinate::from((self.character as u16, self.line_number as u16));
        }
    }
    fn down(&mut self) {
        if self.can_move_down() {
            self.line_number += 1;
            self.pos.y = self.line_number as u16;
        } else if self.can_move_right() {
            self.character = self.max_character - 1;
            self.pos.x = self.character as u16;
        }
    }
    fn up(&mut self) {
        if self.can_move_up() {
            self.line_number -= 1;
            self.pos.y = self.line_number as u16;
        } else if self.can_move_left() {
            self.character = 0;
            self.pos.x = self.character as u16;
        }
    }
    fn right(&mut self, next_line_len: usize) {
        if self.can_move_right() {
            self.character += 1;
            self.pos.x = self.character as u16;
        } else if self.can_move_down() {
            self.line_number += 1;
            self.max_character = next_line_len + 1;
            self.character = 0;
            self.pos = Coordinate::from((self.character as u16, self.line_number as u16));
        }
    }

    fn max_left(&mut self) {
        self.character = 0;
        self.pos.x = self.character as u16;
    }
    fn max_down(&mut self) {
        self.line_number = self.max_line_number - 1;
        self.pos.y = self.line_number as u16;
    }
    fn max_up(&mut self) {
        self.line_number = 0;
        self.pos.y = self.line_number as u16;
    }
    fn max_right(&mut self) {
        if self.max_character != 1 {
            self.character = self.max_character - 1;
            self.pos.x = self.character as u16;
        }
    }

    fn closest_left(&mut self) {
        if self.character >= self.max_character {
            self.character = self.max_character - 1;
            self.pos.x = self.character as u16;
        }
    }
}

impl Highlight {
    pub fn default() -> Self {
        Self {
            p1: Coordinate::default(),
            p2: Coordinate::default(),
        }
    }

    fn select_left(&mut self, cur: &CursorPos) {
        // Expand selection to the left
        if self.p1 > cur.pos {
            self.p1 = cur.pos.clone();
        } else if self.p2 > cur.pos {
            self.p2 = cur.pos.clone();
        }
        // Shrink selection from the right
    }
    fn select_down(&mut self, cur: &CursorPos) {
        // Expand selection downward
        if self.p2 < cur.pos {
            self.p2 = cur.pos.clone();
        } else if self.p1 < cur.pos {
            self.p1 = cur.pos.clone();
        }
        // Shrink selection from the top
    }
    fn select_up(&mut self, cur: &CursorPos) {
        // Expand selection upward
        if self.p1 > cur.pos {
            self.p1 = cur.pos.clone();
        } else if self.p2 > cur.pos {
            self.p2 = cur.pos.clone();
        }
        // Shrink selection from the bottom
    }
    fn select_right(&mut self, cur: &CursorPos) {
        // Expand selection to the right
        if self.p2 < cur.pos {
            self.p2 = cur.pos.clone();
        } else if self.p1 < cur.pos {
            self.p1 = cur.pos.clone();
        }
        // Shrink selection from the left
    }
}

impl DynamicText {
    pub fn default() -> Self {
        let mut r = Vec::new();
        r.push(Vec::new());
        Self {
            position: CursorPos::default(),
            raw_text: r,
            highlight: Highlight::default(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.raw_text
            .clone()
            .into_iter()
            .flat_map(|mut inner: Vec<char>| {
                inner.push('\n');
                let t: Vec<u8> = inner.into_iter().map(|c| c as u8).collect();
                t
            })
            .collect()
    }

    pub fn to_string(&self) -> String {
        let v = self.to_bytes();
        String::from_utf8(v).unwrap()
    }

    pub fn pop(&mut self) -> Option<char> {
        if self.position.can_move_left() {
            let c = self.raw_text[self.position.line_number].remove(self.position.character - 1);
            self.position.left(0);
            self.position.max_character -= 1;
            return Some(c);
        } else if self.position.can_move_up() {
            let c = '\n';
            let mut cur_row = self.raw_text[self.position.line_number].clone();
            self.position.character = self.raw_text[self.position.line_number - 1].len();
            self.raw_text[self.position.line_number - 1].append(&mut cur_row);
            self.raw_text.remove(self.position.line_number);

            self.position.max_line_number -= 1;
            self.position.up();
            self.position.max_character = self.raw_text[self.position.line_number].len() + 1;

            return Some(c);
        }
        None
    }

    pub fn push(&mut self, c: char) {
        if c == '\n' {
            if self.position.can_move_right() {
                let (remaining, newline) =
                    self.raw_text[self.position.line_number].split_at_mut(self.position.character);
                let r = remaining.to_vec().clone();
                let n = newline.to_vec().clone();
                self.position.max_character = n.len();
                self.raw_text[self.position.line_number] = r;
                self.raw_text.insert(self.position.line_number + 1, n);
                self.position.max_line_number += 1;
                self.position.down();
                self.position.max_left();
            } else {
                self.raw_text
                    .insert(self.position.line_number + 1, Vec::new());
                self.position.max_character = 1;
                self.position.max_line_number += 1;
                self.position.down();
                self.position.max_left();
            }
        } else {
            self.raw_text[self.position.line_number].insert(self.position.character, c);
            self.position.max_character += 1;
            self.position.right(0);
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = String> {
        self.raw_text
            .iter()
            .map(|v| v.iter().map(|c| *c).collect::<String>())
    }

    pub fn pos(&self) -> Coordinate {
        self.position.pos.clone()
    }

    pub fn move_left(&mut self) {
        if self.position.can_move_up() {
            self.position
                .left(self.raw_text[self.position.line_number - 1].len());
        } else {
            self.position.left(0);
        }
    }
    pub fn move_down(&mut self) {
        if self.position.can_move_down() {
            self.position.max_character = self.raw_text[self.position.line_number + 1].len() + 1;
            self.position.closest_left();
        }
        self.position.down();
    }
    pub fn move_up(&mut self) {
        if self.position.can_move_up() {
            self.position.max_character = self.raw_text[self.position.line_number - 1].len() + 1;
            self.position.closest_left();
        }
        self.position.up();
    }
    pub fn move_right(&mut self) {
        if self.position.can_move_down() {
            self.position
                .right(self.raw_text[self.position.line_number + 1].len());
        } else {
            self.position.right(0);
        }
    }

    pub fn select_left(&mut self) {
        self.move_left();
        self.highlight.select_left(&self.position);
    }
    pub fn select_down(&mut self) {
        self.move_down();
        self.highlight.select_down(&self.position);
    }
    pub fn select_up(&mut self) {
        self.move_up();
        self.highlight.select_up(&self.position);
    }
    pub fn select_right(&mut self) {
        self.move_right();
        self.highlight.select_right(&self.position);
    }

    pub fn null_highlight(&mut self) {
        self.highlight = Highlight {
            p1: Coordinate::from((self.position.character, self.position.line_number)),
            p2: Coordinate::from((self.position.character, self.position.line_number)),
        }
    }
}
