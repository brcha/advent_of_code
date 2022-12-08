use std::fmt;
use std::fmt::{Display, Formatter};
use std::io;
use std::iter::zip;
use std::ops::{Index, IndexMut};

#[derive(Debug)]
struct Matrix<T: PartialOrd+Copy+PartialEq+Default+Display> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: PartialOrd+Copy+PartialEq+Default+Display> Matrix<T> {
    fn new() -> Self {
        Self {
            data: Vec::new(),
            width: 0,
            height: 0,
        }
    }

    fn push_value(&mut self, value: T) {
        self.data.push(value);
    }

    fn check_row(&self, row: usize) -> Vec<bool> {
        let mut maximum_left = self[(row, 0)];
        let mut hidden_left: Vec<bool> = (0..self.width).map(|_| false).collect();

        for idx in 1 .. self.width-1 {
            if self[(row, idx)] <= maximum_left {
                hidden_left[idx] = true;
            } else {
                maximum_left = self[(row, idx)];
                hidden_left[idx] = false;
            }
        }

        let mut maximum_right = self[(row, self.width - 1)];
        let mut hidden_right: Vec<bool> = (0..self.width).map(|_| false).collect();

        for idx in (1 .. self.width-1).rev() {
            if self[(row, idx)] <= maximum_right {
                hidden_right[idx] = true;
            } else {
                maximum_right = self[(row, idx)];
                hidden_right[idx] = false;
            }
        }

        hidden_left.iter().zip(&hidden_right)
            .map(|(l, r)| l & r)
            .collect()
    }

    fn check_column(&self, col: usize) -> Vec<bool> {
        let mut maximum_top = self[(0, col)];
        let mut hidden_top: Vec<bool> = (0..self.height).map(|_| false).collect();

        for idx in 1 .. self.height-1 {
            if self[(idx, col)] <= maximum_top {
                hidden_top[idx] = true;
            } else {
                maximum_top = self[(idx, col)];
                hidden_top[idx] = false;
            }
        }

        let mut maximum_bottom = self[(self.height-1, col)];
        let mut hidden_bottom: Vec<bool> = (0..self.height).map(|_| false).collect();

        for idx in (1 .. self.height-1).rev() {
            if self[(idx, col)] <= maximum_bottom {
                hidden_bottom[idx] = true;
            } else {
                maximum_bottom = self[(idx, col)];
                hidden_bottom[idx] = false;
            }
        }

        hidden_top.iter().zip(&hidden_bottom)
            .map(|(l, r)| l & r)
            .collect()
    }

    fn transpose(&self) -> Matrix<T> {
        let mut out = Matrix::new();
        out.width = self.height;
        out.height = self.width;
        out.data = (0..self.width*self.height).map(|_| Default::default()).collect();

        for row in 0..self.height {
            for col in 0..self.width {
                out[(col, row)] = self[(row, col)];
            }
        }

        out
    }

    fn check_all(&self) -> Matrix<bool> {
        let mut rows = Matrix::new();
        rows.width = self.width;
        rows.height = self.height;

        let mut false_row: Vec<bool> = (0..self.width).map(|_| false).collect();
        rows.data.append(&mut false_row); // for the first row
        for idx in 1..self.height-1 {
            let mut row = self.check_row(idx);
            rows.data.append(&mut row);
        }
        let mut false_row: Vec<bool> = (0..self.width).map(|_| false).collect();
        rows.data.append(&mut false_row); // for the last row

        let mut cols = Matrix::new();
        cols.width = self.height;
        cols.height = self.width;

        let mut false_col: Vec<bool> = (0..self.height).map(|_| false).collect();
        cols.data.append(&mut false_col);
        for idx in 1..self.width-1 {
            let mut col = self.check_column(idx);
            cols.data.append(&mut col);
        }
        let mut false_col: Vec<bool> = (0..self.height).map(|_| false).collect();
        cols.data.append(&mut false_col);

        let cols = cols.transpose();

        let mut result: Matrix<bool> = zip(rows, cols)
            .map(|(r, c)| r & c)
            .collect();
        result.width = self.width;
        result.height = self.height;

        result
    }

    fn visibility_up(&self, row: usize, col: usize) -> u64 {
        let mut up = 0;

        for idx in (0..row).rev() {
            up += 1;
            if self[(idx, col)] >= self[(row, col)] {
                break;
            }
        }

        up
    }

    fn visibility_down(&self, row: usize, col: usize) -> u64 {
        let mut down = 0;

        for idx in row+1..self.height {
            down += 1;
            if self[(idx, col)] >= self[(row, col)] {
                break;
            }
        }

        down
    }

    fn visibility_left(&self, row: usize, col: usize) -> u64 {
        let mut left = 0;

        for idx in (0..col).rev() {
            left += 1;
            if self[(row, idx)] >= self[(row, col)] {
                break;
            }
        }

        left
    }

    fn visibility_right(&self, row: usize, col: usize) -> u64 {
        let mut right = 0;

        for idx in col+1..self.width {
            right += 1;
            if self[(row, idx)] >= self[(row, col)] {
                break;
            }
        }

        right
    }

    fn compute_visibility(&self) -> Matrix<u64> {
        let mut v = Matrix::new();
        v.height = self.height;
        v.width = self.width;
        v.data = (0..self.width*self.height).map(|_| Default::default()).collect();

        for row in 0..self.height {
            for col in 0..self.width {
                let up = self.visibility_up(row, col);
                let down = self.visibility_down(row, col);
                let left = self.visibility_left(row, col);
                let right = self.visibility_right(row, col);

                v[(row, col)] = up * down * left * right;
            }
        }

        v
    }
}

impl<T: PartialOrd+Copy+PartialEq+Default+Display> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.data[self.width * row + col]
    }
}

impl<T: PartialOrd+Copy+PartialEq+Default+Display> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.data[self.width * row + col]
    }
}

impl<T: PartialOrd+Copy+PartialEq+Default+Display> IntoIterator for Matrix<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<T: PartialOrd+Copy+PartialEq+Default+Display> FromIterator<T> for Matrix<T> {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        let mut m = Matrix::new();

        for i in iter {
            m.push_value(i);
        }

        m
    }
}

impl<T: PartialOrd+Copy+PartialEq+Default+Display> Display for Matrix<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut res = String::new();
        for row in 0..self.height {
            res.push_str("[");
            for col in 0..self.width {
                let value = format!(" {} ", self[(row, col)]);
                res.push_str(&value);
            }
            res.push_str("]\n");
        }
        write!(f, "{}", res)
    }
}

fn main() {
    let mut forrest = Matrix::new();

    for line in io::stdin().lines() {
        let input = line.unwrap();

        if forrest.width == 0 {
            forrest.width = input.len();
        }
        forrest.height += 1;

        for c in input.chars() {
            forrest.push_value(c.to_digit(10).unwrap());
        }
    }

    let checked = forrest.check_all();

    let num_visible_trees = checked.into_iter().filter(|x| !*x).count();

    println!("Number of visible trees is {}", num_visible_trees);

    let visibility = forrest.compute_visibility();


    let max_visibility = visibility.into_iter().max().unwrap();

    println!("Maximum visibility is {}", max_visibility);
}
