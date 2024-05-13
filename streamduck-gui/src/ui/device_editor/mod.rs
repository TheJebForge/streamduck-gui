/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

pub mod mini_device;
pub mod input_grid;
mod properties;
mod tabs;
mod stack;

use egui::{Align, Button, Color32, Frame, Layout, Margin, RichText, Rounding, ScrollArea, SidePanel, TopBottomPanel, Ui, vec2};
use tokio::sync::mpsc::Sender;
use streamduck_rust_client::api::Input;
use streamduck_rust_client::base::NamespacedDeviceIdentifier;
use crate::ui::{Pages, UIMessage, UIState};
use crate::ui::device_editor::input_grid::{Grid, input_grid};
use crate::ui::device_editor::mini_device::mini_device_button;
use crate::ui::device_editor::properties::properties_ui;

#[derive(Default)]
pub struct DeviceEditor {
    pub device: NamespacedDeviceIdentifier,
    pub connected: bool,
    pub grid: Option<Grid>,
    pub stack: Vec<String>,
    pub waiting_for_grid: bool,
    pub grid_collapsed: bool
}

pub fn device_editor(ui: &mut Ui, state: &mut UIState, sender: &Sender<UIMessage>) {
    TopBottomPanel::top("editor-top")
        .frame(Frame::default()
            .inner_margin(10.0)
            .rounding(10.0)
            .fill(Color32::from_rgb(40, 40, 40)))
        .show_separator_line(false)
        .show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                let button_width = 60_f32;
                let item_spacing = ui.style().spacing.item_spacing.x;

                if ui.add(Button::new(RichText::new("\u{f17a7}").size(30.0).line_height(Some(32.0)))
                    .min_size(vec2(button_width, 50.0))
                    .rounding(8.0)).clicked() {
                    state.current_page = Pages::DeviceList;
                }

                mini_device_button(
                    ui,
                    &state.device_editor.device,
                    state.device_editor.connected,
                    button_width + item_spacing
                );

                if ui.add(Button::new(RichText::new("\u{f0493}").size(30.0).line_height(Some(32.0)))
                    .min_size(vec2(button_width, 50.0))
                    .rounding(8.0)).clicked() {

                }
            });
        });

    ui.add_space(5.0);

    ui.allocate_ui_with_layout(ui.available_size(), Layout::left_to_right(Align::Center), |ui| {
        input_grid(ui, state, sender, state.device_editor.grid_collapsed, |ui, state| {
            Frame::default()
                .rounding(Rounding {
                    nw: 0.0,
                    ne: 0.0,
                    sw: 10.0,
                    se: 10.0
                })
                .inner_margin(0.0)
                .fill(Color32::from_rgb(40, 40, 40))
                .show(ui, |ui| {
                    properties_ui(ui, state, sender);
                });
        });
    });
}