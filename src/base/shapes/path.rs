
type Vec2f = nalgebra::Vector2<f32>;
type Mat2f = nalgebra::Matrix2<f32>;

use std::ops::{
    Mul,
    Sub,
    Add,
    //Div
};

use crate::base::{shapes::polyline::BrokenPolylineCommand, defs::rect::FRect};

/// A command for a path.
/// 
/// A path is a sequence of commands that are executed in order to form a shape.
/// For example, a rectangle can be created by moving to the top left corner,
/// then drawing a line to the top right corner, then drawing a line to the bottom
/// right corner, then drawing a line to the bottom left corner, and finally closing.
/// It would be represented, for example, by the following commands:
/// ```rs
/// PathCommand::MoveTo(Vec2f::new(0.0, 0.0));
/// PathCommand::LineTo(Vec2f::new(2.0, 0.0));
/// PathCommand::LineTo(Vec2f::new(2.0, 1.0));
/// PathCommand::LineTo(Vec2f::new(0.0, 1.0));
/// PathCommand::ClosePath;
/// ```
pub enum PathCommand {

    /// Move to a point, effectively starting a new path.
    /// 
    /// This command should be used at the beginning of a path, after a [`ClosePath`] command
    /// of when you want to start a new path. If you do not use this command at the beginning,
    /// the path will be implicitly moved to the origin; if you do not use this command after
    /// a [`ClosePath`] command, the path will be implicitly moved to the last point of the previous path.
    MoveTo(Vec2f),

    /// Move to a point relative to the current position.
    /// 
    /// This command is the same as [`PathCommand::MoveTo`], but the point is relative to the current position.
    MoveToOffset(Vec2f),

    /// Draw a line to a point.
    /// 
    /// This command draws a straight line from the current position to the given point, the current position
    /// is then set to the given point.
    LineTo(Vec2f),

    /// Draw a line to a point relative to the current position.
    /// 
    /// This command is the same as [`PathCommand::LineTo`], but the point is relative to the current position.
    LineToOffset(Vec2f),

    /// Draw a horizontal line to a point.
    /// 
    /// This command draws a straight line from the current position to the given point, the current position
    /// is then set to the given point.
    HorizontalLineTo(f32),

    /// Draw a horizontal line to a point relative to the current position.
    /// 
    /// This command is the same as [`PathCommand::HorizontalLineTo`], but the point is relative to the current position.
    HorizontalLineToOffset(f32),

    /// Draw a vertical line to a point.
    /// 
    /// This command draws a straight line from the current position to the given point, the current position
    /// is then set to the given point.
    VerticalLineTo(f32),

    /// Draw a vertical line to a point relative to the current position.
    /// 
    /// This command is the same as [`PathCommand::VerticalLineTo`], but the point is relative to the current position.
    VerticalLineToOffset(f32),

    /// Close the current path with a straight line.
    /// 
    /// This command draws a straight line from the current position to the starting position of the path.
    /// The current position is then set to the starting position of the path.
    ClosePath,

    /// Draw a cubic Bezier curve to a point.
    CubicBezierCurveTo{
        /// The first control point.
        control_pt_1: Vec2f,
        /// The second control point.
        control_pt_2: Vec2f,
        /// The end point.
        end_pt: Vec2f,
    },

    /// Draw a cubic Bezier curve to a point relative to the current position.
    CubicBezierCurveToOffset{
        /// The first control point.
        control_pt_1_offset: Vec2f,
        /// The second control point.
        control_pt_2_offset: Vec2f,
        /// The end point.
        end_pt_offset: Vec2f,
    },

    /// Draw a cubic Bezier curve to a point.
    SmoothCubicBezierCurveTo{
        /// The control point.
        control_pt_2: Vec2f,
        /// The end point.
        end_pt: Vec2f,
    },

    /// Draw a cubic Bezier curve to a point relative to the current position.
    SmoothCubicBezierCurveToOffset{
        /// The control point.
        control_pt_2_offset: Vec2f,
        /// The end point.
        end_pt_offset: Vec2f,
    },

    /// Draw a quadratic Bezier curve to a point.
    QuadraticBezierCurveTo{
        /// The control point.
        control_pt: Vec2f,
        /// The end point.
        end_pt: Vec2f,
    },

    /// Draw a quadratic Bezier curve to a point relative to the current position.
    QuadraticBezierCurveToOffset{
        /// The control point.
        control_pt_offset: Vec2f,
        /// The end point.
        end_pt_offset: Vec2f,
    },

    /// Draw a quadratic Bezier curve to a point.
    /// 
    /// The last point is the end point.
    SmoothQuadraticBezierCurveTo(Vec2f),

    /// Draw a quadratic Bezier curve to a point relative to the current position.
    /// 
    /// The last point is the end point.
    SmoothQuadraticBezierCurveToOffset(Vec2f),

    /// Draw an elliptical arc to a point.
    /// 
    /// See https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial/Paths#arcs
    //EllipticalArcTo(f32, f32, f32, bool, bool, Vec2f),
    EllipticalArcTo{
        /// The x-axis and y-axis radii of the ellipse.
        radii: Vec2f,
        /// The x-axis-rotation attribute indicates how the ellipse as a whole is rotated relative to the current coordinate system.
        x_axis_rotation: f32,
        /// The large-arc-flag attribute indicates whether an arc should take the longer or shorter way around the ellipse to join its start and end points.
        large_arc_flag: bool,
        /// The sweep-flag attribute indicates whether the arc should be drawn in a "positive-angle" or "negative-angle" direction.
        sweep_flag: bool,
        /// The end point.
        end_pt: Vec2f,
    },

