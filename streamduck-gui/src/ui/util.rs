/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use std::ops::{Add, Mul, Sub};
use egui::{Color32, Id, Response, Ui};
use egui::style::WidgetVisuals;
use interpolation::{Ease, Lerp};

#[derive(Copy, Clone, Debug)]
pub struct Colorf32 {
    r: f32,
    g: f32,
    b: f32,
    a: f32
}

impl From<Color32> for Colorf32 {
    fn from(value: Color32) -> Self {
        Self {
            r: value.r() as f32 / 255.0,
            g: value.g() as f32 / 255.0,
            b: value.b() as f32 / 255.0,
            a: value.a() as f32 / 255.0,
        }
    }
}

impl From<Colorf32> for Color32 {
    fn from(value: Colorf32) -> Self {
        Color32::from_rgba_premultiplied(
            (value.r.min(1.0).max(0.0) * 255.0) as u8,
            (value.g.min(1.0).max(0.0) * 255.0) as u8,
            (value.b.min(1.0).max(0.0) * 255.0) as u8,
            (value.a.min(1.0).max(0.0) * 255.0) as u8
        )
    }
}

impl Mul<f32> for Colorf32 {
    type Output = Colorf32;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
            a: self.a * rhs,
        }
    }
}

impl Mul<Colorf32> for Colorf32 {
    type Output = Colorf32;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
            a: self.a * rhs.a,
        }
    }
}

impl Add<Colorf32> for Colorf32 {
    type Output = Colorf32;

    fn add(self, rhs: Colorf32) -> Self::Output {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
            a: self.a + rhs.a,
        }
    }
}

impl Sub<Colorf32> for Colorf32 {
    type Output = Colorf32;

    fn sub(self, rhs: Colorf32) -> Self::Output {
        Self {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
            a: self.a - rhs.a,
        }
    }
}

pub fn lerp_color(a: &Color32, b: &Color32, t: f32) -> Color32 {
    let a = Colorf32::from(*a);
    let b = Colorf32::from(*b);
    let t = t.min(1.0).max(0.0);

    (a + (b - a) * t).into()
}

pub fn lerp_f32(a: f32, b: f32, t: f32) -> f32 {
    let t = t.min(1.0).max(0.0);

    a + (b - a) * t
}

pub fn interact_lerped_selectable(ui: &Ui, response: &Response, selected: bool, id: Id, animation_time: f32) -> WidgetVisuals {
    let t = ui.ctx().animate_bool_with_time(id, selected, animation_time).cubic_in_out();

    let selected = &ui.style().visuals.selection;
    let mut visuals = *ui.style().interact(response);
    visuals.weak_bg_fill = lerp_color(&visuals.weak_bg_fill, &selected.bg_fill, t);
    visuals.bg_fill = lerp_color(&visuals.bg_fill, &selected.bg_fill, t);
    visuals.fg_stroke.color = lerp_color(&visuals.fg_stroke.color, &selected.stroke.color, t);
    visuals.fg_stroke.width = f32::lerp(&visuals.fg_stroke.width, &selected.stroke.width, &t);

    visuals
}