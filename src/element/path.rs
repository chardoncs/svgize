use quick_xml::events::BytesStart;

use crate::{attr::{impl_attr_accessors, LazyAttrMap}, element::{convert_into_xml, TagName}, push_attr, Point};

use super::{impl_accessor, impl_element, WriteXml};

/// Path element (`<path>`)
///
/// See [MDN](https://developer.mozilla.org/en-US/docs/Web/SVG/Element/path).
pub struct Path {
    data: Option<String>,
    path_length: Option<f32>,
    
    attr: LazyAttrMap,
}

impl Default for Path {
    #[inline]
    #[must_use]
    fn default() -> Self {
        Self {
            data: None,
            path_length: None,
            attr: None,
        }
    }
}

impl Path {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_data<T>(data: T) -> Self
    where
        T: ToString,
    {
        let mut p = Self::default();
        p.set_data(Some(&data));

        p
    }

    impl_accessor!(string* -> data, set_data, "d");
    impl_accessor!(primitive -> path_length, set_path_length, f32, "pathLength");
}

impl WriteXml for Path {
    fn write_xml(&self, writer: &mut quick_xml::Writer<std::io::Cursor<Vec<u8>>>) -> Result<(), crate::error::Error> {
        let tag = Self::tag_name();
            
        let mut bs = BytesStart::new(tag);

        push_attr!(self.data, bs, "d" <- String);
        push_attr!(self.path_length, bs, "pathLength" <- prim);

        push_attr!(map: self.attr, bs);

        convert_into_xml(writer, bs, None, tag)
    }
}

impl_element!(Path?, "path");

impl_attr_accessors!(Path);

#[derive(PartialEq)]
enum PathCommandKind {
    MoveToAbs(Point),
    MoveToRel(Point),
    LineAbs(Point),
    LineRel(Point),
    HorizontalAbs(f32),
    HorizontalRel(f32),
    VerticalAbs(f32),
    VerticalRel(f32),
    CubicBezierAbs {
        con_start: Point,
        con_end: Point,
        to: Point,
    },
    CubicBezierRel {
        d_con_start: Point,
        d_con_end: Point,
        d_to: Point,
    },
    SmoothCubicBezierAbs {
        con_end: Point,
        to: Point,
    },
    SmoothCubicBezierRel {
        d_con_end: Point,
        d_to: Point,
    },
    QuadraticBezierAbs {
        con: Point,
        to: Point,
    },
    QuadraticBezierRel {
        d_con: Point,
        d_to: Point,
    },
    SmoothQuadraticBezierAbs(Point),
    SmoothQuadraticBezierRel(Point),
    EllipticalArcAbs {
        radius_x: f32,
        radius_y: f32,
        angle: f32,
        laf: LargeArcFlag,
        sf: SweepFlag,
        shift: Point,
    },
    EllipticalArcRel {
        radius_x: f32,
        radius_y: f32,
        angle: f32,
        laf: LargeArcFlag,
        sf: SweepFlag,
        d_shift: Point,
    },
    Close,
}

#[derive(PartialEq, Clone, Copy)]
pub enum LargeArcFlag {
    Small = 0,
    Large,
}

#[derive(PartialEq, Clone, Copy)]
pub enum SweepFlag {
    CounterClockwise = 0,
    Clockwise,
}

pub struct PathData {
    cmds: Vec<PathCommandKind>,
}

impl PathData {
    pub fn move_to(&mut self, point: Point) -> &mut Self {
        self.cmds.push(PathCommandKind::MoveToAbs(point));
        self
    }

    pub fn move_to_rel(&mut self, delta: Point) -> &mut Self {
        self.cmds.push(PathCommandKind::MoveToRel(delta));
        self
    }

    pub fn draw_line(&mut self, to: Point) -> &mut Self {
        self.cmds.push(PathCommandKind::LineAbs(to));
        self
    }

    pub fn draw_line_rel(&mut self, delta_to: Point) -> &mut Self {
        self.cmds.push(PathCommandKind::LineRel(delta_to));
        self
    }

