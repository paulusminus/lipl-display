use std::ops::Add;

#[derive(Clone)]
pub struct FontSize {
    size: i32,
}

impl FontSize {
    pub fn value(&self) -> i32 {
        self.size
    }

    pub fn set(&mut self, size: i32) {
        self.size = size;
    }
}

impl Add<i32> for FontSize {
    type Output = FontSize;

    fn add(self, other: i32) -> FontSize {
        FontSize {
            size: self.size + other,
        }
    }
}

impl From<i32> for FontSize {
    fn from(size: i32) -> Self {
        FontSize { size }
    }
}

impl std::fmt::Display for FontSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.size)
    }
}
