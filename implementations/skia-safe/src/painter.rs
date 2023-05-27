
use repaint::{BasicPainter, base::{shapes::{path::PathCommand, Shape, BasicShape}, defs::{colors::default_color_types::RgbaFColor, linalg::Vec2f64}, paint::{Paint, Ink}, pen::Pen, blending::BlendMode, transform::Transform2d}, nalgebra::Matrix4, SaveLayerRec, methods::{TransformError, ClipError, PaintStyle}, ClipOperation, WithPathResource, Context, WithText, FontStyle, FontWeight, FontWidth, FontSlant, PointMode, Canvas, RasterPainter, LocalizedString, FontVariationParameter, FontVariationAxisTag, FontVariationAxis};
use skia_safe::{font_arguments::VariationPosition, FourByteTag};

use crate::{SkiaCanvas, conversions::{create_skia_path, paint_style_to_skia_paint, color_to_skia_color}, IntoSkiaCorrespondingType};




pub struct SkiaContext {
}

impl SkiaContext{
    pub fn new() -> Self {
        Self {
        }
    }
}

impl Context for SkiaContext {
    //fn make_path(&mut self, path_iter: &mut dyn Iterator<Item = PathCommand>) -> Result<PathResource<'context>, ()> { // TODO proper error
    //    Ok(PathResource(PainterResource::new(
    //        Rc::new(create_skia_path(path_iter)),
    //        self.lifecycle()
    //    )))
    //}
}

struct Buffers {
    points: Vec<skia_safe::Point>,
}

impl Buffers {
    fn new() -> Self {
        Self {
            points: Vec::new(),
        }
    }
}

pub struct SkiaPainter<'canvas, 'surface, 'context> {
    canvas: &'canvas mut SkiaCanvas<'surface, 'context>,
    buffers: Buffers,
    font_namager: FontManager,
}

impl<'canvas, 'surface, 'context> SkiaPainter<'canvas, 'surface, 'context> {
    pub fn new(canvas: &'canvas mut SkiaCanvas<'surface, 'context>) -> Self {
        Self {
            canvas,
            buffers: Buffers::new(),
            font_namager: FontManager(skia_safe::FontMgr::new()), // TODO remove and move in the context
        }
    }

    /// The underlying Skia canvas.
    /// **Waring**: This method exposes the underlying Skia canvas allowing
    /// the user to perform. Use with care (especially with `save`/`restore`).
    pub fn skia_canvas_mut(&mut self) -> &mut skia_safe::Canvas {
        self.canvas.surface.canvas()
    }
}

