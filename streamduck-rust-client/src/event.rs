/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use crate::api::Device;
use crate::base::NamespacedDeviceIdentifier;
use crate::ClientEvent;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct SocketEvent {
    pub plugin_name: String,
    pub event_name: String,
    pub data: Option<Value>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct SocketError {
    pub error: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum StreamduckEvent {
    #[serde(skip)]
    ClientConnected,
    #[serde(skip)]
    ClientDisconnected,
    #[serde(skip)]
    SocketError(String),

    #[serde(rename = "Core, Device Connected")]
    DeviceConnected(Device),
    #[serde(rename = "Core, Device Disconnected")]
    DeviceDisconnected(NamespacedDeviceIdentifier),
    #[serde(rename = "Core, Device Appeared")]
    DeviceAppeared(Device),
    #[serde(rename = "Core, Device Disappeared")]
    DeviceDisappeared(NamespacedDeviceIdentifier),

    Other(SocketEvent)
}

impl From<ClientEvent> for StreamduckEvent {
    fn from(value: ClientEvent) -> Self {
        match value {
            ClientEvent::Connected => StreamduckEvent::ClientConnected,
            ClientEvent::Disconnected => StreamduckEvent::ClientDisconnected,
            ClientEvent::Event(value) => {
                let mut json_map = Map::new();
                json_map.insert(
                    format!("{}, {}", value.plugin_name, value.event_name),
                    if let Some(data) = &value.data { data.clone() } else { Value::Null }
                );
                let json_value = Value::Object(json_map);

                match serde_json::from_value::<StreamduckEvent>(json_value) {
                    Ok(event) => event,
                    Err(err) => {
                        println!("Failed to parse event: {}", err);
                        StreamduckEvent::Other(value)
                    }
                }
            }
            ClientEvent::Error(error) => StreamduckEvent::SocketError(error.error)
        }
    }
}