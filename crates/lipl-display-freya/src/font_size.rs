use std::ops::Add;

pub struct FontSize {
    size: usize,
}

impl FontSize {
    pub fn value(&self) -> usize {
        self.size
    }
}

impl Add<usize> for FontSize {
    type Output = FontSize;

    fn add(self, other: usize) -> FontSize {
        FontSize {
            size: self.size + other,
        }
    }
}

impl From<usize> for FontSize {
    fn from(size: usize) -> Self {
        FontSize { size }
    }
}

impl std::fmt::Display for FontSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.size)
    }
}