    /// Draw an elliptical arc to a point relative to the current position.
    /// 
    /// Format:
    /// ```plain
    /// rx ry x-axis-rotation large-arc-flag sweep-flag dx dy
    /// ```
    EllipticalArcToOffset{
        /// The x-axis and y-axis radii of the ellipse.
        radii: Vec2f,
        /// The x-axis-rotation attribute indicates how the ellipse as a whole is rotated relative to the current coordinate system.
        x_axis_rotation: f32,
        /// The large-arc-flag attribute indicates whether an arc should take the longer or shorter way around the ellipse to join its start and end points.
        large_arc_flag: bool,
        /// The sweep-flag attribute indicates whether the arc should be drawn in a "positive-angle" or "negative-angle" direction.
        sweep_flag: bool,
        /// The end point.
        end_pt_offset: Vec2f,
    },
}

/// This is an SVG style path command. It has a slightly different syntax than the
/// [`PathCommand`] enum and could lack some features, but it is easier to use.
/// 
/// See [MDN](https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial/Paths) reference for more information.
#[allow(non_camel_case_types)]
pub enum SvgStylePathCommand {

    /// Move to a point.
    /// 
    /// Format:
    /// ```plain
    /// M x y
    /// ```
    M(f32, f32),

    /// Move to a point relative to the current position.
    /// 
    /// Format:
    /// ```plain
    /// m dx dy
    /// ```
    m(f32, f32),

    /// Draw a line to a point.
    /// 
    /// Format:
    /// ```plain
    /// L x y
    /// ```
    L(f32, f32),

    /// Draw a line to a point relative to the current position.
    /// 
    /// Format:
    /// ```plain
    /// l dx dy
    /// ```
    l(f32, f32),

    /// Draw a horizontal line to a point.
    /// 
    /// Format:
    /// ```plain
    /// H x
    /// ```
    H(f32),

    /// Draw a horizontal line to a point relative to the current position.
    /// 
    /// Format:
    /// ```plain
    /// h dx
    /// ```
    h(f32),

    /// Draw a vertical line to a point.
    /// 
    /// Format:
    /// ```plain
    /// V y
    /// ```
    V(f32),

    /// Draw a vertical line to a point relative to the current position.
    /// 
    /// Format:
    /// ```plain
    /// v dy
    /// ```
    v(f32),

    /// Close the current path with a straight line.
    /// 
    /// Format:
    /// ```plain
    /// Z
    /// ```
    Z,

    /// Close the current path with a straight line.
    /// 
    /// Same as [`SvgStylePathCommand::Z`].
    z,

    /// Draw a cubic Bezier curve to a point.
    /// 
    /// Format:
    /// ```plain
    /// C x1 y1 x2 y2 x y
    /// ```
    C(f32, f32, f32, f32, f32, f32),

    /// Draw a cubic Bezier curve to a point relative to the current position.
    /// 
    /// Format:
    /// ```plain
    /// c dx1 dy1 dx2 dy2 dx dy
    /// ```
    c(f32, f32, f32, f32, f32, f32),

    /// Draw a cubic Bezier curve to a point.
    /// 
    /// Format:
    /// ```plain
    /// S x2 y2 x y
    /// ```
    S(f32, f32, f32, f32),

    /// Draw a cubic Bezier curve to a point relative to the current position.
    /// 
    /// Format:
    /// ```plain
    /// s dx2 dy2 dx dy
    /// ```
    s(f32, f32, f32, f32),

    /// Draw a quadratic Bezier curve to a point.
    /// 
    /// Format:
    /// ```plain
    /// Q x1 y1 x y
    /// ```
    Q(f32, f32, f32, f32),

    /// Draw a quadratic Bezier curve to a point relative to the current position.
    /// 
    /// Format:
    /// ```plain
    /// q dx1 dy1 dx dy
    /// ```
    q(f32, f32, f32, f32),

    /// Draw a quadratic Bezier curve to a point.
    /// 
    /// Format:
    /// ```plain
    /// T x y
    /// ```
    T(f32, f32),

    /// Draw a quadratic Bezier curve to a point relative to the current position.
    /// 
    /// Format:
    /// ```plain
    /// t dx dy
    /// ```
    t(f32, f32),

    /// Draw an elliptical arc to a point.
    /// 
    /// Format:
    /// ```plain
    /// A rx ry x-axis-rotation large-arc-flag sweep-flag x y
    /// ```
    A(f32, f32, f32, bool, bool, f32, f32),

    /// Draw an elliptical arc to a point relative to the current position.
    /// 
    /// Format:
    /// ```plain
    /// a rx ry x-axis-rotation large-arc-flag sweep-flag dx dy
    /// ```
    a(f32, f32, f32, bool, bool, f32, f32),
}

