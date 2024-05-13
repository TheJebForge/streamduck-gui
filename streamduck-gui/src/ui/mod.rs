/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

mod device_list;
mod util;
mod device_editor;

use std::sync::{Arc, Condvar};
use std::thread;
use eframe::{App, CreationContext, NativeOptions, run_native};
use egui::{CentralPanel, Context, FontData, FontDefinitions, FontFamily, FontId, RichText, TextStyle, Frame, Color32, vec2};
use egui::style::ScrollStyle;
use tokio::sync::mpsc::{Receiver, Sender};
use streamduck_rust_client::base::NamespacedDeviceIdentifier;
use crate::APIMessage;
use crate::ui::device_editor::{device_editor, DeviceEditor};
use crate::ui::device_editor::input_grid::Grid;
use crate::ui::device_list::{device_list, DeviceList};
use crate::ui::util::send_ui_message;

pub fn ui_main(tx: Sender<UIMessage>, rx: Receiver<APIMessage>, notify: Receiver<()>) {
    let mut native_options = NativeOptions::default();
    native_options.viewport.min_inner_size = Some(vec2(800.0, 600.0));
    run_native("Streamduck GUI", native_options,
               Box::new(move |cc| Box::new(UIApp::new(cc, tx, rx, notify))))
        .unwrap();
}

pub enum UIMessage {
    SetDeviceAutoconnect {
        identifier: NamespacedDeviceIdentifier,
        autoconnect: bool
    },
    ConnectDevice(NamespacedDeviceIdentifier),
    GetDeviceState(NamespacedDeviceIdentifier),
    PopScreen(NamespacedDeviceIdentifier)
}

struct UIApp {
    tx: Sender<UIMessage>,
    rx: Receiver<APIMessage>,
    state: UIState
}

impl UIApp {
    fn new(cc: &CreationContext<'_>, tx: Sender<UIMessage>, rx: Receiver<APIMessage>, mut notify: Receiver<()>) -> Self {
        // Fonts
        let mut fonts = FontDefinitions::default();

        let mut font_data = FontData::from_static(
            include_bytes!("../../fonts/opensans.ttf")
        );
        font_data.tweak.y_offset_factor = 0.0;

        fonts.font_data.insert(
            "opensans".to_string(),
            font_data
        );

        let mut font_data = FontData::from_static(
            include_bytes!("../../fonts/cousine-nerd-propo.ttf")
        );
        font_data.tweak.y_offset_factor = 0.0;

        fonts.font_data.insert(
            "cousine-propo".to_string(),
            font_data
        );

        let mut font_data = FontData::from_static(
            include_bytes!("../../fonts/roboto-nerd-mono.ttf")
        );
        font_data.tweak.y_offset_factor = 0.0;

        fonts.font_data.insert(
            "roboto-mono".to_string(),
            font_data
        );

        let mut proportional = fonts.families.get_mut(&FontFamily::Proportional)
            .unwrap();

        proportional.insert(0, "cousine-propo".to_string());
        proportional.insert(0, "opensans".to_string());

        fonts.families.get_mut(&FontFamily::Monospace)
            .unwrap()
            .insert(0, "roboto-mono".to_string());

        cc.egui_ctx.set_fonts(fonts);

        // Style
        cc.egui_ctx.style_mut(|style| {
            style.text_styles = [
                (TextStyle::Small, FontId::new(9.0, FontFamily::Proportional)),
                (TextStyle::Body, FontId::new(13.0, FontFamily::Proportional)),
                (TextStyle::Button, FontId::new(13.0, FontFamily::Proportional)),
                (TextStyle::Heading, FontId::new(20.0, FontFamily::Proportional)),
                (TextStyle::Monospace, FontId::new(13.0, FontFamily::Monospace)),
            ].into();

            style.visuals.selection.bg_fill = Color32::from_rgb(98, 163, 136);
            style.visuals.selection.stroke.color = Color32::from_rgb(0, 0, 0);

            style.visuals.window_fill = Color32::from_rgb(20, 20, 20);
            style.visuals.panel_fill = Color32::from_rgb(20, 20, 20);

            style.visuals.hyperlink_color = Color32::from_rgb(98, 163, 136);
            style.interaction.selectable_labels = false;

            style.visuals.widgets.hovered.bg_stroke.color = Color32::from_rgb(90, 90, 90);
            style.visuals.widgets.hovered.bg_stroke.width = 3.0;

            style.visuals.widgets.active.bg_stroke.color = Color32::from_rgb(200, 200, 200);
            style.visuals.widgets.active.bg_stroke.width = 3.0;

            style.spacing.scroll = ScrollStyle::thin();
        });

        let context_copy = cc.egui_ctx.clone();
        thread::spawn(move || {
            while let Some(()) = notify.blocking_recv() {
                context_copy.request_repaint();
            }
        });

        Self {
            tx,
            rx,
            state: UIState {
                device_list: Default::default(),
                device_editor: Default::default(),
                current_page: Pages::DeviceList,
            }
        }
    }
}

pub struct UIState {
    pub device_list: DeviceList,
    pub device_editor: DeviceEditor,
    pub current_page: Pages
}

impl UIState {
    pub fn open_device(&mut self, sender: &Sender<UIMessage>, identifier: NamespacedDeviceIdentifier, connected: bool) {
        if !connected {
            send_ui_message(sender, UIMessage::ConnectDevice(identifier));
        } else {
            self.current_page = Pages::DeviceEditor;
            self.device_editor.device = identifier;
            self.device_editor.connected = connected;
            self.device_editor.waiting_for_grid = false;
            self.device_editor.grid = None;
        }
    }
}

pub enum Pages {
    DeviceList,
    DeviceEditor
}

impl App for UIApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        if let Ok(message) = self.rx.try_recv() {
            match message {
                APIMessage::DeviceList(devices) => {
                    self.state.device_list.devices = devices;
                }

                APIMessage::NewDevice(device) => {
                    self.state.device_list.devices.push(device);
                }
                APIMessage::DeviceGone(device) => {
                    self.state.device_list.devices.retain(|d| d.identifier != device)
                }
                APIMessage::ConnectedDevice(device) => {
                    self.state.device_list.devices.iter_mut()
                        .filter(|d| d.identifier == device.identifier)
                        .for_each(|d| d.connected = true);

                    if self.state.device_editor.device == device.identifier {
                        self.state.device_editor.connected = true;
                    }
                }
                APIMessage::DisconnectedDevice(device) => {
                    self.state.device_list.devices.retain(|d| d.identifier != device);

                    if self.state.device_editor.device == device {
                        self.state.device_editor.connected = false;
                    }
                }
                APIMessage::InputGrid(grid) => {
                    self.state.device_editor.waiting_for_grid = false;
                    self.state.device_editor.grid = Some(Grid::from_inputs(grid));
                }
                
                APIMessage::Stack(stack) => {
                    self.state.device_editor.stack = stack;
                }
                
                APIMessage::ScreenItems(_) => {}
            }
        }

        CentralPanel::default()
            .show(ctx, |ui| {
                match &self.state.current_page {
                    Pages::DeviceList => device_list(ui, &mut self.state, &self.tx),
                    Pages::DeviceEditor => device_editor(ui, &mut self.state, &self.tx)
                }
            });
    }
}