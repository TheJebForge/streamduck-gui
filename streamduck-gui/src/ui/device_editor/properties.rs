/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use std::fmt::format;
use egui::{Color32, Frame, Margin, pos2, Rect, ScrollArea, Ui, vec2};
use tokio::sync::mpsc::Sender;
use crate::ui::{UIMessage, UIState};
use crate::ui::device_editor::tabs::tabs;

pub fn properties_ui(ui: &mut Ui, state: &mut UIState, sender: &Sender<UIMessage>) {
    ui.vertical(|ui| {
        let tab_text = [
            "Properties".to_string(),
            "Templates".to_string()
        ];

        tabs(ui, &tab_text, &tab_text[0], 10.0, state.device_editor.grid_collapsed);

        let next_widget = ui.next_widget_position();
        let available_size = ui.available_size();

        let margin_rect = Rect::from_min_size(
            pos2(next_widget.x + 10.0, next_widget.y + 10.0),
            vec2(available_size.x - 20.0, available_size.y - 20.0)
        );

        ui.allocate_ui_at_rect(margin_rect, |ui| {
            ui.vertical(|ui| {
                ScrollArea::vertical()
                    .show(ui, |ui| {
                        for i in 0..200 {
                            if ui.button("aa").clicked() {
                                state.device_editor.grid_collapsed = !state.device_editor.grid_collapsed;
                            }
                        }


                        ui.allocate_space(ui.available_size());
                    })
            })
        })
    });


    ui.allocate_space(ui.available_size());
}