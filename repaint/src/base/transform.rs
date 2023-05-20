
use crate::base::defs::linalg::*;

//#[derive(Clone, Copy)]
pub enum Transform2d {
    /// The identity transform
    Identity,

    /// The Transform is a pure homogeneous scale
    Scale(f64),

    /// The transform is a non-homogeneous scale with possibly different factors for <i-math>\\hat{x}</i-math> and <i-math>\\hat{y}</i-math>
    XYScale {
        x_factor: f64,
        y_factor: f64,
    },

    /// The transform is non-homogeneous scale
    NonHomogeneousScale(Mat2f64),

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
            Transform2d::Scale(scale) => point * *scale,
            Transform2d::XYScale {
                x_factor,
                y_factor,
            } => Vec2f64::new(point.x * *x_factor, point.y * *y_factor),
            Transform2d::NonHomogeneousScale(linear) => linear * point,
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
            Transform2d::Scale(_) => true,
            Transform2d::XYScale { .. } => true,
            Transform2d::NonHomogeneousScale(_) => true,
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
            Transform2d::NonHomogeneousScale(linear) => Some((
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

    pub fn inverse(&self) -> Option<Transform2d> {
        unimplemented!()
    }
}

fn scaling_matrix2(x_scale: f64, y_scale: f64) -> Mat2f64 {
    let mut m = Mat2f64::zeros();
    m[(0, 0)] = x_scale;
    m[(1, 1)] = y_scale;
    m
}