impl<'canvas, 'surface, 'context> BasicPainter<'context> for SkiaPainter<'canvas, 'surface, 'context> {
    type NativeColor = RgbaFColor;

    //type Resources = SkiaResources;
    type Canvas = SkiaCanvas<'surface, 'context>;
    type Context = SkiaContext;

    fn canvas(&self) -> &Self::Canvas {
        self.canvas
    }

    fn is_blend_mode_valid(&self, _mode: BlendMode) -> bool {
        todo!()
    }

    fn with_save<'a, R>(
        &'a mut self,
        paint: impl FnOnce(&mut Self) -> R,
    ) -> R {
        self.skia_canvas_mut().save();
        let r = paint(self);
        self.skia_canvas_mut().restore();
        r
    }

    fn with_save_layer<'a, R>(
        &'a mut self,
        paint: impl FnOnce(&mut Self) -> R,
        layer_rec: SaveLayerRec,
    ) -> R {
        let mut skia_layer_rec: skia_safe::canvas::SaveLayerRec = Default::default();
        let rect: skia_safe::Rect;
        if let Some(bounds) = layer_rec.bounds {
            rect = skia_safe::Rect::new(
                bounds.min.x as f32,
                bounds.min.y as f32,
                bounds.max.x as f32,
                bounds.max.y as f32,
            );
            skia_layer_rec = skia_layer_rec.bounds(&rect);
        }
        // TODO ...
        self.skia_canvas_mut().save_layer(&skia_layer_rec);
        let r = paint(self);
        self.skia_canvas_mut().restore();
        r
    }

    fn set_transform(&mut self, transform: &Transform2d) -> Result<(), TransformError> {
        use skia_safe::M44;

        let matrix: M44 = if let Some(mat) = transform.to_mat4x4() {
            let mat: Matrix4<f32> = mat.cast();
            let s = mat.as_slice();
            M44::row_major(&[
                s[0], s[1], s[2], s[3],
                s[4], s[5], s[6], s[7],
                s[8], s[9], s[10], s[11],
                s[12], s[13], s[14], s[15]
            ])
        } else {
            return Err(TransformError::Unsupported);
        };

        self.skia_canvas_mut().set_matrix(&matrix);
        Ok(())
    }

    fn concatenate_transform(&mut self, transform: &Transform2d) -> Result<(), TransformError> {
        use skia_safe::M44;
        let matrix: M44 = if let Some(mat) = transform.to_mat4x4() {
            let m: Matrix4<f32> = mat.cast();
            M44::row_major(&[
                m[(0, 0)], m[(0, 1)], m[(0, 2)], m[(0, 3)],
                m[(1, 0)], m[(1, 1)], m[(1, 2)], m[(1, 3)],
                m[(2, 0)], m[(2, 1)], m[(2, 2)], m[(2, 3)],
                m[(3, 0)], m[(3, 1)], m[(3, 2)], m[(3, 3)],
            ])
        } else {
            return Err(TransformError::Unsupported);
        };

        // TODO use when possible self.skia_canvas_mut().concat(matrix)
        self.skia_canvas_mut().concat_44(&matrix);
        Ok(())
    }

    fn clip(&mut self, shape: &impl Shape, clip_operation: ClipOperation) -> Result<(), ClipError> {
        //self.skia_canvas_mut().clip_path(path, op, do_anti_alias)
        use skia_safe::ClipOp;
        let op = match clip_operation {
            ClipOperation::Difference => ClipOp::Difference,
            ClipOperation::Intersect => ClipOp::Intersect,
        };
        if let Some(basic) = shape.to_basic_shape() {
            match basic {
                BasicShape::Rect(rect) => {
                    self.skia_canvas_mut().clip_rect(
                        skia_safe::Rect::new(
                            rect.min.x as f32,
                            rect.min.y as f32,
                            rect.max.x as f32,
                            rect.max.y as f32,
                        ),
                        op,
                        true, // TODO ???
                    );
                    return Ok(());
                }
                // TODO other cases
                _ => {}
            }
        }
        let path = create_skia_path(shape.to_path_iter());
        self.skia_canvas_mut().clip_path(&path, op, true);
        Ok(())
    }

    fn point(
        &mut self,
        pos: Vec2f64,
        style: PaintStyle<Self::NativeColor>,
    ) {
        self.skia_canvas_mut().draw_point(
            (pos.x as f32, pos.y as f32),
            &paint_style_to_skia_paint(&style),
        );
        // TODO points
    }

    fn points(
        &mut self,
        points: impl IntoIterator<Item = Vec2f64>,
        style: PaintStyle<Self::NativeColor>,
        point_mode: PointMode,
    ) {
        self.buffers.points.clear();
        for point in points {
            self.buffers.points.push(skia_safe::Point::new(point.x as f32, point.y as f32));
        }
        let point_mode = match point_mode {
            PointMode::Points => skia_safe::canvas::PointMode::Points,
            PointMode::Lines => skia_safe::canvas::PointMode::Lines,
            PointMode::Polygon => skia_safe::canvas::PointMode::Polygon,
        };
        self.canvas.surface.canvas().draw_points(
            point_mode,
            &self.buffers.points,
            &paint_style_to_skia_paint(&style)
        );
        self.buffers.points.clear();
    }

    fn line(
        &mut self,
        start: Vec2f64,
        end: Vec2f64,
        pen: Pen<Self::NativeColor>,
    ) {
        self.canvas.surface.canvas().draw_line(
            (start.x as f32, start.y as f32),
            (end.x as f32, end.y as f32),
            &paint_style_to_skia_paint(&PaintStyle::Stroke(pen))
        );
    }

    fn draw_path_iter<'s, 'a>(
        &'s mut self,
        path_iter: &mut dyn Iterator<Item = PathCommand>,
        style: PaintStyle<Self::NativeColor>,
    ) {
        let path = create_skia_path(path_iter);
        let paint = paint_style_to_skia_paint(&style);
        self.canvas.surface.canvas().draw_path(path.as_ref(), &paint);
    }

    fn clear(
        &mut self,
        color: Self::NativeColor,
    ) {
        self.canvas.surface.canvas().clear(color_to_skia_color(color));
    }

    fn fill_with(
        &mut self,
        paint: &Paint<Self::NativeColor>,
    ) {
        let rect = self.canvas.shape().into_skia();
        self.skia_canvas_mut().draw_rect(
            rect,
            &paint_style_to_skia_paint(&PaintStyle::Fill(paint.clone())),
        );
        todo!()
    }
}

