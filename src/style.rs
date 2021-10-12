use crate::Color;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum PointType {
    Circle,
    Symbol,
    Text,
    Poi,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Transform {
    transform_functions: Vec<TransformFn>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransformFn {
    Matrix(f64, f64, f64, f64, f64, f64),
    Translate(f64, Option<f64>),
    Scale(f64, Option<f64>),
    Rotate(f64, Option<(f64, f64)>),
    SkewX(f64),
    SkewY(f64),
}

impl Transform {
    pub fn new(transform_functions: Vec<TransformFn>) -> Self {
        Self {
            transform_functions,
        }
    }
}

impl Display for Transform {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        let transform_fns = self
            .transform_functions
            .iter()
            .fold("".to_string(), |mut acc, f| {
                acc.push_str(&f.to_string());
                acc
            });
        write!(fmt, "{}", &transform_fns)
    }
}

impl Display for TransformFn {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result {
        match self {
            TransformFn::Matrix(a, b, c, d, e, f) => {
                write!(fmt, "matrix({}, {}, {}, {}, {}, {})", a, b, c, d, e, f)
            }

            TransformFn::Translate(x, y) => match y {
                Some(y) => write!(fmt, "translate({}, {})", x, y),
                None => write!(fmt, "translate({})", x),
            },

            TransformFn::Scale(x, y) => match y {
                Some(y) => write!(fmt, "scale({}, {})", x, y),
                None => write!(fmt, "scale({})", x),
            },

            TransformFn::Rotate(r, p) => match p {
                Some((x, y)) => write!(fmt, "rotate({}, {}, {})", r, x, y),
                None => write!(fmt, "rotate({})", r),
            },

            TransformFn::SkewX(a) => write!(fmt, "skewX({})", a),

            TransformFn::SkewY(a) => write!(fmt, "skewY({})", a),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Style {
    pub opacity: Option<f32>,
    pub fill: Option<Color>,
    pub fill_opacity: Option<f32>,
    pub stroke_color: Option<Color>,
    pub stroke_width: Option<f32>,
    pub stroke_opacity: Option<f32>,
    pub radius: f32,
    pub css_classes: Option<String>,
    pub id: Option<String>,
    pub point_type: Option<PointType>,
    pub icon_svg_path: Option<String>,
    pub icon_svg_viewbox: Option<(i32, i32, i32, i32)>,
    pub icon_svg_width_height: Option<(i32, i32)>,
    pub text: Option<String>,
    pub text_start_offset: Option<f64>,
    pub transform: Option<Transform>,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            opacity: None,
            fill: None,
            fill_opacity: None,
            stroke_color: None,
            stroke_width: None,
            stroke_opacity: None,
            radius: 1.0,
            css_classes: None,
            id: None,
            point_type: None,
            icon_svg_path: None,
            icon_svg_viewbox: None,
            icon_svg_width_height: None,
            text: None,
            text_start_offset: None,
            transform: None,
        }
    }
}

impl Display for Style {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        if let Some(opacity) = self.opacity {
            write!(fmt, r#" opacity="{}""#, opacity)?;
        }
        if let Some(fill) = self.fill {
            write!(fmt, r#" fill="{}""#, fill)?;
        }
        if let Some(fill_opacity) = self.fill_opacity {
            write!(fmt, r#" fill-opacity="{}""#, fill_opacity)?;
        }
        if let Some(stroke_color) = self.stroke_color {
            write!(fmt, r#" stroke="{}""#, stroke_color)?;
        }
        if let Some(stroke_width) = self.stroke_width {
            write!(fmt, r#" stroke-width="{}""#, stroke_width)?;
        }
        if let Some(stroke_opacity) = self.stroke_opacity {
            write!(fmt, r#" stroke-opacity="{}""#, stroke_opacity)?;
        }
        if let Some(css_classes) = self.css_classes.clone() {
            write!(fmt, r#" class="{}""#, css_classes)?;
        }
        if let Some(id) = self.id.clone() {
            write!(fmt, r#" id="{}""#, id)?;
        }
        if let Some(transform) = &self.transform {
            write!(fmt, r#" transform="{}""#, transform)?;
        }
        Ok(())
    }
}
