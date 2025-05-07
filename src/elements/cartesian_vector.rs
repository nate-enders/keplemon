use super::SphericalVector;
use nalgebra::Vector3;
use pyo3::prelude::*;
use std::ops::{Add, Index, IndexMut, Mul, Sub};

#[pyclass]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CartesianVector {
    n_vector: Vector3<f64>,
}

impl Mul<f64> for CartesianVector {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        Self {
            n_vector: self.n_vector * scalar,
        }
    }
}

impl Add for CartesianVector {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            n_vector: self.n_vector + other.n_vector,
        }
    }
}

impl Sub for CartesianVector {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            n_vector: self.n_vector - other.n_vector,
        }
    }
}

#[pymethods]
impl CartesianVector {
    #[new]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            n_vector: Vector3::new(x, y, z),
        }
    }

    fn __add__(&self, other: &Self) -> Self {
        Self {
            n_vector: self.n_vector + other.n_vector,
        }
    }

    fn __sub__(&self, other: &Self) -> Self {
        Self {
            n_vector: self.n_vector - other.n_vector,
        }
    }

    #[getter]
    fn x(&self) -> f64 {
        self.n_vector.x
    }

    #[getter]
    fn y(&self) -> f64 {
        self.n_vector.y
    }

    #[getter]
    fn z(&self) -> f64 {
        self.n_vector.z
    }

    #[getter]
    pub fn get_magnitude(&self) -> f64 {
        self.n_vector.norm()
    }

    pub fn distance(&self, other: &Self) -> f64 {
        (self.n_vector - other.n_vector).norm()
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.n_vector.dot(&other.n_vector)
    }

    pub fn angle(&self, other: &Self) -> f64 {
        let dot_product = self.dot(other);
        let magnitude_product = self.get_magnitude() * other.get_magnitude();
        if magnitude_product == 0.0 {
            return 0.0;
        }
        (dot_product / magnitude_product).acos()
    }

    pub fn to_spherical(&self) -> SphericalVector {
        let r = self.get_magnitude();
        let ra = self.n_vector.y.atan2(self.n_vector.x);
        let dec = self
            .n_vector
            .z
            .atan2((self.n_vector.x * self.n_vector.x + self.n_vector.y * self.n_vector.y).sqrt());
        SphericalVector::new(r, ra.to_degrees(), dec.to_degrees())
    }
}

impl From<[f64; 3]> for CartesianVector {
    fn from(vec: [f64; 3]) -> Self {
        Self::new(vec[0], vec[1], vec[2])
    }
}

impl From<CartesianVector> for [f64; 3] {
    fn from(cartesian_vector: CartesianVector) -> Self {
        [cartesian_vector.x(), cartesian_vector.y(), cartesian_vector.z()]
    }
}

impl Index<usize> for CartesianVector {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.n_vector[0],
            1 => &self.n_vector[1],
            2 => &self.n_vector[2],
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for CartesianVector {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.n_vector[0],
            1 => &mut self.n_vector[1],
            2 => &mut self.n_vector[2],
            _ => panic!("Index out of bounds"),
        }
    }
}
