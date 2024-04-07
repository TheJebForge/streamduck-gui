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
use crate::ui::{UIMessage, UIState};

mod device_button;

#[derive(Default)]
pub struct DeviceList {
    pub devices: Vec<Device>,
}

pub fn device_list(ui: &mut Ui, state: &mut UIState, sender: &Sender<UIMessage>) {
    Frame::default()
        .fill(Color32::from_rgb(70, 102, 90))
        .rounding(20.0)
        .inner_margin(15.0)
        .show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading(RichText::new("Select Device")
                    .size(35.0)
                    .color(Color32::BLACK));
                ui.add_space(8.0);

                Frame::default()
                    .fill(ui.style().visuals.panel_fill)
                    .rounding(10.0)
                    .inner_margin(10.0)
                    .shadow(Shadow {
                        offset: vec2(2.0, 2.0),
                        blur: 10.0,
                        spread: 0.0,
                        color: Color32::from_rgba_premultiplied(0, 0, 0, 100),
                    })
                    .show(ui, |ui| {
                        ScrollArea::vertical()
                            .show(ui, |ui| {
                                for mut device in &mut state.device_list.devices {
                                    match device_button(ui, &device.identifier, device.connected, device.autoconnect) {
                                        DeviceButtonResponse::Open => {}
                                        DeviceButtonResponse::ToggleAutoconnect => {
                                            device.autoconnect = !device.autoconnect;

                                            let message = UIMessage::SetDeviceAutoconnect {
                                                identifier: device.identifier.clone(),
                                                autoconnect: device.autoconnect
                                            };

                                            let sender = sender.clone();

                                            tokio::spawn(async move {
                                                sender.send(message).await.ok()
                                            });
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