impl<'canvas, 'surface, 'context> RasterPainter<'context> for SkiaPainter<'canvas, 'surface, 'context> {
    fn has_antialias(&self) -> bool {
        true
    }

    fn pixel(
        &mut self,
        pos: Vec2f64,
        ink: Ink<Self::NativeColor>,
    ) {
        self.point(pos, PaintStyle::Fill(ink.into()));
    }
}

impl<'canvas, 'surface, 'context> WithPathResource<'context> for SkiaPainter<'canvas, 'surface, 'context> {
    type Path = skia_safe::Path; // TODO ??

    fn make_path(
        &mut self,
        path_iter: impl Iterator<Item = PathCommand>
    ) -> Result<Self::Path, ()> {
        let path = create_skia_path(path_iter);
        Ok(path)
    }

    fn path(&mut self, path: &Self::Path, style: PaintStyle<Self::NativeColor>) {
        let paint = paint_style_to_skia_paint(&style);
        self.canvas.surface.canvas().draw_path(path.as_ref(), &paint);
    }
}

pub struct Typeface(skia_safe::Typeface);

impl repaint::Typeface for Typeface {
    fn style(&self) -> FontStyle {
        let sk_style = self.0.font_style();
        FontStyle {
            weight: from_skia_weight(sk_style.weight()),
            width: from_skia_width(sk_style.width()),
            slant: from_skia_slant(sk_style.slant()),
        }
    }

    fn is_fixed_pitch(&self) -> bool {
        self.0.is_fixed_pitch()
    }

    fn design_parameters(&self) -> Vec<FontVariationParameter> {
        let axes = self.0.variation_design_parameters();
        let coords = self.0.variation_design_position();
        println!("axes: {:?}, coords: {:?}", axes, coords);
        debug_assert_eq!(axes.is_some(), coords.is_some());
        if axes.is_none() || coords.is_none() {
            return vec![];
        }
        let parameters = axes.unwrap();
        let coords = coords.unwrap();
        debug_assert_eq!(parameters.len(), coords.len());
        parameters.iter().zip(coords.iter()).map(|(axis, coord)| {
            assert_eq!(coord.axis, axis.tag);
            FontVariationParameter {
                axis: FontVariationAxis {
                    tag: FontVariationAxisTag::from_bytes([axis.tag.a(), axis.tag.b(), axis.tag.c(), axis.tag.d()]),
                    range: axis.min..=axis.max,
                    default: axis.def,
                },
                value: coord.value,
            }
        }).collect()
    }

    fn with_parameters(self, parameters: &[FontVariationParameter]) -> Option<Self> {
        let coords: Vec<skia_safe::font_arguments::variation_position::Coordinate> = parameters
            .iter()
            .map(|p| skia_safe::font_arguments::variation_position::Coordinate {
                    axis: FourByteTag::from_chars(
                        p.axis.tag.tag()[0] as char,
                        p.axis.tag.tag()[1] as char,
                        p.axis.tag.tag()[2] as char,
                        p.axis.tag.tag()[3] as char
                    ),
                    value: p.value,
            })
            .collect();
        let args = skia_safe::FontArguments::new()
            .set_variation_design_position(VariationPosition {
                coordinates: &coords,
            });
        // TODO args.set_palette(palette) ????
        Some(Typeface(self.0.clone_with_arguments(&args)?))
    }

