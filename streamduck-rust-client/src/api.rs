/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use serde::{Deserialize, Serialize};
use crate::base::{DeviceIdentifier, NamespacedDeviceIdentifier, NamespacedName};

pub trait StreamduckRequest {
    fn name(&self) -> NamespacedName;
    fn has_data(&self) -> bool;
}

#[derive(Serialize)]
pub struct CoreVersion;

impl StreamduckRequest for CoreVersion {
    fn name(&self) -> NamespacedName {
        NamespacedName::new("Core", "Socket Version")
    }

    fn has_data(&self) -> bool {
        false
    }
}

#[derive(Serialize)]
pub struct ListDevices;

impl StreamduckRequest for ListDevices {
    fn name(&self) -> NamespacedName {
        NamespacedName::new("Core", "List Devices")
    }

    fn has_data(&self) -> bool {
        false
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

    fn has_data(&self) -> bool {
        true
    }
}