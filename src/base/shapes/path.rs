type Vec2f = nalgebra::Vector2<f32>;
type Mat2f = nalgebra::Matrix2<f32>;

/// A command for a path.
///
/// A path is a sequence of commands that are executed in order to form a shape.
/// For example, a rectangle can be created by moving to the top left corner,
/// then drawing a line to the top right corner, then drawing a line to the bottom
/// right corner, then drawing a line to the bottom left corner, and finally closing.
/// It would be represented, for example, by the following commands:
/// ```rust
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
    ///
    /// TODO ...
    MoveTo(Vec2f),

    /// Move to a point relative to the current position.
    ///
    /// This command is the same as [`PathCommand::MoveTo`], but the point is relative to the current position.
    ///
    /// TODO ...
    MoveToOffset(Vec2f),

    /// Draw a line to a point.
    ///
    /// This command draws a straight line from the current position to the given point, the current position
    /// is then set to the given point.
    ///
    /// TODO ...
    LineTo(Vec2f),

    /// Draw a line to a point relative to the current position.
    ///
    /// This command is the same as [`PathCommand::LineTo`], but the point is relative to the current position.
    ///
    /// TODO ...
    LineToOffset(Vec2f),

    /// Draw a horizontal line to a point.
    ///
    /// This command draws a straight line from the current position to the given point, the current position
    /// is then set to the given point.
    ///
    /// TODO ...
    HorizontalLineTo(f32),

    /// Draw a horizontal line to a point relative to the current position.
    ///
    /// This command is the same as [`PathCommand::HorizontalLineTo`], but the point is relative to the current position.
    ///
    /// TODO ...
    HorizontalLineToOffset(f32),

    /// Draw a vertical line to a point.
    ///
    /// This command draws a straight line from the current position to the given point, the current position
    /// is then set to the given point.
    ///
    /// TODO ...
    VerticalLineTo(f32),

    /// Draw a vertical line to a point relative to the current position.
    ///
    /// This command is the same as [`PathCommand::VerticalLineTo`], but the point is relative to the current position.
    ///
    /// TODO ...
    VerticalLineToOffset(f32),

    /// Close the current path with a straight line.
    ///
    /// This command draws a straight line from the current position to the starting position of the path.
    /// The current position is then set to the starting position of the path.
    ///
    /// TODO ...
    ClosePath,

    /// Draw a cubic Bezier curve to a point.
    ///
    /// TODO ...
    CubicBezierCurveTo {
        /// The first control point.
        control_pt_1: Vec2f,
        /// The second control point.
        control_pt_2: Vec2f,
        /// The end point.
        end_pt: Vec2f,
    },

    /// Draw a cubic Bezier curve to a point relative to the current position.
    ///
    /// TODO ...
    CubicBezierCurveToOffset {
        /// The first control point.
        control_pt_1_offset: Vec2f,
        /// The second control point.
        control_pt_2_offset: Vec2f,
        /// The end point.
        end_pt_offset: Vec2f,
    },

    /// Draw a cubic Bezier curve to a point.
    ///
    /// TODO ...
    SmoothCubicBezierCurveTo {
        /// The control point.
        control_pt_2: Vec2f,
        /// The end point.
        end_pt: Vec2f,
    },

    /// Draw a cubic Bezier curve to a point relative to the current position.
    ///
    /// TODO ...
    SmoothCubicBezierCurveToOffset {
        /// The control point.
        control_pt_2_offset: Vec2f,
        /// The end point.
        end_pt_offset: Vec2f,
    },

    /// Draw a quadratic Bezier curve to a point.
    ///
    /// TODO ...
    QuadraticBezierCurveTo {
        /// The control point.
        control_pt: Vec2f,
        /// The end point.
        end_pt: Vec2f,
    },

    /// Draw a quadratic Bezier curve to a point relative to the current position.
    ///
    /// TODO ...
    QuadraticBezierCurveToOffset {
        /// The control point.
        control_pt_offset: Vec2f,
        /// The end point.
        end_pt_offset: Vec2f,
    },

    /// Draw a quadratic Bezier curve to a point.
    ///
    /// The last point is the end point.
    ///
    /// TODO ...
    SmoothQuadraticBezierCurveTo(Vec2f),

    /// Draw a quadratic Bezier curve to a point relative to the current position.
    ///
    /// The last point is the end point.
    ///
    /// TODO ...
    SmoothQuadraticBezierCurveToOffset(Vec2f),

    /// Draw an elliptical arc to a point.
    ///
    /// See https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial/Paths#arcs
    ///
    /// TODO ...
    //EllipticalArcTo(f32, f32, f32, bool, bool, Vec2f),
    EllipticalArcTo {
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
    /// TODO ...
    EllipticalArcToOffset {
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
            SvgStylePathCommand::C(x1, y1, x2, y2, x, y) => PathCommand::CubicBezierCurveTo {
                control_pt_1: Vec2f::new(*x1, *y1),
                control_pt_2: Vec2f::new(*x2, *y2),
                end_pt: Vec2f::new(*x, *y),
            },
            SvgStylePathCommand::c(dx1, dy1, dx2, dy2, dx, dy) => {
                PathCommand::CubicBezierCurveToOffset {
                    control_pt_1_offset: Vec2f::new(*dx1, *dy1),
                    control_pt_2_offset: Vec2f::new(*dx2, *dy2),
                    end_pt_offset: Vec2f::new(*dx, *dy),
                }
            }
            SvgStylePathCommand::S(x2, y2, x, y) => PathCommand::SmoothCubicBezierCurveTo {
                control_pt_2: Vec2f::new(*x2, *y2),
                end_pt: Vec2f::new(*x, *y),
            },
            SvgStylePathCommand::s(dx2, dy2, dx, dy) => {
                PathCommand::SmoothCubicBezierCurveToOffset {
                    control_pt_2_offset: Vec2f::new(*dx2, *dy2),
                    end_pt_offset: Vec2f::new(*dx, *dy),
                }
            }
            SvgStylePathCommand::Q(x1, y1, x, y) => PathCommand::QuadraticBezierCurveTo {
                control_pt: Vec2f::new(*x1, *y1),
                end_pt: Vec2f::new(*x, *y),
            },
            SvgStylePathCommand::q(dx1, dy1, dx, dy) => PathCommand::QuadraticBezierCurveToOffset {
                control_pt_offset: Vec2f::new(*dx1, *dy1),
                end_pt_offset: Vec2f::new(*dx, *dy),
            },
            SvgStylePathCommand::T(x, y) => {
                PathCommand::SmoothQuadraticBezierCurveTo(Vec2f::new(*x, *y))
            }
            SvgStylePathCommand::t(dx, dy) => {
                PathCommand::SmoothQuadraticBezierCurveToOffset(Vec2f::new(*dx, *dy))
            }
            SvgStylePathCommand::A(rx, ry, x_axis_rotation, large_arc_flag, sweep_flag, x, y) => {
                PathCommand::EllipticalArcTo {
                    radii: Vec2f::new(*rx, *ry),
                    x_axis_rotation: *x_axis_rotation * std::f32::consts::PI / 180.0,
                    large_arc_flag: *large_arc_flag,
                    sweep_flag: *sweep_flag,
                    end_pt: Vec2f::new(*x, *y),
                }
            }
            SvgStylePathCommand::a(rx, ry, x_axis_rotation, large_arc_flag, sweep_flag, dx, dy) => {
                PathCommand::EllipticalArcToOffset {
                    radii: Vec2f::new(*rx, *ry),
                    x_axis_rotation: *x_axis_rotation * std::f32::consts::PI / 180.0,
                    large_arc_flag: *large_arc_flag,
                    sweep_flag: *sweep_flag,
                    end_pt_offset: Vec2f::new(*dx, *dy),
                }
            }
        }
    }
}