    pub fn draw_horizonal_line(&mut self, x: f32) -> &mut Self {
        self.cmds.push(PathCommandKind::HorizontalAbs(x));
        self
    }

    pub fn draw_horizontal_line_rel(&mut self, dx: f32) -> &mut Self {
        self.cmds.push(PathCommandKind::HorizontalRel(dx));
        self
    }

    pub fn draw_vertical_line(&mut self, y: f32) -> &mut Self {
        self.cmds.push(PathCommandKind::VerticalAbs(y));
        self
    }

    pub fn draw_vertical_line_rel(&mut self, dy: f32) -> &mut Self {
        self.cmds.push(PathCommandKind::VerticalRel(dy));
        self
    }

    pub fn draw_cubic_bezier(&mut self, con_start: Point, con_end: Point, to: Point) -> &mut Self {
        self.cmds.push(PathCommandKind::CubicBezierAbs { con_start, con_end, to });
        self
    }

    pub fn draw_cubic_bezier_rel(&mut self, d_con_start: Point, d_con_end: Point, d_to: Point) -> &mut Self {
        self.cmds.push(PathCommandKind::CubicBezierRel { d_con_start, d_con_end, d_to });
        self
    }

    pub fn draw_smooth_cubic_bezier(&mut self, con_end: Point, to: Point) -> &mut Self {
        self.cmds.push(PathCommandKind::SmoothCubicBezierAbs { con_end, to });
        self
    }

    pub fn draw_smooth_cubic_bezier_rel(&mut self, d_con_end: Point, d_to: Point) -> &mut Self {
        self.cmds.push(PathCommandKind::SmoothCubicBezierRel { d_con_end, d_to });
        self
    }

    pub fn draw_quadratic_bezier(&mut self, con: Point, to: Point) -> &mut Self {
        self.cmds.push(PathCommandKind::QuadraticBezierAbs { con, to });
        self
    }

    pub fn draw_quadratic_bezier_rel(&mut self, d_con: Point, d_to: Point) -> &mut Self {
        self.cmds.push(PathCommandKind::QuadraticBezierRel { d_con, d_to });
        self
    }

    pub fn draw_smooth_quadratic_bezier(&mut self, to: Point) -> &mut Self {
        self.cmds.push(PathCommandKind::SmoothQuadraticBezierAbs(to));
        self
    }

    pub fn draw_smooth_quadratic_bezier_rel(&mut self, d_to: Point) -> &mut Self {
        self.cmds.push(PathCommandKind::SmoothQuadraticBezierRel(d_to));
        self
    }

    pub fn draw_elliptical_arc(
        &mut self,
        rx: f32, ry: f32,
        angle: f32,
        large_arc_flag: LargeArcFlag,
        sweep_flag: SweepFlag,
        shift: Point
    ) -> &mut Self {
        self.cmds.push(PathCommandKind::EllipticalArcAbs {
            radius_x: rx,
            radius_y: ry,
            angle,
            laf: large_arc_flag,
            sf: sweep_flag,
            shift,
        });
        self
    }
   
    pub fn draw_elliptical_arc_rel(
        &mut self,
        rx: f32, ry: f32,
        angle: f32,
        large_arc_flag: LargeArcFlag,
        sweep_flag: SweepFlag,
        d_shift: Point
    ) -> &mut Self {
        self.cmds.push(PathCommandKind::EllipticalArcRel {
            radius_x: rx,
            radius_y: ry,
            angle,
            laf: large_arc_flag,
            sf: sweep_flag,
            d_shift,
        });
        self
    }

    pub fn close(&mut self) -> &mut Self {
        self.cmds.push(PathCommandKind::Close);
        self
    }
}