impl SvgStylePathCommand {
    /// Convert the SVG style path command to a [`PathCommand`].
    pub fn to_path_command(&self) -> PathCommand {
        match self {
            SvgStylePathCommand::M(x, y) => PathCommand::MoveTo(Vec2f::new(*x, *y)),
            SvgStylePathCommand::m(dx, dy) => PathCommand::MoveToOffset(Vec2f::new(*dx, *dy)),
            SvgStylePathCommand::L(x, y) => PathCommand::LineTo(Vec2f::new(*x, *y)),
            SvgStylePathCommand::l(dx, dy) => PathCommand::LineToOffset(Vec2f::new(*dx, *dy)),
            SvgStylePathCommand::H(x) => PathCommand::HorizontalLineTo(*x),
            SvgStylePathCommand::h(dx) => PathCommand::HorizontalLineToOffset(*dx),
            SvgStylePathCommand::V(y) => PathCommand::VerticalLineTo(*y),
            SvgStylePathCommand::v(dy) => PathCommand::VerticalLineToOffset(*dy),
            SvgStylePathCommand::Z | SvgStylePathCommand::z => PathCommand::ClosePath,
            SvgStylePathCommand::C(x1, y1, x2, y2, x, y) => PathCommand::CubicBezierCurveTo{
                control_pt_1: Vec2f::new(*x1, *y1),
                control_pt_2: Vec2f::new(*x2, *y2),
                end_pt: Vec2f::new(*x, *y),
            },
            SvgStylePathCommand::c(dx1, dy1, dx2, dy2, dx, dy) => PathCommand::CubicBezierCurveToOffset{
                control_pt_1_offset: Vec2f::new(*dx1, *dy1),
                control_pt_2_offset: Vec2f::new(*dx2, *dy2),
                end_pt_offset: Vec2f::new(*dx, *dy),
            },
            SvgStylePathCommand::S(x2, y2, x, y) => PathCommand::SmoothCubicBezierCurveTo{
                control_pt_2: Vec2f::new(*x2, *y2),
                end_pt: Vec2f::new(*x, *y),
            },
            SvgStylePathCommand::s(dx2, dy2, dx, dy) => PathCommand::SmoothCubicBezierCurveToOffset{
                control_pt_2_offset: Vec2f::new(*dx2, *dy2),
                end_pt_offset: Vec2f::new(*dx, *dy),
            },
            SvgStylePathCommand::Q(x1, y1, x, y) => PathCommand::QuadraticBezierCurveTo{
                control_pt: Vec2f::new(*x1, *y1),
                end_pt: Vec2f::new(*x, *y),
            },
            SvgStylePathCommand::q(dx1, dy1, dx, dy) => PathCommand::QuadraticBezierCurveToOffset{
                control_pt_offset: Vec2f::new(*dx1, *dy1),
                end_pt_offset: Vec2f::new(*dx, *dy),
            },
            SvgStylePathCommand::T(x, y) => PathCommand::SmoothQuadraticBezierCurveTo(Vec2f::new(*x, *y)),
            SvgStylePathCommand::t(dx, dy) => PathCommand::SmoothQuadraticBezierCurveToOffset(Vec2f::new(*dx, *dy)),
            SvgStylePathCommand::A(rx, ry, x_axis_rotation, large_arc_flag, sweep_flag, x, y) => PathCommand::EllipticalArcTo{
                radii: Vec2f::new(*rx, *ry),
                x_axis_rotation: *x_axis_rotation * std::f32::consts::PI / 180.0,
                large_arc_flag: *large_arc_flag,
                sweep_flag: *sweep_flag,
                end_pt: Vec2f::new(*x, *y),
            },
            SvgStylePathCommand::a(rx, ry, x_axis_rotation, large_arc_flag, sweep_flag, dx, dy) => PathCommand::EllipticalArcToOffset{
                radii: Vec2f::new(*rx, *ry),
                x_axis_rotation: *x_axis_rotation * std::f32::consts::PI / 180.0,
                large_arc_flag: *large_arc_flag,
                sweep_flag: *sweep_flag,
                end_pt_offset: Vec2f::new(*dx, *dy),
            },
        }
    }
}

