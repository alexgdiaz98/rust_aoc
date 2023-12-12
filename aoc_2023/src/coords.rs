use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Coord2D(pub isize, pub isize);

impl Add for Coord2D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Coord2D {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Coord2D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl SubAssign for Coord2D {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Display for Coord2D {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

impl Coord2D {
    #[allow(dead_code)]
    pub fn manhattan_distance(&self, rhs: &Self) -> isize {
        (self.0 - rhs.0).abs() + (self.1 - rhs.1).abs()
    }

    #[allow(dead_code)]
    pub fn magnitude(&self) -> isize {
        self.0.abs() + self.1.abs()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Coord3D(pub isize, pub isize, pub isize);

impl Add for Coord3D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + self.2)
    }
}

impl AddAssign for Coord3D {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Coord3D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl SubAssign for Coord3D {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Display for Coord3D {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.0, self.1, self.2)
    }
}

impl Coord3D {
    #[allow(dead_code)]
    pub fn manhattan_distance(&self, rhs: &Self) -> isize {
        (self.0 - rhs.0).abs() + (self.1 - rhs.1).abs() + (self.2 - rhs.2).abs()
    }

    #[allow(dead_code)]
    pub fn magnitude(&self) -> isize {
        self.0.abs() + self.1.abs() + self.2.abs()
    }
}