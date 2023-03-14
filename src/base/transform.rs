
pub type Vec2f = nalgebra::Vector2<f32>;
pub type Mat2f = nalgebra::Matrix2<f32>;

//#[derive(Clone, Copy)]
pub enum Transform2d {
    /// The identity transform
    Identity,

    /// The Transform is a pure homogeneous scale
    Scale(f32),

    /// The transform is a non-homogeneous scale with possibly different factors for <i-math>\\hat{x}</i-math> and <i-math>\\hat{y}</i-math>
    XYScale {
        x_factor: f32,
        y_factor: f32,
    },

    /// The transform is non-homogeneous scale
    NonHomogeneousScale(Mat2f),

    /// The transform is an affine transform
    Affine {
        linear: Mat2f,
        translation: Vec2f,
    },

    // The transform is a general transform that preserves lines
    GeneralLinesPreserving(Box<dyn Fn(Vec2f) -> Vec2f>),

    // The transform is a general transform
    General(Box<dyn Fn(Vec2f) -> Vec2f>),
}

impl Transform2d {
    /// Applies the transform to a point
    pub fn eval(&self, point: Vec2f) -> Vec2f {
        match self {
            Transform2d::Identity => point,
            Transform2d::Scale(scale) => point * *scale,
            Transform2d::XYScale {
                x_factor,
                y_factor,
            } => Vec2f::new(point.x * *x_factor, point.y * *y_factor),
            Transform2d::NonHomogeneousScale(linear) => linear * point,
            Transform2d::Affine {
                linear,
                translation,
            } => linear * point + *translation,
            Transform2d::GeneralLinesPreserving(transform) => transform(point),
            Transform2d::General(transform) => transform(point),
        }
    }

    // tells if the transform preserves lines
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
}