// TODO transform, tolerance, ...
pub fn discretize<'a, 'b>(path: &[PathCommand], params: &'b DiscretizationParams<'a>) -> Vec<BrokenPolylineCommand> {

    // The resulting polyline series.
    let mut result = Vec::new();

    let mut current_position = Vec2f::new(0.0, 0.0);

    // this is used to close the polyline when a `ClosePath` command is encountered
    let mut current_polyline_start = Vec2f::new(0.0, 0.0);
    let mut current_control_point = Vec2f::new(0.0, 0.0);

    for command in path {
        match command {
            PathCommand::MoveTo(position) => {
                result.push(BrokenPolylineCommand::MoveTo(*position));

                // update the current position
                current_position = *position;

                // update the current polyline start since a new polyline is started
                current_polyline_start = current_position;

                // reset the current control point to avoid invalid control points
                current_control_point = current_position;
            },
            PathCommand::MoveToOffset(offset) => {
                let new_position = current_position + *offset;

                result.push(BrokenPolylineCommand::MoveTo(new_position));

                // update the current position
                current_position = new_position;

                // update the current polyline start since a new polyline is started
                current_polyline_start = current_position;

                // reset the current control point to avoid invalid control points
                current_control_point = current_position;
            },
            PathCommand::LineTo(position) => {
                let mut put_point = |point: Vec2f| {
                    result.push(BrokenPolylineCommand::LineTo(point));
                };
                discretize_segment(current_position, *position, &mut put_point, true, params);

                // update the current position
                current_position = *position;

                // reset the current control point to avoid invalid control points
                current_control_point = current_position;
            },
            PathCommand::LineToOffset(offset) => {
                let new_position = current_position + *offset;

                let mut put_point = |point: Vec2f| {
                    result.push(BrokenPolylineCommand::LineTo(point));
                };
                discretize_segment(current_position, new_position, &mut put_point, true, params);

                // update the current position
                current_position = new_position;

                // reset the current control point to avoid invalid control points
                current_control_point = current_position;
            },
            PathCommand::HorizontalLineTo(x) => {
                let new_position = Vec2f::new(*x, current_position.y);

                let mut put_point = |point: Vec2f| {
                    result.push(BrokenPolylineCommand::LineTo(point));
                };
                discretize_segment(current_position, new_position, &mut put_point, true, params);

                // update the current position
                current_position = new_position;

                // reset the current control point to avoid invalid control points
                current_control_point = current_position;
            },
            PathCommand::HorizontalLineToOffset(dx) => {
                let new_position = current_position + Vec2f::new(*dx, 0.0);

                let mut put_point = |point: Vec2f| {
                    result.push(BrokenPolylineCommand::LineTo(point));
                };
                discretize_segment(current_position, new_position, &mut put_point, true, params);

                // update the current position
                current_position = new_position;

                // reset the current control point to avoid invalid control points
                current_control_point = current_position;
            },
            PathCommand::VerticalLineTo(y) => {
                let new_position = Vec2f::new(current_position.x, *y);

                let mut put_point = |point: Vec2f| {
                    result.push(BrokenPolylineCommand::LineTo(point));
                };
                discretize_segment(current_position, new_position, &mut put_point, true, params);

                // update the current position
                current_position = new_position;

                // reset the current control point to avoid invalid control points
                current_control_point = current_position;
            },
            PathCommand::VerticalLineToOffset(dy) => {
                let new_position = current_position + Vec2f::new(0.0, *dy);

                let mut put_point = |point: Vec2f| {
                    result.push(BrokenPolylineCommand::LineTo(point));
                };
                discretize_segment(current_position, new_position, &mut put_point, true, params);

                // update the current position
                current_position = new_position;

                // reset the current control point to avoid invalid control points
                current_control_point = current_position;
            },
            PathCommand::ClosePath => {
                if current_position != current_polyline_start {
                    result.push(BrokenPolylineCommand::LineTo(current_polyline_start));
                }
                current_position = current_polyline_start;
                current_control_point = current_position;
            }
            PathCommand::CubicBezierCurveTo {
                control_pt_1,
                control_pt_2,
                end_pt
            } => {
                // TODO this is a temporary solution
                let mut put_point = |point: Vec2f| {
                    result.push(BrokenPolylineCommand::LineTo(point));
                };

                discretize_cubic_bezier(current_position, *control_pt_1, *control_pt_2, *end_pt, &mut put_point, true, params);

                // update the current position
                current_position = *end_pt;

                // update the current control point
                current_control_point = *control_pt_2;
            }
            PathCommand::CubicBezierCurveToOffset {
                control_pt_1_offset,
                control_pt_2_offset,
                end_pt_offset
            } => {
                let control_point_1 = current_position + *control_pt_1_offset;
                let control_point_2 = current_position + *control_pt_2_offset;
                let end_point = current_position + *end_pt_offset;

                // TODO this is a temporary solution
                let mut put_point = |point: Vec2f| {
                    result.push(BrokenPolylineCommand::LineTo(point));
                };

                discretize_cubic_bezier(current_position, control_point_1, control_point_2, end_point, &mut put_point, true, params);

                // update the current position
                current_position = end_point;

                // update the current control point
                current_control_point = control_point_2;
            }
            PathCommand::SmoothCubicBezierCurveTo {
                control_pt_2,
                end_pt
            } => {
                // TODO this is a temporary solution
                let mut put_point = |point: Vec2f| {
                    result.push(BrokenPolylineCommand::LineTo(point));
                };

                let control_point_1 = current_position + (current_position - current_control_point);

                discretize_cubic_bezier(current_position, control_point_1, *control_pt_2, *end_pt, &mut put_point, true, params);

                // update the current position
                current_position = *end_pt;

                // update the current control point
                current_control_point = *control_pt_2;
            }
            PathCommand::SmoothCubicBezierCurveToOffset {
                control_pt_2_offset,
                end_pt_offset
            } => {
                let control_point_2 = current_position + *control_pt_2_offset;
                let end_point = current_position + *end_pt_offset;

                // TODO this is a temporary solution
                let mut put_point = |point: Vec2f| {
                    result.push(BrokenPolylineCommand::LineTo(point));
                };

                let control_point_1 = current_position + (current_position - current_control_point);

                discretize_cubic_bezier(current_position, control_point_1, control_point_2, end_point, &mut put_point, true, params);

                // update the current position
                current_position = end_point;

                // update the current control point
                current_control_point = control_point_2;
            }
            PathCommand::QuadraticBezierCurveTo {
                control_pt,
                end_pt
            } => {
                // TODO this is a temporary solution
                let mut put_point = |point: Vec2f| {
                    result.push(BrokenPolylineCommand::LineTo(point));
                };

                discretize_quadratic_bezier(current_position, *control_pt, *end_pt, &mut put_point, true, params);

                // update the current position
                current_position = *end_pt;

                // update the current control point
                current_control_point = *control_pt;
            }
            PathCommand::QuadraticBezierCurveToOffset {
                control_pt_offset,
                end_pt_offset
            } => {
                let control_point = current_position + *control_pt_offset;
                let end_point = current_position + *end_pt_offset;

                // TODO this is a temporary solution
                let mut put_point = |point: Vec2f| {
                    result.push(BrokenPolylineCommand::LineTo(point));
                };

                discretize_quadratic_bezier(current_position, control_point, end_point, &mut put_point, true, params);

                // update the current position
                current_position = end_point;

                // update the current control point
                current_control_point = control_point;
            }
            PathCommand::SmoothQuadraticBezierCurveTo(end_point) => {
                // TODO this is a temporary solution
                let mut put_point = |point: Vec2f| {
                    result.push(BrokenPolylineCommand::LineTo(point));
                };

                let control_point = current_position + (current_position - current_control_point);

                discretize_quadratic_bezier(current_position, control_point, *end_point, &mut put_point, true, params);

                // update the current position
                current_position = *end_point;

                // update the current control point
                current_control_point = control_point;
            }
            PathCommand::SmoothQuadraticBezierCurveToOffset(end_point_offset) => {
                let end_point = current_position + *end_point_offset;

                // TODO this is a temporary solution
                let mut put_point = |point: Vec2f| {
                    result.push(BrokenPolylineCommand::LineTo(point));
                };

                let control_point = current_position + (current_position - current_control_point);

                discretize_quadratic_bezier(current_position, control_point, end_point, &mut put_point, true, params);

                // update the current position
                current_position = end_point;

                // update the current control point
                current_control_point = control_point;
            }
            PathCommand::EllipticalArcTo {
                radii,
                x_axis_rotation,
                large_arc_flag,
                sweep_flag,
                end_pt
            } => {
                // TODO this is a temporary solution
                let mut put_point = |point: Vec2f| {
                    result.push(BrokenPolylineCommand::LineTo(point));
                };

                discretize_elliptical_arc(radii.x, radii.y, *x_axis_rotation, *large_arc_flag, *sweep_flag, current_position, *end_pt, &mut put_point, true, params);

                // update the current position
                current_position = *end_pt;

                // TODO update the current control point
            }
            PathCommand::EllipticalArcToOffset {
                radii,
                x_axis_rotation, large_arc_flag,
                sweep_flag,
                end_pt_offset
            } => {
                let end_point = current_position + *end_pt_offset;

                // TODO this is a temporary solution
                let mut put_point = |point: Vec2f| {
                    result.push(BrokenPolylineCommand::LineTo(point));
                };

                discretize_elliptical_arc(radii.x, radii.y, *x_axis_rotation, *large_arc_flag, *sweep_flag, current_position, end_point, &mut put_point, true, params);

                // update the current position
                current_position = end_point;

                // TODO update the current control point
            }
            //_ => (),
        }
    }

    result
}


