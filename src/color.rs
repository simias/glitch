use std::convert::Into;
use std::ops::{Mul, Add};
use std::cmp::{Eq, PartialEq, Ord, PartialOrd, Ordering};

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Color(pub u8, pub u8, pub u8);

impl Into<HdrColor> for Color {
    fn into(self) -> HdrColor {
        let Color(r, g, b) = self;

        HdrColor((r as f32) / 255.,
                 (g as f32) / 255.,
                 (b as f32) / 255.)
    }
}

impl PartialOrd for Color {
    fn partial_cmp(&self, other: &Color) -> Option<Ordering> {
        let &Color(r1, g1, b1) = self;
        let &Color(r2, g2, b2) = other;

        let r1 = r1 as u16;
        let g1 = g1 as u16;
        let b1 = b1 as u16;

        let r2 = r2 as u16;
        let g2 = g2 as u16;
        let b2 = b2 as u16;

        (r1 + g1 + b1).partial_cmp(&(r2 + g2 + b2))
    }
}

impl Ord for Color {
    fn cmp(&self, other: &Color) -> Ordering {
        let &Color(r1, g1, b1) = self;
        let &Color(r2, g2, b2) = other;

        let r1 = r1 as u16;
        let g1 = g1 as u16;
        let b1 = b1 as u16;

        let r2 = r2 as u16;
        let g2 = g2 as u16;
        let b2 = b2 as u16;

        (r1 + g1 + b1).cmp(&(r2 + g2 + b2))
    }
}

#[derive(Copy, Clone)]
pub struct HdrColor(pub f32, pub f32, pub f32);

impl HdrColor {
    pub fn black() -> HdrColor {
        HdrColor(0., 0., 0.)
    }

    pub fn sqrt(&self) -> HdrColor {
        let &HdrColor(r, g, b) = self;

        HdrColor(r.sqrt(), g.sqrt(), b.sqrt())
    }

    pub fn avg(&self) -> f32 {
        let &HdrColor(r, g, b) = self;

        (r + g + b) / 3.
    }

    pub fn r(&self) -> f32 {
        let &HdrColor(r, _, _) = self;

        r
    }

    pub fn g(&self) -> f32 {
        let &HdrColor(_, g, _) = self;

        g
    }

    pub fn b(&self) -> f32 {
        let &HdrColor(_, _, b) = self;

        b
    }
}

impl Into<Color> for HdrColor {
    fn into(self) -> Color {
        let HdrColor(r, g, b) = self;

        Color((r * 255.).round() as u8,
              (g * 255.).round() as u8,
              (b * 255.).round() as u8)
    }
}

impl Add for HdrColor {
    type Output = HdrColor;

    fn add(self, rhs: HdrColor) -> HdrColor {
        let HdrColor(r1, g1, b1) = self;
        let HdrColor(r2, g2, b2) = rhs;

        HdrColor(r1 + r2, g1 + g2, b1 + b2)
    }
}

impl Mul<f32> for HdrColor {
    type Output = HdrColor;

    fn mul(self, m: f32) -> HdrColor {
        let HdrColor(r, g, b) = self;

        HdrColor(r * m, g * m, b * m)
    }
}

impl Mul<HdrColor> for HdrColor {
    type Output = HdrColor;

    fn mul(self, rhs: HdrColor) -> HdrColor {
        let HdrColor(r1, g1, b1) = self;
        let HdrColor(r2, g2, b2) = rhs;

        HdrColor(r1 * r2, g1 * g2, b1 * b2)
    }
}

impl PartialEq for HdrColor {
    fn eq(&self, other: &HdrColor) -> bool {
        let s: Color = (*self).into();
        let o: Color = (*other).into();

        s.eq(&o)
    }
}

impl Eq for HdrColor {}

impl PartialOrd for HdrColor {
    fn partial_cmp(&self, other: &HdrColor) -> Option<Ordering> {
        let s: Color = (*self).into();
        let o: Color = (*other).into();

        s.partial_cmp(&o)
    }
}

impl Ord for HdrColor {
    fn cmp(&self, other: &HdrColor) -> Ordering {
        let s: Color = (*self).into();
        let o: Color = (*other).into();

        s.cmp(&o)
    }
}