/// Some basic discretization utilities.
///
/// This module contains some very simple algorithms for discretizing paths and polylines.
pub mod discretization {
    use super::{Mat2f, Vec2f};
    use crate::base::defs::rect::FRect;
    use crate::base::shapes::{path::PathCommand, polyline::BrokenPolylineCommand};

    use std::ops::{
        Add,
        //Div
        Mul,
        Sub,
    };

    /// This struct contains the parameters for the discretization of a curve.
    ///
    /// The discretization algorithm will try to approximate the curve with a polyline
    /// that has a maximum distance between the curve and the polyline of `tolerance` and
    /// a maximum angle between the curve and the polyline of `max_angle`.
    #[derive(Clone, Copy, Debug)]
    pub struct DiscretizationParams {
        /// maximum distance between the curve and the polyline
        pub tolerance: f32,

        /// maximum angle between the curve and the polyline
        pub max_angle: f32,

        /// Area Of Interest, if the curve is outside of this area, it will not be subdivided
        /// in order to avoid wasting time on curves that are not visible or near-infinite.
        ///
        /// If you don't want to use this feature, set it to `None`.
        pub aoi: Option<FRect>,
    }

    impl Default for DiscretizationParams {
        fn default() -> Self {
            Self {
                tolerance: 0.1,
                max_angle: 0.1,
                aoi: None,
            }
        }
    }

    /// This enum represents the transform to apply to the curve before discretizing it.
    #[derive(Clone, Copy)]
    pub enum DiscretizationTransform<'a> {
        /// The identity transform
        Identity,

        /// The Transform is a pure homogeneous scale
        Scale(f32),

        /// The transform is non-homogeneous scale
        NonHomogeneousScale(Mat2f),

        /// The transform is an affine transform
        Affine {
            linear: Mat2f,
            translation: Vec2f,
        },

        // The transform is a general transform that preserves lines
        GeneralLinesPreserving(&'a dyn Fn(Vec2f) -> Vec2f),

        // The transform is a general transform
        General(&'a dyn Fn(Vec2f) -> Vec2f),
    }

    impl<'a> DiscretizationTransform<'a> {
        /// Applies the transform to a point
        pub fn eval(&self, point: Vec2f) -> Vec2f {
            match self {
                DiscretizationTransform::Identity => point,
                DiscretizationTransform::Scale(scale) => point * *scale,
                DiscretizationTransform::NonHomogeneousScale(linear) => linear * point,
                DiscretizationTransform::Affine {
                    linear,
                    translation,
                } => linear * point + *translation,
                DiscretizationTransform::GeneralLinesPreserving(transform) => transform(point),
                DiscretizationTransform::General(transform) => transform(point),
            }
        }

        pub fn is_line_preserving(&self) -> bool {
            match self {
                DiscretizationTransform::Identity => true,
                DiscretizationTransform::Scale(_) => true,
                DiscretizationTransform::NonHomogeneousScale(_) => true,
                DiscretizationTransform::Affine { .. } => true,
                DiscretizationTransform::GeneralLinesPreserving(_) => true,
                DiscretizationTransform::General(_) => false,
            }
        }
    }

    /// This is used to perform the discretization of a path.
    ///
    /// ## Example
    /// ```rust
    /// let path_commands = [
    ///    PathCommand::MoveTo(Vec2f::new(0.0, 0.0)),
    ///    PathCommand::LineTo(Vec2f::new(1.0, 0.0)),
    /// ];
    /// let discretizer = PathDiscretizer::new(
    ///     DiscretizationParams::default(),
    ///     DiscretizationTransform::Identity
    /// );
    /// let mut it = path.iter();
    /// for command in discretizer.discretize(&mut it) {
    ///    // do something ...
    /// }
    /// ```
    pub struct PathDiscretizer<'t> {
        /// The discretization parameters.
        params: DiscretizationParams,
        /// transform to apply to the curve
        transform: DiscretizationTransform<'t>,
    }