/// This struct contains the parameters for the discretization of a curve.
pub struct DiscretizationParams<'a> {
    // maximum number of subdivisions for each piece of the curve
    //max_subdivisions: i32,

    /// maximum distance between the curve and the polyline
    pub tolerance: f32,

    /// maximum angle between the curve and the polyline
    pub max_angle: f32,

    /// area of interest, if the curve is outside of this area, it will not be subdivided
    pub aoi: Option<FRect>,

    /// transform to apply to the curve
    pub transform: DiscretizationTransform<'a>,
}

pub enum DiscretizationTransform<'a> {
    /// The identity transform
    Identity,

    /// the Transform is a pure homogeneous scale
    Scale(f32),

    /// The transform is non-homogeneous scale
    NonHomogeneousScale(Mat2f),

    /// The transform is an affine transform
    Affine{
        linear: Mat2f,
        translation: Vec2f,
    },

    // The transform is a general transform that preserves lines
    GeneralLinesPreserving(&'a dyn Fn(Vec2f) -> Vec2f),

    // The transform is a general transform
    General(&'a dyn Fn(Vec2f) -> Vec2f),
}

pub fn eval_discretization_transform<'a>(transform: &DiscretizationTransform<'a>, point: Vec2f) -> Vec2f {
    match transform {
        DiscretizationTransform::Identity => point,
        DiscretizationTransform::Scale(scale) => point * *scale,
        DiscretizationTransform::NonHomogeneousScale(linear) => linear * point,
        DiscretizationTransform::Affine{linear, translation} => linear * point + *translation,
        DiscretizationTransform::GeneralLinesPreserving(transform) => transform(point),
        DiscretizationTransform::General(transform) => transform(point),
    }
}

trait ParametricCurve {
    fn eval(&self, t: f32) -> Vec2f;
    fn start(&self) -> f32;
    fn end(&self) -> f32;
}