fn to_cmd_str(cmd: &PathCommandKind) -> String {
    match cmd {
        PathCommandKind::MoveToAbs(_) => "M",
        PathCommandKind::MoveToRel(_) => "m",
        PathCommandKind::LineAbs(_) => "L",
        PathCommandKind::LineRel(_) => "l",
        PathCommandKind::HorizontalAbs(_) => "H",
        PathCommandKind::HorizontalRel(_) => "h",
        PathCommandKind::VerticalAbs(_) => "V",
        PathCommandKind::VerticalRel(_) => "v",
        PathCommandKind::CubicBezierAbs { .. } => "C",
        PathCommandKind::CubicBezierRel { .. } => "c",
        PathCommandKind::SmoothCubicBezierAbs { .. } => "S",
        PathCommandKind::SmoothCubicBezierRel { .. } => "s",
        PathCommandKind::QuadraticBezierAbs { .. } => "Q",
        PathCommandKind::QuadraticBezierRel { .. } => "q",
        PathCommandKind::SmoothQuadraticBezierAbs(_) => "T",
        PathCommandKind::SmoothQuadraticBezierRel(_) => "t",
        PathCommandKind::EllipticalArcAbs { .. } => "A",
        PathCommandKind::EllipticalArcRel { .. } => "a",
        PathCommandKind::Close => "Z",
    }.to_string()
}

impl ToString for PathData {
    fn to_string(&self) -> String {
        let mut clauses: Vec<String> = Vec::new();
        let mut tokens: Vec<String> = Vec::with_capacity(7);

        for entry in self.cmds.iter() {

            tokens.push(to_cmd_str(entry));

            match entry {
                PathCommandKind::MoveToAbs(point) | 
                    PathCommandKind::MoveToRel(point) | 
                    PathCommandKind::LineAbs(point) | 
                    PathCommandKind::LineRel(point) |
                    PathCommandKind::SmoothQuadraticBezierAbs(point) |
                    PathCommandKind::SmoothQuadraticBezierRel(point) =>
                {
                    tokens.push(point.to_string());
                }
                PathCommandKind::HorizontalAbs(coord) |
                    PathCommandKind::HorizontalRel(coord) |
                    PathCommandKind::VerticalAbs(coord) |
                    PathCommandKind::VerticalRel(coord) =>
                {
                    tokens.push(coord.to_string());
                }
                PathCommandKind::CubicBezierAbs { con_start, con_end, to } => {
                    tokens.push(con_start.to_string());
                    tokens.push(con_end.to_string());
                    tokens.push(to.to_string());
                }
                PathCommandKind::CubicBezierRel { d_con_start, d_con_end, d_to } => {
                    tokens.push(d_con_start.to_string());
                    tokens.push(d_con_end.to_string());
                    tokens.push(d_to.to_string());
                }
                PathCommandKind::SmoothCubicBezierAbs { con_end, to } => {
                    tokens.push(con_end.to_string());
                    tokens.push(to.to_string());
                }
                PathCommandKind::SmoothCubicBezierRel { d_con_end, d_to } => {
                    tokens.push(d_con_end.to_string());
                    tokens.push(d_to.to_string());
                }
                PathCommandKind::QuadraticBezierAbs { con, to } => {
                    tokens.push(con.to_string());
                    tokens.push(to.to_string());
                }
                PathCommandKind::QuadraticBezierRel { d_con, d_to } => {
                    tokens.push(d_con.to_string());
                    tokens.push(d_to.to_string());
                }
                PathCommandKind::EllipticalArcAbs { radius_x, radius_y, angle, laf, sf, shift } => {
                    tokens.push(radius_x.to_string());
                    tokens.push(radius_y.to_string());
                    tokens.push(angle.to_string());
                    tokens.push((*laf as u8).to_string());
                    tokens.push((*sf as u8).to_string());
                    tokens.push(shift.to_string());
                }
                PathCommandKind::EllipticalArcRel { radius_x, radius_y, angle, laf, sf, d_shift } => {
                    tokens.push(radius_x.to_string());
                    tokens.push(radius_y.to_string());
                    tokens.push(angle.to_string());
                    tokens.push((*laf as u8).to_string());
                    tokens.push((*sf as u8).to_string());
                    tokens.push(d_shift.to_string());
                }
                PathCommandKind::Close => {}
            }

            clauses.push(tokens.join(" "));
            tokens.clear();
        }

        clauses.join(" ")
    }
}
