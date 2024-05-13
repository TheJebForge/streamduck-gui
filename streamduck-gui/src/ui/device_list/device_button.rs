/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use egui::{Color32, FontFamily, FontId, Id, pos2, Rect, Sense, Stroke, Ui, vec2};
use egui::epaint::TextShape;
use streamduck_rust_client::base::NamespacedDeviceIdentifier;
use interpolation::{Lerp, Ease};
use crate::ui::util::{interact_lerped_selectable, lerp_color};

pub fn device_button(ui: &mut Ui, device: &NamespacedDeviceIdentifier, connected: bool, autoconnect: bool) -> DeviceButtonResponse {
    let max_rect = ui.max_rect();

    let animation_time = 0.2f32;
    
    let element_width = max_rect.width();
    let element_height = 80.0_f32;

    let rounding = 10.0_f32;
    let margin = 25.0_f32;

    let id = Id::new(device);
    let (_, element_rect) = ui.allocate_space(vec2(element_width, element_height));

    let main_button_rect = Rect::from_min_max(
        element_rect.min,
        pos2(element_rect.max.x - element_height - margin, element_rect.max.y)
    );
    let checkbox_rect = Rect::from_min_max(
        pos2(main_button_rect.max.x + margin, main_button_rect.min.y),
        pos2(main_button_rect.max.x + margin + element_height, main_button_rect.max.y)
    );

    let main_button_response = ui.allocate_rect(main_button_rect, Sense::click());
    let checkbox_response = ui.allocate_rect(checkbox_rect, Sense::click());

    // Don't draw if not visible
    if !ui.is_rect_visible(element_rect) {
        return DeviceButtonResponse::Nothing;
    }

    let main_button_style = interact_lerped_selectable(
        ui,
        &main_button_response,
        connected,
        id.with("main_connected"),
        animation_time
    );
    let checkbox_style = interact_lerped_selectable(
        ui,
        &checkbox_response,
        connected,
        id.with("checkbox_connected"),
        animation_time
    );

    ui.painter().rect(main_button_rect, rounding, main_button_style.bg_fill, main_button_style.bg_stroke);
    ui.painter().rect(checkbox_rect, rounding, checkbox_style.bg_fill, checkbox_style.bg_stroke);

    // Checkbox
    let checkbox_bg_rect = checkbox_rect.shrink(8.0);
    ui.painter().rect(checkbox_bg_rect, rounding, ui.style().visuals.panel_fill, Stroke::NONE);

    let animated_bool_id = id.with("animated_bool");
    let lerped_autoconnect = ui.ctx()
        .animate_value_with_time(animated_bool_id, if autoconnect { 1.0 } else { 0.0 }, 0.15)
        .cubic_in_out();

    let checkbox_fg_rect = checkbox_bg_rect.shrink(
        f32::lerp(&(checkbox_bg_rect.width() / 2.0), &3.0, &lerped_autoconnect)
    );
    let selection = ui.style().visuals.selection.bg_fill;

    ui.painter().rect(
        checkbox_fg_rect,
        f32::lerp(&10.0, &(rounding - 2.0), &lerped_autoconnect),
        selection,
        Stroke::NONE);

    // Connected tip
    let animated_connected_id = id.with("connected_tip");
    let animated_connected = ui.ctx().animate_value_with_time(
        animated_connected_id,
        if connected { 1.0 } else { 0.0 },
        animation_time
    );

    let connected_color = lerp_color(
        &Color32::TRANSPARENT,
        &Color32::BLACK,
        animated_connected.cubic_in_out()
    );

    let connected_galley = ui.painter().layout(
        "CONNECTED".to_string(),
        FontId::new(20.0, FontFamily::Monospace),
        connected_color,
        1000.0
    );

    let connected_text_gap = (main_button_rect.height() / 2.0 - connected_galley.rect.height() / 2.0);

    ui.painter().galley(
        pos2(
            main_button_rect.max.x - connected_text_gap - connected_galley.rect.width(),
            main_button_rect.min.y + connected_text_gap
        ),
        connected_galley,
        Color32::WHITE
    );

    // Device name
    let top_text_galley = ui.painter().layout(
        device.identifier.to_string(),
        FontId::new(20.0, FontFamily::Proportional),
        main_button_style.fg_stroke.color,
        main_button_rect.width()
    );

    let bottom_text_galley = ui.painter().layout(
        device.name.to_string(),
        FontId::new(14.0, FontFamily::Proportional),
        main_button_style.fg_stroke.color,
        main_button_rect.width()
    );

    let text_outer_margin = 10.0_f32;
    let text_margin = -1.0_f32;

    let total_text_height = top_text_galley.rect.height() + text_margin + bottom_text_galley.rect.height();
    let start_text_pos = pos2(
        element_rect.min.x + text_outer_margin,
        element_rect.min.y + (element_rect.height() / 2.0 - total_text_height / 2.0)
    );
    let second_text_pos = start_text_pos
        + vec2(0.0, top_text_galley.rect.height() + text_margin);

    ui.painter().galley(
        start_text_pos,
        top_text_galley,
        Color32::WHITE
    );
    ui.painter().galley(
        second_text_pos,
        bottom_text_galley,
        Color32::WHITE
    );

    // Autoconnect tip
    let autoconnect_tip_galley = ui.painter().layout(
        "AUTOCONNECT".to_string(),
        FontId::new(10.5, FontFamily::Monospace),
        ui.style().visuals.widgets.inactive.fg_stroke.color,
        1000.0
    );

    let autoconnect_tip_pos = pos2(
        checkbox_rect.min.x - autoconnect_tip_galley.rect.height() - 2.0,
        checkbox_rect.min.y
            + (checkbox_rect.height() / 2.0 - autoconnect_tip_galley.rect.width() / 2.0)
            + autoconnect_tip_galley.rect.width()
    );

    ui.painter().add(
        TextShape::new(
            autoconnect_tip_pos,
            autoconnect_tip_galley,
            Color32::WHITE
        ).with_angle(-std::f32::consts::TAU * 0.25)
    );

    // Response
    if main_button_response.clicked() {
        return DeviceButtonResponse::Open;
    }

    if checkbox_response.clicked() {
        return DeviceButtonResponse::ToggleAutoconnect;
    }

    DeviceButtonResponse::Nothing
}

pub enum DeviceButtonResponse {
    Open,
    ToggleAutoconnect,
    Nothing
}