/// This function discretizes a parametric curve.
fn discretize_parametric_curve<'a, 'b>(
    curve: &dyn ParametricCurve,
    callback: &mut dyn FnMut(Vec2f) -> (),
    params: &'b DiscretizationParams<'a>,
    skip_first: bool,
    initial_subs: u32,
) {
    const MIN_DT: f32 = 1e-3; // TODO use
    const DT_TOL: f32 = 1e-3; // TODO use
    // TODO rename in *_DT in *_STEP ?

    // we do not want to discretize curves that have non-finite parameter endpoints
    if !curve.start().is_finite() || !curve.end().is_finite() {
        // this is a meaningless curve we cannot discretize
        return;
    }

    // we do not want to discretize curves that have very small parameter range
    if (curve.end() - curve.start()).abs() < DT_TOL {
        // this is a degenerate curve
        return;
    }

    // evaluate the curve at the start point if needed
    if !skip_first {
        callback(curve.eval(curve.start()));
    }

    // the number of subdivisions for the first step,
    // we add because we want `initial_subs` to represent the number of
    // intermediate points, not the number of subdivisions ?!?!?!?!
    let initial_divisions = initial_subs + 1;

    // the current step size
    let mut curr_t_step = (curve.end() - curve.start()) / (initial_divisions as f32);
    // the current parameter value
    let mut curr_t: f32 = 0.0;
    // the current point
    let mut curr_point = curve.eval(curve.start());
    // the previous point
    //let mut prev_pt: Option<Vec2f> = None;

    // checks if the value is close or past the end of the curve
    let is_end_or_past_end = |t: f32| -> bool {
        // if the value is close to the end, we consider it to be the end
        if (t - curve.end()).abs() < DT_TOL {
            return true;
        }

        // past the end checks
        if curve.end() > curve.start() {
            // if the curve is increasing, we check if the value is greater than the end
            t >= curve.end()
        } else {
            // if the curve is decreasing, we check if the value is less than the end
            t <= curve.end()
        }
    };

    // picks the next t value and returns the next t value, whether the curve is finished, and the step size
    let next_t_for_step = |t: f32, step: f32| -> (f32, bool, f32) {
        if is_end_or_past_end(t + step) {
            (curve.end(), true, curve.end() - t)
        } else {
            (t + step, false, step)
        }
    };

    loop {
        // check if the given next point satisfies the discretization accuracy conditions
        let is_ok = |next_point: &Vec2f, mid_pt: &Vec2f| -> bool {
            // if a point is outside of the area of interest, we don't need to check the other conditions
            // since we would not care about precision, we can just return true
            // ? this might not be correct as intermediate points might be inside the aoi
            // ? but the current next_point might not be
            if let Some(aoi) = &params.aoi {
                if !aoi.contains(next_point) {
                    return true;
                }
            }

            // check that the midpoint is close enough to the line between the current point and the next point
            // (this is an approximation of the distance between the curve and the line, i.e. the "error")
            let dist = dist_to_segment(&mid_pt, (&curr_point, &next_point));
            if dist > params.tolerance {
                return false;
            }

            // compute the angle between the next_point and the line between the current point and the previous point
            //let angle = if let Some(prev_pt) = prev_pt {
            //    // TODO written by copilot, to check
            //    let v = curr_point - prev_pt;
            //    let w = next_point - prev_pt;
            //    let v = v.normalize();
            //    let w = w.normalize();
            //    let cos_angle = v.dot(&w);
            //    let angle = cos_angle.acos();
            //    angle
            //} else {
            //    0.0
            //};
            let angle = angle_between(&(mid_pt - curr_point), &(next_point - mid_pt));
            // TODO instead of "angle_between" that requires atan2, we could just use the cross product
            // or the dot product of the normalized vectors and cache sin or cos of the max angle
            if angle.abs() > params.max_angle {
                return false;
            }

            // if we reached this point, the point respects the accuracy conditions
            true
        };

        // pick the next point
        let (next_t, next_is_end, next_pt) = {
            let (next_t, next_is_end, stepped) = next_t_for_step(curr_t, curr_t_step);
            let next_pt = curve.eval(next_t);
            let mid_pt = curve.eval(curr_t + stepped / 2.0);
            let next_pt_is_ok = is_ok(&next_pt, &mid_pt);
            // Now, if the next point is the end and it is ok, we can use it.
            // But if the next point is not the end and it is ok, we can try to double the step
            // until we reach the end or the point is not ok to avoid too many points.
            // If the next point is either the end or not, but it is not ok, we can halve the step
            // until we reach the end or the point is ok.
            // note that every time we halve the step, we have to check that the step is greater than MIN_DT
            if next_is_end && next_pt_is_ok {
                println!("1");
                (next_t, next_is_end, next_pt)
            } else if !next_is_end && next_pt_is_ok {
                println!("2");
                let mut step = stepped;
                let mut t = next_t;
                let mut pt = next_pt;
                let mut next_is_end = next_is_end;
                loop {
                    let (next_t, next_is_end_2, stepped) = next_t_for_step(curr_t, step * 2.0);
                    let next_pt = curve.eval(next_t);
                    let mid_pt = curve.eval(curr_t + stepped / 2.0);
                    let next_pt_is_ok = is_ok(&next_pt, &mid_pt);
                    if !next_pt_is_ok {
                        break;
                    }
                    step = stepped;
                    t = next_t;
                    pt = next_pt;
                    next_is_end = next_is_end_2;
                    if next_is_end {
                        break;
                    }
                }
                (t, next_is_end, pt)
            } else { // && !next_pt_is_ok
                println!("3");
                let mut step = stepped;
                let mut t = next_t;
                let mut pt = next_pt;
                let mut next_is_end = next_is_end;
                loop {
                    let (next_t, next_is_end_2, stepped) = next_t_for_step(curr_t, step / 2.0);
                    let next_pt = curve.eval(next_t);
                    let mid_pt = curve.eval(curr_t + stepped / 2.0);
                    let next_pt_is_ok = is_ok(&next_pt, &mid_pt);
                    if stepped.abs() < MIN_DT {
                        break;
                    }
                    step = stepped;
                    t = next_t;
                    pt = next_pt;
                    next_is_end = next_is_end_2;
                    if next_is_end || next_pt_is_ok {
                        break;
                    }
                }
                println!("(t, next_is_end, pt): {:?}", (t, next_is_end, pt, step));
                (t, next_is_end, pt)
            }
        };

        println!("next_t: {}, next_is_end: {}, next_pt: {:?}", next_t, next_is_end, next_pt);
        
        callback(next_pt);

        if next_is_end {
            break;
        }

        curr_t_step = next_t - curr_t;
        //prev_pt = Some(curr_point);
        curr_point = next_pt;
        curr_t = next_t;
        println!("curr_t_step: {}", curr_t_step);
    }
}

struct ParametricLineFunction {
    p0: Vec2f,
    p1: Vec2f,
}