    impl<'t> PathDiscretizer<'t> {
        /// Creates a new path discretizer for the given parameters and transform.
        pub fn new(params: DiscretizationParams, transform: DiscretizationTransform<'t>) -> Self {
            Self { params, transform }
        }

        /// Discretizes a path, see the example in the [`PathDiscretizer`] struct.
        pub fn discretize<'i, 'a>(
            &'a self,
            path: &'i mut dyn Iterator<Item = &'i PathCommand>,
        ) -> PathDiscretizerIterator<'i, 't, 'a> {
            PathDiscretizerIterator {
                discretizer: self,
                path,
                state: PathDiscretizationState::new(),
            }
        }
    }

    /// Iterates through a path and discretizes it.
    ///
    /// This is not intended to be used directly, use [`PathDiscretizer::discretize`] instead.
    pub struct PathDiscretizerIterator<'i: 'a, 't: 'a, 'a> {
        /// The discretizer that created this iterator.
        discretizer: &'a PathDiscretizer<'t>,
        /// The path to discretize.
        path: &'i mut dyn Iterator<Item = &'i PathCommand>,
        /// The current state of the discretization.
        state: PathDiscretizationState<'a>,
    }

    impl<'i, 't, 'here> Iterator for PathDiscretizerIterator<'i, 't, 'here> {
        type Item = BrokenPolylineCommand;

        fn next(&mut self) -> Option<Self::Item> {
            // TODO instead of recursion, use next_impl and that retuns Some<Option<Self::Item>> or
            // Retry, here we should loop while Retry to avoid stack overflow caused by recursion

            // If we are currently in the middle of a subpath, continue iterating over it.
            if let Some(ref mut sub_iter) = self.state.subpath_iterator {
                if let Some(pt) = sub_iter.next() {
                    return Some(BrokenPolylineCommand::LineTo(pt));
                } else {
                    self.state.subpath_iterator = None;
                }
            }

            // Otherwise, try to pick up the next path command and start a new subpath.
            if let Some(command) = self.path.next() {
                match command {
                    PathCommand::MoveTo(pt) => {
                        self.state.move_to(pt);
                        return Some(BrokenPolylineCommand::MoveTo(self.state.current_position));
                    }
                    PathCommand::MoveToOffset(offset) => {
                        let pt = self.state.current_position + offset;
                        self.state.move_to(&pt);
                        return Some(BrokenPolylineCommand::MoveTo(pt));
                    }
                    PathCommand::LineTo(pt) => {
                        let it = curves::discretize_segment(
                            self.state.current_position,
                            *pt,
                            true,
                            &self.discretizer.params,
                            &self.discretizer.transform,
                        );
                        self.state.set_curr_pos_no_ctrl_pt(&pt);
                        self.state.subpath_iterator = Some(it);
                        return self.next();
                    }
                    PathCommand::LineToOffset(offset) => {
                        let pt = self.state.current_position + offset;
                        let it = curves::discretize_segment(
                            self.state.current_position,
                            pt,
                            true,
                            &self.discretizer.params,
                            &self.discretizer.transform,
                        );
                        self.state.set_curr_pos_no_ctrl_pt(&pt);
                        self.state.subpath_iterator = Some(it);
                        return self.next();
                    }
                    PathCommand::HorizontalLineTo(x) => {
                        let pt = Vec2f::new(*x, self.state.current_position.y);
                        let it = curves::discretize_segment(
                            self.state.current_position,
                            pt,
                            true,
                            &self.discretizer.params,
                            &self.discretizer.transform,
                        );
                        self.state.set_curr_pos_no_ctrl_pt(&pt);
                        self.state.subpath_iterator = Some(it);
                        return self.next();
                    }
                    PathCommand::HorizontalLineToOffset(offset) => {
                        let pt = Vec2f::new(
                            self.state.current_position.x + offset,
                            self.state.current_position.y,
                        );
                        let it = curves::discretize_segment(
                            self.state.current_position,
                            pt,
                            true,
                            &self.discretizer.params,
                            &self.discretizer.transform,
                        );
                        self.state.set_curr_pos_no_ctrl_pt(&pt);
                        self.state.subpath_iterator = Some(it);
                        return self.next();
                    }
                    PathCommand::VerticalLineTo(y) => {
                        let pt = Vec2f::new(self.state.current_position.x, *y);
                        let it = curves::discretize_segment(
                            self.state.current_position,
                            pt,
                            true,
                            &self.discretizer.params,
                            &self.discretizer.transform,
                        );
                        self.state.set_curr_pos_no_ctrl_pt(&pt);
                        self.state.subpath_iterator = Some(it);
                        return self.next();
                    }
                    PathCommand::VerticalLineToOffset(offset) => {
                        let pt = Vec2f::new(
                            self.state.current_position.x,
                            self.state.current_position.y + offset,
                        );
                        let it = curves::discretize_segment(
                            self.state.current_position,
                            pt,
                            true,
                            &self.discretizer.params,
                            &self.discretizer.transform,
                        );
                        self.state.set_curr_pos_no_ctrl_pt(&pt);
                        self.state.subpath_iterator = Some(it);
                        return self.next();
                    }
                    PathCommand::ClosePath => {
                        let start = self.state.current_polyline_start;
                        if self.state.current_position != self.state.current_polyline_start {
                            // ? maybe use discretize_segment here, or maybe drawing a straight line is fine
                            // even if the transform is not line preserving?
                            self.state.set_curr_pos_no_ctrl_pt(&start);
                            return Some(BrokenPolylineCommand::LineTo(
                                self.state.current_polyline_start,
                            ));
                        } else {
                            self.state.set_curr_pos_no_ctrl_pt(&start);
                            return self.next(); // TODO or maybe return MoveTo?
                        }
                    }
                    PathCommand::CubicBezierCurveTo {
                        control_pt_1,
                        control_pt_2,
                        end_pt,
                    } => {
                        let discretizer = curves::discretize_cubic_bezier(
                            self.state.current_position,
                            *control_pt_1,
                            *control_pt_2,
                            *end_pt,
                            true,
                            &self.discretizer.params,
                            &self.discretizer.transform,
                        );
                        self.state
                            .set_current_pos_and_ctrl_pt(&end_pt, &control_pt_2);
                        self.state.subpath_iterator = Some(discretizer);
                        return self.next();
                    }
                    PathCommand::CubicBezierCurveToOffset {
                        control_pt_1_offset,
                        control_pt_2_offset,
                        end_pt_offset,
                    } => {
                        let control_pt_1 = self.state.current_position + control_pt_1_offset;
                        let control_pt_2 = self.state.current_position + control_pt_2_offset;
                        let end_pt = self.state.current_position + end_pt_offset;
                        let discretizer = curves::discretize_cubic_bezier(
                            self.state.current_position,
                            control_pt_1,
                            control_pt_2,
                            end_pt,
                            true,
                            &self.discretizer.params,
                            &self.discretizer.transform,
                        );
                        self.state
                            .set_current_pos_and_ctrl_pt(&end_pt, &control_pt_2);
                        self.state.subpath_iterator = Some(discretizer);
                        return self.next();
                    }
                    PathCommand::SmoothCubicBezierCurveTo {
                        control_pt_2,
                        end_pt,
                    } => {
                        let control_pt_1 = self.state.current_position
                            + (self.state.current_position - self.state.current_control_point);
                        let discretizer = curves::discretize_cubic_bezier(
                            self.state.current_position,
                            control_pt_1,
                            *control_pt_2,
                            *end_pt,
                            true,
                            &self.discretizer.params,
                            &self.discretizer.transform,
                        );
                        self.state
                            .set_current_pos_and_ctrl_pt(&end_pt, &control_pt_2);
                        self.state.subpath_iterator = Some(discretizer);
                        return self.next();
                    }
                    PathCommand::SmoothCubicBezierCurveToOffset {
                        control_pt_2_offset,
                        end_pt_offset,
                    } => {
                        let control_pt_1 = self.state.current_position
                            + (self.state.current_position - self.state.current_control_point);
                        let control_pt_2 = self.state.current_position + control_pt_2_offset;
                        let end_pt = self.state.current_position + end_pt_offset;
                        let discretizer = curves::discretize_cubic_bezier(
                            self.state.current_position,
                            control_pt_1,
                            control_pt_2,
                            end_pt,
                            true,
                            &self.discretizer.params,
                            &self.discretizer.transform,
                        );
                        self.state
                            .set_current_pos_and_ctrl_pt(&end_pt, &control_pt_2);
                        self.state.subpath_iterator = Some(discretizer);
                        return self.next();
                    }
                    PathCommand::QuadraticBezierCurveTo { control_pt, end_pt } => {
                        let discretizer = curves::discretize_quadratic_bezier(
                            self.state.current_position,
                            *control_pt,
                            *end_pt,
                            true,
                            &self.discretizer.params,
                            &self.discretizer.transform,
                        );
                        self.state.set_current_pos_and_ctrl_pt(&end_pt, &control_pt);
                        self.state.subpath_iterator = Some(discretizer);
                        return self.next();
                    }
                    PathCommand::QuadraticBezierCurveToOffset {
                        control_pt_offset,
                        end_pt_offset,
                    } => {
                        let control_pt = self.state.current_position + control_pt_offset;
                        let end_pt = self.state.current_position + end_pt_offset;
                        let discretizer = curves::discretize_quadratic_bezier(
                            self.state.current_position,
                            control_pt,
                            end_pt,
                            true,
                            &self.discretizer.params,
                            &self.discretizer.transform,
                        );
                        self.state.set_current_pos_and_ctrl_pt(&end_pt, &control_pt);
                        self.state.subpath_iterator = Some(discretizer);
                        return self.next();
                    }
                    PathCommand::SmoothQuadraticBezierCurveTo(end_pt) => {
                        let control_pt = self.state.current_position
                            + (self.state.current_position - self.state.current_control_point);
                        let discretizer = curves::discretize_quadratic_bezier(
                            self.state.current_position,
                            control_pt,
                            *end_pt,
                            true,
                            &self.discretizer.params,
                            &self.discretizer.transform,
                        );
                        self.state.set_current_pos_and_ctrl_pt(end_pt, &control_pt);
                        self.state.subpath_iterator = Some(discretizer);
                        return self.next();
                    }
                    PathCommand::SmoothQuadraticBezierCurveToOffset(end_point_offset) => {
                        let control_pt = self.state.current_position
                            + (self.state.current_position - self.state.current_control_point);
                        let end_pt = self.state.current_position + end_point_offset;
                        let discretizer = curves::discretize_quadratic_bezier(
                            self.state.current_position,
                            control_pt,
                            end_pt,
                            true,
                            &self.discretizer.params,
                            &self.discretizer.transform,
                        );
                        self.state.set_current_pos_and_ctrl_pt(&end_pt, &control_pt);
                        self.state.subpath_iterator = Some(discretizer);
                        return self.next();
                    }
                    PathCommand::EllipticalArcTo {
                        radii,
                        x_axis_rotation,
                        large_arc_flag,
                        sweep_flag,
                        end_pt,
                    } => {
                        let discretizer = curves::discretize_elliptical_arc(
                            &radii,
                            *x_axis_rotation,
                            *large_arc_flag,
                            *sweep_flag,
                            self.state.current_position,
                            *end_pt,
                            true,
                            &self.discretizer.params,
                            &self.discretizer.transform,
                        );
                        self.state.set_curr_pos_no_ctrl_pt(&end_pt);
                        self.state.subpath_iterator = Some(discretizer);
                        return self.next();
                    }
                    PathCommand::EllipticalArcToOffset {
                        radii,
                        x_axis_rotation,
                        large_arc_flag,
                        sweep_flag,
                        end_pt_offset,
                    } => {
                        let end_pt = self.state.current_position + end_pt_offset;
                        let discretizer = curves::discretize_elliptical_arc(
                            &radii,
                            *x_axis_rotation,
                            *large_arc_flag,
                            *sweep_flag,
                            self.state.current_position,
                            end_pt,
                            true,
                            &self.discretizer.params,
                            &self.discretizer.transform,
                        );
                        self.state.set_curr_pos_no_ctrl_pt(&end_pt);
                        self.state.subpath_iterator = Some(discretizer);
                        return self.next();
                    }
                }
            }

            // If we have no more path commands, we are done.
            None
        }
    }

    struct PathDiscretizationState<'a> {
        current_position: Vec2f,
        current_polyline_start: Vec2f,
        current_control_point: Vec2f,
        subpath_iterator: Option<Box<dyn Iterator<Item = Vec2f> + 'a>>,
    }

    impl<'a> PathDiscretizationState<'a> {
        fn new() -> Self {
            Self {
                current_position: Vec2f::new(0.0, 0.0),
                current_polyline_start: Vec2f::new(0.0, 0.0),
                current_control_point: Vec2f::new(0.0, 0.0),
                subpath_iterator: None,
            }
        }

        /// Sets the current position and control point to the given point.
        fn set_curr_pos_no_ctrl_pt(&mut self, pt: &Vec2f) {
            self.current_position = *pt;
            self.current_control_point = *pt;
        }

        /// Sets the current position and control point to the given point and sets the current polyline start to the given point.
        fn move_to(&mut self, pt: &Vec2f) {
            self.set_curr_pos_no_ctrl_pt(pt);
            self.current_polyline_start = *pt;
        }

        /// Sets the current position and control point to the given point.
        fn set_current_pos_and_ctrl_pt(&mut self, pos: &Vec2f, ctrl_pt: &Vec2f) {
            self.current_position = *pos;
            self.current_control_point = *ctrl_pt;
        }
    }

    /// A parametric curve is a curve that can be evaluated at any point in the interval `[start, end]`.
    pub trait ParametricCurve {
        fn eval(&self, t: f32) -> Vec2f;
        fn start(&self) -> f32;
        fn end(&self) -> f32;
        fn param_range_width(&self) -> f32 {
            (self.end() - self.start()).abs()
        }

        /// checks if the value is close or past the end of the curve
        fn is_end_or_past_end(&self, t: f32) -> bool {
            // if the value is close to the end, we consider it to be the end
            if (t - self.end()).abs() < std::f32::EPSILON.sqrt() * self.param_range_width() {
                return true;
            }

            // past the end checks
            if self.end() > self.start() {
                // if the curve is increasing, we check if the value is greater than the end
                t >= self.end()
            } else {
                // if the curve is decreasing, we check if the value is less than the end
                t <= self.end()
            }
        }

        /// picks the next t value and returns the next t value, whether the curve is finished, and the step size
        fn next_t_for_step(&self, t: f32, step: f32) -> (f32, bool, f32) {
            if self.is_end_or_past_end(t + step) {
                (self.end(), true, self.end() - t)
            } else {
                (t + step, false, step)
            }
        }
    }

    /// This is an iterator that discretizes a parametric curve into a series of points.
    /// The generated points are guaranteed to be within the given accuracy of the curve.
    ///
    /// ## Notes
    /// At the moment, the generated points just lie on the curve. In the future, we may want to
    /// generate points that may not lie on the curve, but are within the given accuracy.
    /// to minimize the error across the intermediate points.
    pub struct ParametricCurveDiscretizerIterator<'t, 'a> {
        curve: Box<dyn ParametricCurve>,
        params: &'a DiscretizationParams,           // TODO reference
        transform: &'a DiscretizationTransform<'t>, // TODO reference
        skip_first: bool,
        initial_subs: u32,
        state: Option<ParametricCurveDiscretizerState>,
    }

    impl<'t, 'a> ParametricCurveDiscretizerIterator<'t, 'a> {
        pub fn new(
            curve: Box<dyn ParametricCurve>,
            params: &'a DiscretizationParams,
            transform: &'a DiscretizationTransform<'t>,
            skip_first: bool,
            initial_subs: u32,
        ) -> Self {
            Self {
                curve,
                params,
                transform,
                skip_first,
                initial_subs,
                state: None,
            }
        }

        /// check if the given next point satisfies the discretization accuracy conditions
        fn is_ok<'b1, 'c1>(
            state: &ParametricCurveDiscretizerState,
            params: &DiscretizationParams,
            next_point: &Vec2f,
            mid_pt: &Vec2f,
        ) -> bool {
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
            let dist = dist_to_segment(&mid_pt, (&state.pt, &next_point));
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
            let angle = angle_between(&(mid_pt - state.pt), &(next_point - mid_pt));
            // TODO instead of "angle_between" that requires atan2, we could just use the cross product
            // or the dot product of the normalized vectors and cache sin or cos of the max angle
            if angle.abs() > params.max_angle {
                return false;
            }

            // if we reached this point, the point respects the accuracy conditions
            true
        }
    }

    /// The state of the discretizer
    struct ParametricCurveDiscretizerState {
        /// The current `t` value
        t: f32,
        /// The current `t` step size
        dt: f32,
        /// The current point
        pt: Vec2f,
        /// `true` if the curve is finished
        finished: bool, // TODO this is unnecessary, maybe we could set to a value that is past the end of the curve to indicate that it is finished?
    }

    impl<'t, 'a> Iterator for ParametricCurveDiscretizerIterator<'t, 'a> {
        type Item = Vec2f;

        fn next(&mut self) -> Option<Self::Item> {
            // the transformed curve
            let s = |t: f32| {
                return self.transform.eval(self.curve.eval(t));
            };

            if let Some(ref mut state) = self.state {
                if state.finished {
                    return None;
                }

                // pick the next point
                let (next_t, next_is_end, next_pt) = {
                    let (next_t, next_is_end, stepped) =
                        self.curve.next_t_for_step(state.t, state.dt);
                    let next_pt = s(next_t);
                    let mid_pt = s(state.t + stepped / 2.0);
                    let next_pt_is_ok = Self::is_ok(&state, &self.params, &next_pt, &mid_pt);
                    // Now, if the next point is the end and it is ok, we can use it.
                    // But if the next point is not the end and it is ok, we can try to double the step
                    // until we reach the end or the point is not ok to avoid too many points.
                    // If the next point is either the end or not, but it is not ok, we can halve the step
                    // until we reach the end or the point is ok.
                    // note that every time we halve the step, we have to check that the step is greater than MIN_DT
                    if next_is_end && next_pt_is_ok {
                        (next_t, next_is_end, next_pt)
                    } else if !next_is_end && next_pt_is_ok {
                        let mut step = stepped;
                        let mut t = next_t;
                        let mut pt = next_pt;
                        let mut next_is_end = next_is_end;
                        loop {
                            let (next_t, next_is_end_2, stepped) =
                                self.curve.next_t_for_step(state.t, step * 2.0);
                            let next_pt = s(next_t);
                            let mid_pt = s(state.t + stepped / 2.0);
                            let next_pt_is_ok =
                                Self::is_ok(&state, &self.params, &next_pt, &mid_pt);
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
                    } else {
                        // && !next_pt_is_ok
                        let mut step = stepped;
                        let mut t = next_t;
                        let mut pt = next_pt;
                        let mut next_is_end = next_is_end;
                        loop {
                            let (next_t, next_is_end_2, stepped) =
                                self.curve.next_t_for_step(state.t, step / 2.0);
                            let next_pt = s(next_t);
                            let mid_pt = s(state.t + stepped / 2.0);
                            let next_pt_is_ok =
                                Self::is_ok(&state, &self.params, &next_pt, &mid_pt);
                            if stepped.abs()
                                < std::f32::EPSILON.sqrt() * self.curve.param_range_width()
                            {
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
                        (t, next_is_end, pt)
                    }
                };

                let result_pt = next_pt;

                if next_is_end {
                    // we are done with the curve
                    //return None;
                    state.finished = true;
                }

                state.dt = next_t - state.t;
                //prev_pt = Some(curr_point);
                state.pt = next_pt;
                state.t = next_t;

                return Some(result_pt);
            } else {
                // if we are here, self.state is None, i.e. we are at the beginning of the iteration
                // we need to perform some checks and initialize the state

                // we do not want to discretize curves that have non-finite parameter endpoints
                if !self.curve.start().is_finite() || !self.curve.end().is_finite() {
                    // this is a meaningless curve we cannot discretize
                    return None;
                }

                // TODO move this check to the constructor
                // we do not want to discretize curves that have very small parameter range
                if (self.curve.end() - self.curve.start()).abs() < std::f32::EPSILON.sqrt() {
                    // this is a degenerate curve
                    return None;
                }

                // the transformed start point
                let pt = s(self.curve.start());
                if !pt.x.is_finite() || !pt.y.is_finite() {
                    // this is a meaningless curve we cannot discretize
                    return None;
                }

                // the number of subdivisions for the first step,
                // we add because we want `initial_subs` to represent the number of
                // intermediate points, not the number of subdivisions ?!?!?!?!
                let initial_divisions = self.initial_subs + 1;

                // the initial step size
                let dt = (self.curve.end() - self.curve.start()) / (initial_divisions as f32);

                self.state = Some(ParametricCurveDiscretizerState {
                    t: self.curve.start(),
                    dt,
                    pt,
                    finished: false,
                });

                // evaluate the curve at the start point if needed
                if self.skip_first {
                    return self.next();
                } else {
                    return Some(pt);
                }
            }
        }
    }

    /// Some helper functions for parametric curves
    pub mod curves {

        use super::*;

        /// A segment described as a parametric curve
        pub struct ParametricSegment {
            p0: Vec2f,
            p1: Vec2f,
        }

        impl ParametricCurve for ParametricSegment {
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

        /// Iterates over the two points of a segment
        pub struct SegmentPointsIterator {
            begin: Vec2f,
            end: Vec2f,
            index: usize,
        }

        impl SegmentPointsIterator {
            pub fn new(begin: &Vec2f, end: &Vec2f, skip_first: bool) -> Self {
                Self {
                    begin: *begin,
                    end: *end,
                    index: if skip_first { 1 } else { 0 },
                }
            }
        }

        impl Iterator for SegmentPointsIterator {
            type Item = Vec2f;

            fn next(&mut self) -> Option<Self::Item> {
                if self.index == 0 {
                    self.index += 1;
                    Some(self.begin)
                } else if self.index == 1 {
                    self.index += 1;
                    Some(self.end)
                } else {
                    None
                }
            }
        }

        /// Discretizes a segment.
        ///
        /// If the transform is line-preserving, the segment is discretized into two points,
        /// otherwise the segment is discretized using the given discretization parameters.
        pub fn discretize_segment<'a, 't: 'a>(
            p0: Vec2f,
            p1: Vec2f,
            skip_first: bool,
            params: &'a DiscretizationParams,
            transform: &'a DiscretizationTransform<'t>,
        ) -> Box<dyn Iterator<Item = Vec2f> + 'a> {
            if transform.is_line_preserving() {
                let p0_primed = transform.eval(p0);
                let p1_primed = transform.eval(p1);
                let it = SegmentPointsIterator::new(&p0_primed, &p1_primed, skip_first);
                Box::new(it)
            } else {
                let line_curve = ParametricSegment { p0, p1 };
                let discretizer = ParametricCurveDiscretizerIterator::new(
                    Box::new(line_curve),
                    params,
                    transform,
                    skip_first,
                    1,
                ); // TODO maybe zero initial subdivisions is enough?
                Box::new(discretizer)
            }
        }

        /// A parametric cubic 2d bezier curve
        #[derive(Clone, Copy, Debug)]
        struct ParametricCubic2dBezier {
            /// The first pass-through point
            p0: Vec2f,
            /// The first control point
            p1: Vec2f,
            /// The second control point
            p2: Vec2f,
            /// The second pass-through point
            p3: Vec2f,
        }

        impl ParametricCubic2dBezier {
            fn new(p0: Vec2f, p1: Vec2f, p2: Vec2f, p3: Vec2f) -> Self {
                Self { p0, p1, p2, p3 }
            }
        }

        impl ParametricCurve for ParametricCubic2dBezier {
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

        /// Discretizes a cubic bezier curve.
        pub fn discretize_cubic_bezier<'a, 't: 'a>(
            p0: Vec2f,
            p1: Vec2f,
            p2: Vec2f,
            p3: Vec2f,
            skip_first: bool,
            params: &'a DiscretizationParams,
            transform: &'a DiscretizationTransform<'t>,
        ) -> Box<dyn Iterator<Item = Vec2f> + 'a> {
            const DEGREE: u32 = 3;
            let curve = ParametricCubic2dBezier::new(p0, p1, p2, p3);
            let discretizer = ParametricCurveDiscretizerIterator::new(
                Box::new(curve),
                params,
                transform,
                skip_first,
                DEGREE,
            );
            Box::new(discretizer)
        }

        /// A parametric quadratic 2d bezier curve
        struct ParametricQuadratic2dBezier {
            /// The first pass-through point
            p0: Vec2f,
            /// The control point
            p1: Vec2f,
            /// The second pass-through point
            p2: Vec2f,
        }

        impl ParametricQuadratic2dBezier {
            fn new(p0: Vec2f, p1: Vec2f, p2: Vec2f) -> Self {
                Self { p0, p1, p2 }
            }
        }

        impl ParametricCurve for ParametricQuadratic2dBezier {
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

        /// Discretizes a quadratic bezier curve.
        pub fn discretize_quadratic_bezier<'a, 't: 'a>(
            p0: Vec2f,
            p1: Vec2f,
            p2: Vec2f,
            skip_first: bool,
            params: &'a DiscretizationParams,
            transform: &'a DiscretizationTransform<'t>,
        ) -> Box<dyn Iterator<Item = Vec2f> + 'a> {
            const DEGREE: u32 = 2;
            let curve = ParametricQuadratic2dBezier::new(p0, p1, p2);
            let discretizer = ParametricCurveDiscretizerIterator::new(
                Box::new(curve),
                params,
                transform,
                skip_first,
                DEGREE,
            );
            Box::new(discretizer)
        }

        /// A parametric arc function.
        #[derive(Clone, Copy, Debug)]
        pub struct CenterParametricArc {
            center: Vec2f,
            radii: Vec2f,
            start_angle: f32,
            //end_angle: f32,
            sweep: f32,
            x_axis_rotation: f32,
        }

        impl CenterParametricArc {
            fn new(
                center: Vec2f,
                radii: Vec2f,
                start_angle: f32,
                sweep: f32,
                x_axis_rotation: f32,
            ) -> Self {
                Self {
                    center,
                    radii,
                    start_angle,
                    sweep,
                    x_axis_rotation,
                }
            }
        }

        impl ParametricCurve for CenterParametricArc {
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

        /// Converts an endpoint parametric arc function to a center parametric arc function.
        pub fn endpoint_to_center_parametric_arc_function(
            radii: &Vec2f,
            x_axis_rotation: f32,
            large_arc_flag: bool,
            sweep_flag: bool,
            p0: Vec2f,
            p1: Vec2f,
        ) -> CenterParametricArc {
            // https://www.w3.org/TR/SVG/implnote.html
            // https://math.stackexchange.com/questions/4285747/last-step-of-conversion-from-endpoint-to-center-parameterization-of-an-elliptica
            // https://crates.io/crates/contrast_renderer

            let (phi_rot_mat, phi_inv_rot_mat) = {
                let cos = x_axis_rotation.cos();
                let sin = x_axis_rotation.sin();
                (
                    Mat2f::new(cos, -sin, sin, cos),
                    Mat2f::new(cos, sin, -sin, cos),
                )
            };

            let p0_primed = phi_inv_rot_mat * (p0 - p1) * 0.5;

            let lambda = (sqr(p0_primed.x) / sqr(radii.x)) + (sqr(p0_primed.y) / sqr(radii.y));

            let radii: Vec2f = {
                if lambda > 1.0 {
                    radii * lambda.sqrt()
                } else {
                    *radii
                }
            };
            let (rx, ry) = (radii.x, radii.y);

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

            let c = phi_rot_mat * c_primed + (p0 + p1) * 0.5;

            //let angle_between = |v0: Vec2f, v1: Vec2f| {
            //    // TODO by copilot, very clever! But to check...
            //    let dot = v0.dot(&v1);
            //    let det = v0.x * v1.y - v0.y * v1.x;
            //    det.atan2(dot)
            //};

            let theta_0 = angle_between(
                &Vec2f::new(1.0, 0.0),
                &(p0_primed - c_primed).component_div(&radii),
            );
            // TODO this mod may be wrong...
            let delta_theta = angle_between(
                &(p0_primed - c_primed).component_div(&radii),
                &(-p0_primed - c_primed).component_div(&radii),
            ) % (2.0 * std::f32::consts::PI);
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

            CenterParametricArc::new(c, radii, theta_0, delta_theta, x_axis_rotation)
        }

        /// Discretizes an elliptical arc.
        pub fn discretize_elliptical_arc<'a, 't: 'a>(
            radii: &Vec2f,
            x_axis_rotation: f32,
            large_arc_flag: bool,
            sweep_flag: bool,
            p0: Vec2f,
            p1: Vec2f,
            skip_first: bool,
            params: &'a DiscretizationParams,
            transform: &'a DiscretizationTransform<'t>,
        ) -> Box<dyn Iterator<Item = Vec2f> + 'a> {
            let arc = endpoint_to_center_parametric_arc_function(
                radii,
                x_axis_rotation,
                large_arc_flag,
                sweep_flag,
                p0,
                p1,
            );
            let discretizer = ParametricCurveDiscretizerIterator::new(
                Box::new(arc),
                params,
                transform,
                skip_first,
                1,
            );
            Box::new(discretizer)
        }
    }

    #[inline]
    pub fn lerp<T, V>(a: V, b: V, t: T) -> V
    where
        T: Copy,
        V: Mul<T, Output = V> + Add<V, Output = V>,
        f32: Sub<T, Output = T>,
    {
        a * (1.0 - t) + b * t
    }

    #[inline]
    pub fn sqr<T>(x: T) -> <T as Mul>::Output
    where
        T: Mul<T> + Copy,
    {
        x * x
    }

    pub fn dist_to_segment(pt: &Vec2f, segment: (&Vec2f, &Vec2f)) -> f32 {
        let (a, b) = segment;
        let ab = b - a;
        let ap = pt - a;
        let t = ap.dot(&ab) / ab.dot(&ab);
        let closest = lerp(*a, *b, t);
        (closest - pt).norm()
    }

    #[inline]
    pub fn angle_between(v0: &Vec2f, v1: &Vec2f) -> f32 {
        // TODO by copilot, very clever! But to check...
        let dot = v0.dot(&v1);
        let det = v0.x * v1.y - v0.y * v1.x;
        det.atan2(dot)
    }
}
