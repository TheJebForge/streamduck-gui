/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::api::StreamduckRequest;
use crate::base::NamespacedName;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct SocketMessage {
    pub name: NamespacedName,
    pub data: Value,
    #[serde(rename = "RequestID")]
    pub request_id: Option<String>
}

impl SocketMessage {
    pub fn new_from<T>(value: T, request_id: &str) -> Result<SocketMessage, serde_json::Error>
    where T : StreamduckRequest + Serialize {
        Ok(Self {
            name: value.name(),
            data: serde_json::to_value(value)?,
            request_id: Some(request_id.to_string())
        })
    }
}