impl ParametricLineFunction {
    fn new(p0: Vec2f, p1: Vec2f) -> Self {
        Self { p0, p1 }
    }
}

impl ParametricCurve for ParametricLineFunction {
    fn eval(&self, t: f32) -> Vec2f {
        self.p0 + (self.p1 - self.p0) * t
    }
    fn start(&self) -> f32 {
        0.0
    }
    fn end(&self) -> f32 {
        1.0
    }
}

fn discretize_segment<'a, 'b, F: FnMut(Vec2f) -> ()>(p0: Vec2f, p1: Vec2f, put_point: &mut F, skip_first: bool, params: &'b DiscretizationParams<'a>) {
    match params.transform {
        DiscretizationTransform::Identity => {
            if skip_first {
                put_point(p1);
            } else {
                put_point(p0);
                put_point(p1);
            }
        },
        DiscretizationTransform::Scale(_) | DiscretizationTransform::NonHomogeneousScale(_) | DiscretizationTransform::GeneralLinesPreserving(_) => {
            let p0_primed = eval_discretization_transform(&params.transform, p0);
            let p1_primed = eval_discretization_transform(&params.transform, p1);
            if skip_first {
                put_point(p1_primed);
            } else {
                put_point(p0_primed);
                put_point(p1_primed);
            }
        },
        DiscretizationTransform::Affine{linear: _, translation: _} => {
            // TODO avoid code duplication
            let p0_primed = eval_discretization_transform(&params.transform, p0);
            let p1_primed = eval_discretization_transform(&params.transform, p1);
            if skip_first {
                put_point(p1_primed);
            } else {
                put_point(p0_primed);
                put_point(p1_primed);
            }
        }
        DiscretizationTransform::General(_) => {
            let p0_primed = eval_discretization_transform(&params.transform, p0);
            let p1_primed = eval_discretization_transform(&params.transform, p1);
            let line = ParametricLineFunction::new(p0_primed, p1_primed);
            discretize_parametric_curve(&line, put_point, params, skip_first, 1);
        },
    }
}

#[derive(Clone, Copy, Debug)]
struct Cubic2dBezierFunction {
    p0: Vec2f,
    p1: Vec2f,
    p2: Vec2f,
    p3: Vec2f,
}

impl Cubic2dBezierFunction {
    fn new(p0: Vec2f, p1: Vec2f, p2: Vec2f, p3: Vec2f) -> Self {
        Self { p0, p1, p2, p3 }
    }
}

impl ParametricCurve for Cubic2dBezierFunction {
    fn eval(&self, t: f32) -> Vec2f {
        let t2 = t * t;
        let t3 = t2 * t;
        let mt = 1.0 - t;
        let mt2 = mt * mt;
        let mt3 = mt2 * mt;
        self.p0 * mt3 + self.p1 * 3.0 * mt2 * t + self.p2 * 3.0 * mt * t2 + self.p3 * t3
    }

    fn start(&self) -> f32 {
        0.0
    }

    fn end(&self) -> f32 {
        1.0
    }
}

fn cubic_bezier_to_curve(p0: Vec2f, p1: Vec2f, p2: Vec2f, p3: Vec2f) -> Cubic2dBezierFunction {
    Cubic2dBezierFunction::new(p0, p1, p2, p3)
}

fn discretize_cubic_bezier<'a, 'b, F: FnMut(Vec2f) -> ()>(p0: Vec2f, p1: Vec2f, p2: Vec2f, p3: Vec2f, put_point: &mut F, skip_first: bool, params: &'b DiscretizationParams<'a>) {
    const DEGREE: u32 = 3;
    let curve = cubic_bezier_to_curve(p0, p1, p2, p3);
    println!("discretizing cubic bezier: {:?}", curve);
    discretize_parametric_curve(&curve, put_point, params, skip_first, DEGREE);
}

struct Quadratic2dBezierFunction {
    p0: Vec2f,
    p1: Vec2f,
    p2: Vec2f,
}

impl Quadratic2dBezierFunction {
    fn new(p0: Vec2f, p1: Vec2f, p2: Vec2f) -> Self {
        Self { p0, p1, p2 }
    }
}

impl ParametricCurve for Quadratic2dBezierFunction {
    fn eval(&self, t: f32) -> Vec2f {
        let t2 = t * t;
        let mt = 1.0 - t;
        let mt2 = mt * mt;
        self.p0 * mt2 + self.p1 * 2.0 * mt * t + self.p2 * t2
    }

    fn start(&self) -> f32 {
        0.0
    }

    fn end(&self) -> f32 {
        1.0
    }
}

fn quadratic_bezier_to_curve(p0: Vec2f, p1: Vec2f, p2: Vec2f) -> Quadratic2dBezierFunction {
    Quadratic2dBezierFunction::new(p0, p1, p2)
}

fn discretize_quadratic_bezier(p0: Vec2f, p1: Vec2f, p2: Vec2f, put_point: &mut impl FnMut(Vec2f) -> (), skip_first: bool, params: &DiscretizationParams) {
    const DEGREE: u32 = 2;
    let curve = quadratic_bezier_to_curve(p0, p1, p2);
    discretize_parametric_curve(&curve, put_point, params, skip_first, DEGREE);
}

#[derive(Clone, Copy, Debug)]
struct CenterParametricArcFunction {
    center: Vec2f,
    radii: Vec2f,
    start_angle: f32,
    //end_angle: f32,
    sweep: f32,
    x_axis_rotation: f32
}

