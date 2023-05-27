
use crate::base::defs::linalg::*;

use nalgebra::{Matrix4, Rotation2};

//#[derive(Clone, Copy)]
pub enum Transform2d {
    /// The identity transform
    Identity,

    Translate(Vec2f64),

    Rotate(f64),

    /// The Transform is a pure homogeneous scale
    Scale(f64),

    /// The transform is a non-homogeneous scale with possibly different factors for <i-math>\\hat{x}</i-math> and <i-math>\\hat{y}</i-math>
    XYScale {
        x_factor: f64,
        y_factor: f64,
    },

    /// The transform is non-homogeneous scale
    Mat2(Mat2f64),

    /// The transform is an affine transform
    Affine {
        linear: Mat2f64,
        translation: Vec2f64,
    },

    // The transform is a general transform that preserves lines
    GeneralLinesPreserving(Box<dyn Fn(Vec2f64) -> Vec2f64>),

    // The transform is a general transform
    General(Box<dyn Fn(Vec2f64) -> Vec2f64>),
}

impl Transform2d {
    /// Applies the transform to a point
    pub fn eval(&self, point: Vec2f64) -> Vec2f64 {
        match self {
            Transform2d::Identity => point,
            Transform2d::Translate(translation) => point + *translation,
            Transform2d::Rotate(angle) => Rotation2::new(*angle).matrix() * point,
            Transform2d::Scale(scale) => point * *scale,
            Transform2d::XYScale {
                x_factor,
                y_factor,
            } => Vec2f64::new(point.x * *x_factor, point.y * *y_factor),
            Transform2d::Mat2(linear) => linear * point,
            Transform2d::Affine {
                linear,
                translation,
            } => linear * point + *translation,
            Transform2d::GeneralLinesPreserving(transform) => transform(point),
            Transform2d::General(transform) => transform(point),
        }
    }

    /// Tells if the transform preserves lines.
    pub fn is_line_preserving(&self) -> bool {
        match self {
            Transform2d::Identity => true,
            Transform2d::Translate(_) => true,
            Transform2d::Rotate(_) => false,
            Transform2d::Scale(_) => true,
            Transform2d::XYScale { .. } => true,
            Transform2d::Mat2(_) => true,
            Transform2d::Affine { .. } => true,
            Transform2d::GeneralLinesPreserving(_) => true,
            Transform2d::General(_) => false,
        }
    }

    /// Returns the linear part and the translation part of the transform
    /// if the transform is affine.
    pub fn to_affine(&self) -> Option<(Mat2f64, Vec2f64)> {
        match self {
            Transform2d::Identity => Some((
                Mat2f64::identity(),
                Vec2f64::zeros()
            )),
            Transform2d::Translate(translation) => Some((
                Mat2f64::identity(),
                *translation
            )),
            Transform2d::Rotate(angle) => Some((
                *Rotation2::new(*angle).matrix(),
                Vec2f64::zeros()
            )),
            Transform2d::Scale(scale) => Some((
                scaling_matrix2(*scale, *scale),
                Vec2f64::zeros()
            )),
            Transform2d::XYScale {
                x_factor,
                y_factor,
            } => Some((
                //Mat2f::new_nonuniform_scaling(&Vec2f::new(*x_factor, *y_factor)),
                scaling_matrix2(*x_factor, *y_factor),
                Vec2f64::zeros(),
            )),
            Transform2d::Mat2(linear) => Some((
                *linear,
                Vec2f64::zeros()
            )),
            Transform2d::Affine {
                linear,
                translation,
            } => Some((
                *linear,
                *translation
            )),
            Transform2d::GeneralLinesPreserving(_) => None,
            Transform2d::General(_) => None,
        }
    }

    pub fn to_mat4x4(&self) -> Option<Matrix4<f64>> {
        if let Some((linear, translation)) = self.to_affine() {
            Some(Matrix4::new(
                linear[(0, 0)], linear[(0, 1)], 0.0, translation.x,
                linear[(1, 0)], linear[(1, 1)], 0.0, translation.y,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0
            ))
        } else {
            None
        }
    }

    pub fn inverse(&self) -> Option<Transform2d> {
        unimplemented!()
    }

    // TODO composition
}

fn scaling_matrix2(x_scale: f64, y_scale: f64) -> Mat2f64 {
    let mut m = Mat2f64::zeros();
    m[(0, 0)] = x_scale;
    m[(1, 1)] = y_scale;
    m
}