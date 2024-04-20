/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use egui::{Color32, FontFamily, FontId, Id, pos2, Rect, Sense, Stroke, Ui, vec2};
use egui::epaint::TextShape;
use streamduck_rust_client::base::NamespacedDeviceIdentifier;
use interpolation::{Lerp, Ease};
use crate::ui::util::{interact_lerped_selectable, lerp_color, lerped_selectable};

pub fn mini_device_button(ui: &mut Ui, device: &NamespacedDeviceIdentifier, connected: bool, how_much_to_leave: f32) {
    // Device name galleys
    let id = Id::new(device).with("mini");
    let main_button_style = lerped_selectable(
        ui,
        connected,
        id.with("main_connected"),
        0.3
    );

    let top_text_galley = ui.painter().layout(
        device.identifier.to_string(),
        FontId::new(16.0, FontFamily::Proportional),
        main_button_style.fg_stroke.color,
        1000.0
    );

    let bottom_text_galley = ui.painter().layout(
        device.name.to_string(),
        FontId::new(11.0, FontFamily::Proportional),
        main_button_style.fg_stroke.color,
        1000.0
    );

    // Element allocation
    let element_width = ui.available_width() - how_much_to_leave;
    let element_height = ui.available_height();

    let rounding = 8.0_f32;

    let (_, element_rect) = ui.allocate_space(vec2(element_width, element_height));

    ui.painter().rect(element_rect, rounding, main_button_style.bg_fill, main_button_style.bg_stroke);

    // Connected tip
    let animated_connected_id = id.with("connected_tip");
    let animated_connected = ui.ctx().animate_value_with_time(
        animated_connected_id,
        if connected { 1.0 } else { 0.0 },
        0.3
    );

    let connected_color = lerp_color(
        &Color32::TRANSPARENT,
        &Color32::BLACK,
        animated_connected.cubic_in_out()
    );

    let connected_galley = ui.painter().layout(
        "CONNECTED".to_string(),
        FontId::new(16.0, FontFamily::Monospace),
        connected_color,
        1000.0
    );

    let connected_text_gap = (element_rect.height() / 2.0 - connected_galley.rect.height() / 2.0);

    ui.painter().galley(
        pos2(
            element_rect.max.x - connected_text_gap - connected_galley.rect.width(),
            element_rect.min.y + connected_text_gap
        ),
        connected_galley,
        Color32::WHITE
    );

    // Disconnected tip
    let disconnected_color = lerp_color(
        &Color32::TRANSPARENT,
        &Color32::WHITE,
        1.0 - animated_connected.cubic_in_out()
    );

    let disconnected_galley = ui.painter().layout(
        "DISCONNECTED".to_string(),
        FontId::new(16.0, FontFamily::Monospace),
        disconnected_color,
        1000.0
    );

    ui.painter().galley(
        pos2(
            element_rect.max.x - connected_text_gap - disconnected_galley.rect.width(),
            element_rect.min.y + connected_text_gap
        ),
        disconnected_galley,
        Color32::WHITE
    );

    // Device name
    let text_outer_margin = 10.0_f32;
    let text_margin = -3.0_f32;

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
}