impl CenterParametricArcFunction {
    fn new(center: Vec2f, radii: Vec2f, start_angle: f32, sweep: f32, x_axis_rotation: f32) -> Self {
        Self { center, radii, start_angle, sweep, x_axis_rotation }
    }
}

impl ParametricCurve for CenterParametricArcFunction {
    fn eval(&self, t: f32) -> Vec2f {
        let angle = self.start_angle + t * self.sweep;
        let x = self.center.x + self.radii.x * angle.cos();
        let y = self.center.y + self.radii.y * angle.sin();
        let pt = Vec2f::new(x, y);
        //pt.rotate_around(self.center, self.x_axis_rotation)
        let rot_mat = {
            let cos = self.x_axis_rotation.cos();
            let sin = self.x_axis_rotation.sin();
            Mat2f::new(cos, -sin, sin, cos)
        };
        rot_mat * (pt - self.center) + self.center
    }

    fn start(&self) -> f32 {
        0.0
    }

    fn end(&self) -> f32 {
        1.0
    }
}

fn endpoint_to_center_parametric_arc_function(rx: f32, ry: f32, x_axis_rotation: f32, large_arc_flag: bool, sweep_flag: bool, p0: Vec2f, p1: Vec2f) -> CenterParametricArcFunction {
    // https://www.w3.org/TR/SVG/implnote.html
    // https://math.stackexchange.com/questions/4285747/last-step-of-conversion-from-endpoint-to-center-parameterization-of-an-elliptica
    // https://crates.io/crates/contrast_renderer

    let radii = Vec2f::new(rx, ry);

    let (phi_rot_mat, phi_inv_rot_mat) = {
        let cos = x_axis_rotation.cos();
        let sin = x_axis_rotation.sin();
        (Mat2f::new(cos, -sin, sin, cos), Mat2f::new(cos, sin, -sin, cos))
    };

    let p0_primed = phi_inv_rot_mat * (p0 - p1) * 0.5;
    println!("p0_primed: {:?}", p0_primed);
    let c_primed = (
        (sqr(rx) * sqr(ry) - sqr(rx) * sqr(p0_primed.y) - sqr(ry) * sqr(p0_primed.x)) /
        (sqr(rx) * sqr(p0_primed.y) + sqr(ry) * sqr(p0_primed.x))
    ).sqrt() *
    (if large_arc_flag != sweep_flag { 1.0 } else { -1.0 }) *
    //Vec2f::new(
    //    p0_primed.y,
    //    -p0_primed.x
    //).normalize();
    Vec2f::new(
        p0_primed.y * rx / ry,
        -p0_primed.x * ry / rx
    );
    println!("c_primed: {:?}", c_primed);

    let c = phi_rot_mat * c_primed + (p0 + p1) * 0.5;

    //let angle_between = |v0: Vec2f, v1: Vec2f| {
    //    // TODO by copilot, very clever! But to check...
    //    let dot = v0.dot(&v1);
    //    let det = v0.x * v1.y - v0.y * v1.x;
    //    det.atan2(dot)
    //};

    let theta_0 = angle_between(&Vec2f::new(1.0, 0.0), &(p0_primed - c_primed).component_div(&radii));
    // TODO this mod may be wrong...
    let delta_theta = angle_between(&(p0_primed - c_primed).component_div(&radii), &(-p0_primed - c_primed).component_div(&radii)) % (2.0 * std::f32::consts::PI);
    let delta_theta = if sweep_flag {
        if delta_theta < 0.0 {
            delta_theta + 2.0 * std::f32::consts::PI
        } else {
            delta_theta
        }
    } else {
        if delta_theta > 0.0 {
            delta_theta - 2.0 * std::f32::consts::PI
        } else {
            delta_theta
        }
    }; // ?????????????????????????

    // TODO correct out of range radii....

    CenterParametricArcFunction::new(c, radii, theta_0, delta_theta, x_axis_rotation)
}

fn discretize_elliptical_arc<F: FnMut(Vec2f) -> ()>(rx: f32, ry: f32, x_axis_rotation: f32, large_arc_flag: bool, sweep_flag: bool, p0: Vec2f, p1: Vec2f, put_point: &mut F, skip_first: bool, params: &DiscretizationParams)
{
    let arc = endpoint_to_center_parametric_arc_function(rx, ry, x_axis_rotation, large_arc_flag, sweep_flag, p0, p1);
    println!("arc: {:?}", arc);
    discretize_parametric_curve(&arc, put_point, params, skip_first, 2);
}

#[inline]
fn lerp<T, V>(a: V, b: V, t: T) -> V
where
    T: Copy,
    V: Mul<T, Output = V> + Add<V, Output = V>,
    f32: Sub<T, Output = T>,
{
    a * (1.0 - t) + b * t
}

#[inline]
fn sqr<T>(x: T) -> <T as Mul>::Output
where
    T: Mul<T> + Copy,
{
    x * x
}


fn dist_to_segment(pt: &Vec2f, segment: (&Vec2f, &Vec2f)) -> f32 {
    let (a, b) = segment;
    let ab = b - a;
    let ap = pt - a;
    let t = ap.dot(&ab) / ab.dot(&ab);
    let closest = lerp(*a, *b, t);
    (closest - pt).norm()
}

fn angle_between(v0: &Vec2f, v1: &Vec2f) -> f32 {
    // TODO by copilot, very clever! But to check...
    let dot = v0.dot(&v1);
    let det = v0.x * v1.y - v0.y * v1.x;
    det.atan2(dot)
}