    fn family_name(&self) -> String {
        self.0.family_name()
    }

    fn enumerate_family_names(&self) -> Vec<LocalizedString> {
        self.0.new_family_name_iterator().map(|name| LocalizedString { string: name.string, language: name.language }).collect()
    }
}

pub struct Font(skia_safe::Font);

impl Font {
    pub fn new(sk_font: skia_safe::Font) -> Self {
        Self(sk_font)
    }
}

impl repaint::Font for Font {
    type Typeface = Typeface;
}

pub struct TextBlob(skia_safe::TextBlob);

impl repaint::TextBlob for TextBlob {
}

pub struct FontManager(skia_safe::FontMgr);

impl repaint::FontManager for FontManager {
    type Iter = std::vec::IntoIter<String>; // TODO inefficient
    fn families(&self) -> Self::Iter {
        self.0.family_names().collect::<Vec<_>>().into_iter()
    }
}

impl<'canvas, 'surface, 'context> WithText<'context> for SkiaPainter<'canvas, 'surface, 'context> {
    type Typeface = Typeface;
    type Font = Font;
    type TextBlob = TextBlob;
    type FontManager = FontManager;

    fn font_manager(&self) -> &Self::FontManager {
        &self.font_namager
    }

    fn typeface(&mut self, family_name: &str, style: FontStyle) -> Self::Typeface {
        let sk_style = skia_safe::FontStyle::new(
            to_skia_weight(style.weight),
            to_skia_width(style.width),
            to_skia_slant(style.slant),
        );
        let typeface = skia_safe::Typeface::from_name(family_name, sk_style).unwrap();
        //let in_file = std::fs::read("RoboFlex.ttf").unwrap();
        //println!("in_file: {:?}", in_file.len());
        //let data = unsafe {skia_safe::Data::new_bytes(&in_file) };
        //skia_safe::Typeface::from_data(data, 0).unwrap();
        Typeface(typeface)
    }

    fn font(&mut self, typeface: &Self::Typeface, size: f32) -> Self::Font {
        let font = skia_safe::Font::from_typeface(&typeface.0, size);
        Font(font)
    }

    fn make_text_blob(&mut self, text: &str, font: &Self::Font) -> Self::TextBlob {
        let blob = skia_safe::TextBlob::new(text, &font.0).unwrap();
        TextBlob(blob)
    }

    fn draw_text_blob(&mut self, text_blob: &Self::TextBlob, pos: Vec2f64, style: PaintStyle<Self::NativeColor>) {
        let paint = paint_style_to_skia_paint(&style);
        self.canvas.surface.canvas().draw_text_blob(text_blob.0.as_ref(), (pos.x as f32, pos.y as f32), &paint);
    }
}

fn to_skia_weight(weight: FontWeight) -> skia_safe::font_style::Weight {
    match weight {
        FontWeight::Invisible => skia_safe::font_style::Weight::INVISIBLE,
        FontWeight::Thin => skia_safe::font_style::Weight::THIN,
        FontWeight::ExtraLight => skia_safe::font_style::Weight::EXTRA_LIGHT,
        FontWeight::Light => skia_safe::font_style::Weight::LIGHT,
        FontWeight::Normal => skia_safe::font_style::Weight::NORMAL,
        FontWeight::Medium => skia_safe::font_style::Weight::MEDIUM,
        FontWeight::SemiBold => skia_safe::font_style::Weight::SEMI_BOLD,
        FontWeight::Bold => skia_safe::font_style::Weight::BOLD,
        FontWeight::ExtraBold => skia_safe::font_style::Weight::EXTRA_BOLD,
        FontWeight::Black => skia_safe::font_style::Weight::BLACK,
        FontWeight::ExtraBlack => skia_safe::font_style::Weight::EXTRA_BLACK,
        FontWeight::Custom(weight) => weight.into(),
    }
}

