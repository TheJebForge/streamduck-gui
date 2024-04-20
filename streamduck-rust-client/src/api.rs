/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use serde::{Deserialize, Serialize};
use crate::base::{DeviceIdentifier, NamespacedDeviceIdentifier, NamespacedName};

pub trait StreamduckRequest {
    fn name(&self) -> NamespacedName;
}

#[derive(Serialize)]
pub struct CoreVersion;

impl StreamduckRequest for CoreVersion {
    fn name(&self) -> NamespacedName {
        NamespacedName::new("Core", "Socket Version")
    }
}

#[derive(Serialize)]
pub struct ListDevices;

impl StreamduckRequest for ListDevices {
    fn name(&self) -> NamespacedName {
        NamespacedName::new("Core", "List Devices")
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Device {
    pub identifier: NamespacedDeviceIdentifier,
    pub connected: bool,
    pub autoconnect: bool
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SetDeviceAutoconnect {
    pub identifier: NamespacedDeviceIdentifier,
    pub autoconnect: bool
}

impl StreamduckRequest for SetDeviceAutoconnect {
    fn name(&self) -> NamespacedName {
        NamespacedName::new("Core", "Set Device Autoconnect")
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Input {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
    pub icon: InputIcon
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq)]
pub enum InputIcon {
    Button,
    Toggle,
    AnalogButton,
    Slider,
    Knob,
    Encoder,
    TouchScreen,
    Joystick,
    Trackball,
    Touchpad,
    Sensor
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetDeviceInputs {
    pub identifier: NamespacedDeviceIdentifier
}

impl StreamduckRequest for GetDeviceInputs {
    fn name(&self) -> NamespacedName {
        NamespacedName::new("Core", "Get Device Inputs")
    }
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConnectDevice {
    pub identifier: NamespacedDeviceIdentifier
}

impl StreamduckRequest for ConnectDevice {
    fn name(&self) -> NamespacedName {
        NamespacedName::new("Core", "Connect Device")
    }
}