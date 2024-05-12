/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use eframe::emath::pos2;
use egui::{Color32, FontFamily, FontId, Id, Rect, Rounding, Sense, Stroke, Ui, vec2};
use interpolation::Ease;
use crate::ui::util::lerp_color;

pub enum TabResponse<'a> {
    TabClicked(&'a str),
    Nothing
}

pub fn tabs<'a>(ui: &mut Ui, tabs: &'a [String], current_tab: &'a String, bottom_margin: f32, collapsed: bool) -> TabResponse<'a> {
    let available_space = ui.available_size();

    let animated_collapse = ui.ctx().animate_value_with_time(
        Id::new(tabs).with("collapse"),
        if collapsed { 0.0 } else { 1.0 },
        0.2
    ).cubic_in_out();

    let tab_height = 40f32 * animated_collapse;
    let element_width = available_space.x;
    let element_height = tab_height + bottom_margin;

    let (id, rect) = ui.allocate_space(vec2(element_width, element_height));





    ui.painter().rect(rect, 0.0, ui.style().visuals.panel_fill, Stroke::NONE);

    let mut current_offset = 0.0f32;
    for tab in tabs {
        let galley = ui.painter().layout(
            tab.clone(),
            FontId::new(20.0 * animated_collapse, FontFamily::Proportional),
            Color32::PLACEHOLDER,
            10000.0
        );

        let horizontal_margin = 15.0f32;
        let width = galley.rect.width() + (horizontal_margin * 2.0);

        let size = vec2(width, tab_height);
        let tab_pos = pos2(
            rect.min.x + current_offset,
            rect.min.y
        );
        let tab_rect = Rect::from_min_size(tab_pos, size);

        current_offset += width;

        let response = ui.allocate_rect(tab_rect, Sense::click());
        let tab_style = ui.style().interact(&response);

        let tab_color = if tab == current_tab {
            Color32::from_rgb(40, 40, 40)
        } else {
            Color32::from_rgb(30, 30, 30)
        };

        ui.painter().rect(
            tab_rect,
            Rounding {
                nw: 10.0,
                ne: 10.0,
                sw: 0.0,
                se: 0.0,
            },
            tab_color,
            Stroke::NONE
        );

        ui.painter().rect(
            tab_rect,
            10.0,
            if response.hovered() {
                tab_style.bg_fill
            } else {
                tab_color
            },
            Stroke::NONE
        );

        let text_pos = pos2(
            tab_rect.min.x + horizontal_margin,
            tab_rect.min.y + (tab_rect.height() / 2.0 - galley.rect.height() / 2.0)
        );

        ui.painter().galley(text_pos, galley, lerp_color(&Color32::TRANSPARENT, &tab_style.fg_stroke.color, animated_collapse));
    }

    // Bottom adapter
    let bottom_rect = Rect::from_min_size(
        pos2(rect.min.x, rect.max.y - bottom_margin),
        vec2(rect.width(), bottom_margin)
    );
    ui.painter().rect(
        bottom_rect,
        Rounding {
            nw: 10.0 * (1.0 - animated_collapse),
            ne: 10.0,
            sw: 0.0,
            se: 0.0,
        },
        Color32::from_rgb(40, 40, 40),
        Stroke::NONE
    );

    TabResponse::Nothing
}

