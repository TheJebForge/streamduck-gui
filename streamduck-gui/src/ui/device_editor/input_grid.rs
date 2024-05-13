/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use egui::{Color32, Id, pos2, Rangef, Rect, Sense, Spinner, Stroke, Ui, vec2};
use interpolation::Ease;

use tokio::sync::mpsc::Sender;
use streamduck_rust_client::api::{Input, InputIcon};
use crate::ui::{UIMessage, UIState};
use crate::ui::device_editor::stack::stack_line;
use crate::ui::util::send_ui_message;

#[derive(Default, Debug)]
pub struct Grid {
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,

    pub width: f32,
    pub height: f32,
    pub width_to_height_ratio: f32,

    pub items: Vec<GridItem>
}

#[derive(Debug)]
pub struct GridItem {
    pub looks: Input,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32
}

impl Grid {
    pub fn from_inputs(inputs: Vec<Input>) -> Self {
        let mut grid = Self::default();

        // Calculate bounds
        for input in inputs.into_iter() {
            let min_x = input.x as f32;
            let min_y = input.y as f32;

            let width = input.w as f32;
            let height = input.h as f32;

            let max_x = min_x + width - 1.0;
            let max_y = min_y + height - 1.0;

            grid.min_x = f32::min(grid.min_x, min_x);
            grid.min_y = f32::min(grid.min_y, min_y);
            grid.max_x = f32::max(grid.max_x, max_x);
            grid.max_y = f32::max(grid.max_y, max_y);

            grid.items.push(GridItem {
                looks: input,
                x: min_x,
                y: min_y,
                w: width,
                h: height,
            });
        }

        grid.width = grid.max_x - grid.min_x + 1.0;
        grid.height = grid.max_y - grid.min_y + 1.0;
        grid.width_to_height_ratio = grid.width / grid.height;

        // Adjust item positions
        for item in grid.items.iter_mut() {
            item.x -= grid.min_x;
            item.y -= grid.min_y;
        }

        grid
    }
}

pub fn input_grid(ui: &mut Ui, state: &mut UIState, sender: &Sender<UIMessage>, collapsed: bool, inner_ui: impl FnOnce(&mut Ui, &mut UIState)) {
    // No spacing
    let old_spacing = ui.spacing().item_spacing;
    ui.spacing_mut().item_spacing.x = 0.0;

    let id = Id::new("editor_input_grid");

    // Collapse animation
    let animated_collapse = ui.ctx().animate_value_with_time(
        id.with("collapse"),
        if collapsed { 0.0 } else { 1.0 },
        0.2f32
    ).cubic_in_out();

    // Sizes
    let draw_threshold = 2f32;
    
    let panel_min_width = 300_f32;
    let panel_width = ui.data_mut(|t| *t.get_persisted_mut_or(id.with("width"), panel_min_width));

    let all_width = ui.available_width();
    let panel_width = f32::min(panel_width, all_width - panel_min_width);

    let resize_width = 10.0_f32 * animated_collapse;

    let left_width = all_width - panel_width - (resize_width / 2.0);
    let theoretical_left_width = left_width;
    
    let left_width = left_width * animated_collapse;

    let input_grid_fully_visible = left_width > (theoretical_left_width - 5.0);

    let right_width = all_width - (left_width + resize_width);
    let height = ui.available_height();
    
    let stack_height = 45f32;
    
    let left_size = vec2(left_width, height);
    let right_size = vec2(right_width, height);

    let (_left_id, left_rect) = ui.allocate_space(left_size);
    
    let left_panel_top_right = left_rect.right_top();
    let right_rect = Rect::from_min_size(
        pos2(left_panel_top_right.x + resize_width, left_panel_top_right.y),
        right_size
    );
    
    let stack_rect = Rect::from_min_size(
        left_rect.min,
        vec2(theoretical_left_width, stack_height)
    );
    
    let left_rect = left_rect.with_min_y(stack_rect.max.y);

    // Margin
    let input_margin = 10f32;
    let left_rect = left_rect.shrink(input_margin);

    // Actual input grid and stack
    if left_width > draw_threshold {
        if let Some(grid) = &state.device_editor.grid {
            let aspected_height = theoretical_left_width / grid.width_to_height_ratio;
            let expected_height = f32::min(aspected_height, left_rect.height());

            let scale = expected_height / grid.height;
            let expected_width = grid.width * scale;

            let center = pos2(
                left_rect.left() + theoretical_left_width / 2.0 - input_margin,
                left_rect.top() + left_rect.height() / 2.0
            );

            let grid_rect = Rect::from_center_size(center, vec2(expected_width, expected_height));

            for item in &grid.items {
                let id = Id::new(&item.looks);

                let min = pos2(
                    grid_rect.min.x + (item.x * scale),
                    grid_rect.min.y + (item.y * scale)
                );

                let max = pos2(
                    min.x + item.w * scale,
                    min.y + item.h * scale
                );

                let gap = 0.02 * scale;

                let item_rect = Rect::from_min_max(min, max)
                    .shrink(gap);

                let rounding: f32 = match &item.looks.icon {
                    InputIcon::Button | InputIcon::Toggle | InputIcon::AnalogButton
                    | InputIcon::Slider | InputIcon::TouchScreen | InputIcon::Touchpad => 0.1,
                    InputIcon::Knob | InputIcon::Encoder | InputIcon::Joystick
                    | InputIcon::Trackball | InputIcon::Sensor => 10000.0,
                } * scale;

                let response = ui.interact(item_rect, id, Sense::click());
                let style = ui.style().interact(&response);

                ui.painter().rect(item_rect, rounding, style.bg_fill, style.bg_stroke);
            }
        } else {
            let spinner = Spinner::new();
            spinner.paint_at(ui, Rect::from_center_size(left_rect.center(), vec2(75.0, 75.0)));

            if !state.device_editor.waiting_for_grid {
                state.device_editor.waiting_for_grid = true;
                send_ui_message(sender, UIMessage::GetDeviceState(state.device_editor.device.clone()));
            }
        }
    }

    // Resizer
    let (resize_rect, resize_resp) = ui.allocate_exact_size(
        vec2(resize_width, ui.available_height()),
        if input_grid_fully_visible { Sense::drag() } else { Sense::focusable_noninteractive() }
    );
    let resize_style = ui.style().interact(&resize_resp);

    let handle_height = 70_f32 * animated_collapse;

    let resize_center = resize_rect.center();
    let resize_range = Rangef::new(
        resize_center.y - handle_height / 2.0,
        resize_center.y + handle_height / 2.0
    );
    ui.painter().vline(resize_center.x, resize_range, resize_style.fg_stroke);

    // Resize Logic
    if resize_resp.dragged() && input_grid_fully_visible {
        if let Some(pointer) = resize_resp.interact_pointer_pos() {
            let width = all_width - (pointer.x - left_rect.min.x);
            let width = width.clamp(panel_min_width, all_width - panel_min_width);
            ui.data_mut(|t| t.insert_persisted(id.with("width"), width));
        }
    }

    // Stack UI
    if left_width > draw_threshold {
        ui.allocate_ui_at_rect(stack_rect, |ui| stack_line(ui, state, sender));
    }
    
    // Right UI
    ui.allocate_ui_at_rect(right_rect, |ui| inner_ui(ui, state));

    // Restore spacing
    ui.spacing_mut().item_spacing = old_spacing;
}