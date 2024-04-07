/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
//! Base types of Streamduck

use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct NamespacedName {
    pub plugin_name: String,
    pub name: String
}

impl NamespacedName {
    pub fn new(plugin_name: &str, name: &str) -> NamespacedName {
        NamespacedName {
            plugin_name: plugin_name.to_string(),
            name: name.to_string()
        }
    }
}

impl Display for NamespacedName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.plugin_name)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DeviceIdentifier {
    pub identifier: String,
    pub description: String
}

impl DeviceIdentifier {
    pub fn new(identifier: &str, description: &str) -> DeviceIdentifier {
        DeviceIdentifier {
            identifier: identifier.to_string(),
            description: description.to_string()
        }
    }
}

impl Display for DeviceIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.identifier, self.description)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq)]
pub struct NamespacedDeviceIdentifier {
    #[serde(rename = "NamespacedName")]
    pub name: NamespacedName,
    #[serde(rename = "DeviceIdentifier")]
    pub identifier: DeviceIdentifier
}

impl Display for NamespacedDeviceIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} by {}", self.identifier, self.name)
    }
}