fn from_skia_weight(weight: skia_safe::font_style::Weight) -> FontWeight {
    match weight {
        skia_safe::font_style::Weight::INVISIBLE => FontWeight::Invisible,
        skia_safe::font_style::Weight::THIN => FontWeight::Thin,
        skia_safe::font_style::Weight::EXTRA_LIGHT => FontWeight::ExtraLight,
        skia_safe::font_style::Weight::LIGHT => FontWeight::Light,
        skia_safe::font_style::Weight::NORMAL => FontWeight::Normal,
        skia_safe::font_style::Weight::MEDIUM => FontWeight::Medium,
        skia_safe::font_style::Weight::SEMI_BOLD => FontWeight::SemiBold,
        skia_safe::font_style::Weight::BOLD => FontWeight::Bold,
        skia_safe::font_style::Weight::EXTRA_BOLD => FontWeight::ExtraBold,
        skia_safe::font_style::Weight::BLACK => FontWeight::Black,
        skia_safe::font_style::Weight::EXTRA_BLACK => FontWeight::ExtraBlack,
        _ => FontWeight::from_number(*weight)
    }
}

fn to_skia_width(width: FontWidth) -> skia_safe::font_style::Width {
    match width {
        FontWidth::UltraCondensed => skia_safe::font_style::Width::ULTRA_CONDENSED,
        FontWidth::ExtraCondensed => skia_safe::font_style::Width::EXTRA_CONDENSED,
        FontWidth::Condensed => skia_safe::font_style::Width::CONDENSED,
        FontWidth::SemiCondensed => skia_safe::font_style::Width::SEMI_CONDENSED,
        FontWidth::Normal => skia_safe::font_style::Width::NORMAL,
        FontWidth::SemiExpanded => skia_safe::font_style::Width::SEMI_EXPANDED,
        FontWidth::Expanded => skia_safe::font_style::Width::EXPANDED,
        FontWidth::ExtraExpanded => skia_safe::font_style::Width::EXTRA_EXPANDED,
        FontWidth::UltraExpanded => skia_safe::font_style::Width::ULTRA_EXPANDED,
        FontWidth::Custom(width) => width.into(),
    }
}

fn from_skia_width(width: skia_safe::font_style::Width) -> FontWidth {
    match width {
        skia_safe::font_style::Width::ULTRA_CONDENSED => FontWidth::UltraCondensed,
        skia_safe::font_style::Width::EXTRA_CONDENSED => FontWidth::ExtraCondensed,
        skia_safe::font_style::Width::CONDENSED => FontWidth::Condensed,
        skia_safe::font_style::Width::SEMI_CONDENSED => FontWidth::SemiCondensed,
        skia_safe::font_style::Width::NORMAL => FontWidth::Normal,
        skia_safe::font_style::Width::SEMI_EXPANDED => FontWidth::SemiExpanded,
        skia_safe::font_style::Width::EXPANDED => FontWidth::Expanded,
        skia_safe::font_style::Width::EXTRA_EXPANDED => FontWidth::ExtraExpanded,
        skia_safe::font_style::Width::ULTRA_EXPANDED => FontWidth::UltraExpanded,
        _ => FontWidth::Custom(*width),
    }
}

fn to_skia_slant(slant: FontSlant) -> skia_safe::font_style::Slant {
    match slant {
        FontSlant::Upright => skia_safe::font_style::Slant::Upright,
        FontSlant::Italic => skia_safe::font_style::Slant::Italic,
        FontSlant::Oblique => skia_safe::font_style::Slant::Oblique,
    }
}

fn from_skia_slant(slant: skia_safe::font_style::Slant) -> FontSlant {
    match slant {
        skia_safe::font_style::Slant::Upright => FontSlant::Upright,
        skia_safe::font_style::Slant::Italic => FontSlant::Italic,
        skia_safe::font_style::Slant::Oblique => FontSlant::Oblique,
    }
}