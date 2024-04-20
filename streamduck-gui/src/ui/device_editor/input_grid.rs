/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use egui::{Color32, Id, Rangef, Sense, Stroke, Ui, vec2};

pub fn input_grid(ui: &mut Ui, inner_ui: impl FnOnce(&mut Ui)) {
    // No spacing
    let old_spacing = ui.spacing().item_spacing;
    ui.spacing_mut().item_spacing.x = 0.0;

    // Sizes
    let id = Id::new("editor_input_grid");

    let panel_min_width = 300_f32;
    let panel_width = ui.data_mut(|t| *t.get_persisted_mut_or(id.with("width"), panel_min_width));

    let all_width = ui.available_width();
    let panel_width = f32::min(panel_width, all_width - panel_min_width);

    let resize_width = 10.0_f32;

    let left_width = all_width - panel_width - (resize_width / 2.0);
    let right_width = all_width - (left_width + resize_width);
    let height = ui.available_height();

    let left_size = vec2(left_width, height);
    let right_size = vec2(right_width, height);

    let (left_id, left_rect) = ui.allocate_space(left_size);

    // Actual input grid

    ui.painter().rect(left_rect, 10.0, Color32::from_rgb(40, 40, 40), Stroke::NONE);

    // Resizer
    let (resize_rect, resize_resp) = ui.allocate_exact_size(vec2(resize_width, ui.available_height()), Sense::drag());
    let resize_style = ui.style().interact(&resize_resp);

    let handle_height = 70_f32;

    let resize_center = resize_rect.center();
    let resize_range = Rangef::new(
        resize_center.y - handle_height / 2.0,
        resize_center.y + handle_height / 2.0
    );
    ui.painter().vline(resize_center.x, resize_range, resize_style.fg_stroke);

    // Resize Logic
    if resize_resp.dragged() {
        if let Some(pointer) = resize_resp.interact_pointer_pos() {
            let width = all_width - (pointer.x - left_rect.min.x);
            let width = width.clamp(panel_min_width, all_width - panel_min_width);
            ui.data_mut(|t| t.insert_persisted(id.with("width"), width));
        }
    }

    // Right UI
    ui.allocate_ui(right_size, |ui| inner_ui(ui));

    // Restore spacing
    ui.spacing_mut().item_spacing = old_spacing;
}