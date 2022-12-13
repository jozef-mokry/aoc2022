use std::collections::HashSet;
use std::ops::{Add, AddAssign, Sub};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Point(isize, isize);

impl Point {
    fn clamp(&self) -> Self {
        if self.0.abs() == 2 || self.1.abs() == 2 {
            Point(self.0 / 2, self.1 / 2)
        } else {
            *self
        }
    }
}

impl From<&str> for Point {
    fn from(s: &str) -> Self {
        match s {
            "D" => Point(1, 0),
            "U" => Point(-1, 0),
            "L" => Point(0, -1),
            "R" => Point(0, 1),
            _ => unreachable!(),
        }
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

fn main() {
    let mut head = Point(0, 0);
    // each tail is position RELATIVE to previous tail (or HEAD for 1st tail)
    let mut tails = [Point(0, 0); 9]; // change to 1 for part1
    let mut seen: HashSet<Point> = HashSet::new();
    seen.insert(Point(0, 0));
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let mut parts = line.split_whitespace();
        let (dir, count) = (
            parts.next().unwrap().into(),
            parts.next().unwrap().parse::<usize>().unwrap(),
        );
        for _ in 0..count {
            head += dir;
            let mut pos = head;
            let mut prev_step = dir;
            for tail in &mut tails {
                let new_tail = (*tail - prev_step).clamp();
                prev_step = new_tail - (*tail - prev_step);
                *tail = new_tail;
                pos += *tail;
            }
            seen.insert(pos);
        }
    }
    println!("{}", seen.len());
}
