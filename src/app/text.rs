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
        match (self.x.partial_cmp(&other.x), self.y.partial_cmp(&other.y)) {
            (Some(Ordering::Equal), y_ord) => y_ord,
            (x_ord, Some(Ordering::Equal)) => x_ord,
            (Some(x_ord), Some(y_ord)) if x_ord == y_ord => Some(x_ord),
            _ => None,
        }
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
    max_line_number: usize,
    character: usize,
    max_character: usize,
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
    raw_text: Vec<Vec<char>>,
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
    fn left(&mut self) {
        if self.can_move_left() {
            self.character -= 1;
            self.pos.x = self.character as u16;
        }
    }
    fn down(&mut self) {
        if self.can_move_down() {
            self.line_number += 1;
            self.pos.y = self.line_number as u16;
        }
    }
    fn up(&mut self) {
        if self.can_move_up() {
            self.line_number -= 1;
            self.pos.y = self.line_number as u16;
        }
    }
    fn right(&mut self) {
        if self.can_move_right() {
            self.character += 1;
            self.pos.x = self.character as u16;
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

    pub fn null_highlight(&mut self) -> Highlight {
        Highlight {
            p1: Coordinate::from((self.character, self.line_number)),
            p2: Coordinate::from((self.character, self.line_number)),
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
        if cur.pos > self.p1 {
            return;
        } else {
            self.p1 = cur.pos.clone();
        }
    }
    fn select_down(&mut self, cur: &CursorPos) {
        if cur.pos < self.p2 {
            return;
        } else {
            self.p2 = cur.pos.clone();
        }
    }
    fn select_up(&mut self, cur: &CursorPos) {
        if cur.pos > self.p1 {
            return;
        } else {
            self.p1 = cur.pos.clone();
        }
    }
    fn select_right(&mut self, cur: &CursorPos) {
        if cur.pos < self.p2 {
            return;
        } else {
            self.p2 = cur.pos.clone();
        }
    }

    fn unselect_left(&mut self, cur: &CursorPos) {
        if cur.pos > self.p2 {
            return;
        } else {
            self.p2 = cur.pos.clone();
        }
    }
    fn unselect_down(&mut self, cur: &CursorPos) {
        if cur.pos < self.p1 {
            return;
        } else {
            self.p1 = cur.pos.clone();
        }
    }
    fn unselect_up(&mut self, cur: &CursorPos) {
        if cur.pos > self.p2 {
            return;
        } else {
            self.p2 = cur.pos.clone();
        }
    }
    fn unselect_right(&mut self, cur: &CursorPos) {
        if cur.pos < self.p1 {
            return;
        } else {
            self.p1 = cur.pos.clone();
        }
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

    pub fn pop(&mut self) -> Option<char> {
        if self.position.can_move_left() {
            let c = self.raw_text[self.position.line_number].remove(self.position.character - 1);
            self.position.left();
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
            self.position.right();
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
        self.position.left();
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
        self.position.right();
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

    pub fn unselect_left(&mut self) {
        self.move_left();
        self.highlight.unselect_left(&self.position);
    }
    pub fn unselect_down(&mut self) {
        self.move_down();
        self.highlight.unselect_down(&self.position);
    }
    pub fn unselect_up(&mut self) {
        self.move_up();
        self.highlight.unselect_up(&self.position);
    }
    pub fn unselect_right(&mut self) {
        self.move_right();
        self.highlight.unselect_right(&self.position);
    }
}
