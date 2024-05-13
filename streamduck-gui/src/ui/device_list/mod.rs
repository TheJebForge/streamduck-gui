/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use std::collections::HashSet;
use egui::{Color32, Frame, RichText, ScrollArea, Ui, vec2};
use egui::epaint::Shadow;
use tokio::sync::mpsc::Sender;
use streamduck_rust_client::api::Device;
use streamduck_rust_client::base::{DeviceIdentifier, NamespacedDeviceIdentifier, NamespacedName};
use crate::ui::device_list::device_button::{device_button, DeviceButtonResponse};
use crate::ui::{Pages, UIMessage, UIState};
use crate::ui::util::send_ui_message;

mod device_button;

#[derive(Default)]
pub struct DeviceList {
    pub devices: Vec<Device>,
}

pub fn device_list(ui: &mut Ui, state: &mut UIState, sender: &Sender<UIMessage>) {
    Frame::default()
        .fill(Color32::from_rgb(40, 40, 40))
        .rounding(10.0)
        .inner_margin(10.0)
        .show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading(RichText::new("Select Device")
                    .size(36.0));
            });
        });

    ui.add_space(2.0);

    Frame::default()
        .fill(Color32::from_rgb(40, 40, 40))
        .rounding(10.0)
        .inner_margin(10.0)
        .show(ui, |ui| {
            ui.vertical_centered(|ui| {
                Frame::default()
                    .fill(ui.style().visuals.panel_fill)
                    .rounding(8.0)
                    .inner_margin(10.0)
                    .show(ui, |ui| {
                        ScrollArea::vertical()
                            .show(ui, |ui| {
                                for (index, response) in
                                    state.device_list.devices.iter().enumerate()
                                        .map(|(index, device)| {
                                            (index, device_button(ui, &device.identifier, device.connected, device.autoconnect))
                                        }).collect::<Vec<_>>() {
                                    match response {
                                        DeviceButtonResponse::Open => {
                                            state.open_device(
                                                sender,
                                                state.device_list.devices[index].identifier.clone(),
                                                state.device_list.devices[index].connected
                                            );
                                        }
                                        DeviceButtonResponse::ToggleAutoconnect => {
                                            let device = &mut state.device_list.devices[index];

                                            device.autoconnect = !device.autoconnect;

                                            let message = UIMessage::SetDeviceAutoconnect {
                                                identifier: device.identifier.clone(),
                                                autoconnect: device.autoconnect
                                            };

                                            send_ui_message(sender, message);
                                        }
                                        DeviceButtonResponse::Nothing => {}
                                    }
                                }
                            });
                        ui.allocate_space(ui.available_size());
                    })
            });

            ui.allocate_space(ui.available_size());
        });
}