/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
#![warn(missing_docs)]

pub mod api;
pub(crate) mod message;
pub mod event;
pub mod base;

use std::collections::HashMap;
use url::Url;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use ezsockets::{Client, ClientConfig, Error};
use ezsockets::client::ClientCloseMode;
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::{Serialize};
use serde::de::DeserializeOwned;
use thiserror::Error;
use tokio::sync::{mpsc, Mutex};
use tokio::sync::oneshot;
use crate::api::{ConnectDevice, CoreVersion, Device, GetDeviceInputs, GetDeviceItems, GetDeviceScreenStack, Input, ListDevices, PartialScreenItem, PopScreen, PushNewEmptyScreen, SetDeviceAutoconnect, StreamduckRequest};
use crate::base::NamespacedDeviceIdentifier;
use crate::event::{SocketError, SocketEvent, StreamduckEvent};
use crate::message::SocketMessage;

pub struct Streamduck {
    client: Client<ClientHandler>,
    event_receiver: Mutex<mpsc::Receiver<ClientEvent>>
}

struct ClientHandler {
    handle: Client<Self>,
    active_requests: HashMap<String, oneshot::Sender<SocketMessage>>,
    event_sender: mpsc::Sender<ClientEvent>
}

#[derive(Error, Debug)]
pub enum ClientHandlerError {
    #[error("Message didn't have Request ID")]
    MissingRequestID,
    #[error("Message for unknown sender")]
    MissingSender,
    #[error("Message didn't contain any data")]
    EmptyData,
    #[error("Error received from request: {0}")]
    RequestError(String)
}

pub(crate) enum ClientEvent {
    Connected,
    Disconnected,
    Event(SocketEvent),
    Error(SocketError)
}

#[async_trait]
impl ezsockets::ClientExt for ClientHandler {
    type Call = (SocketMessage, oneshot::Sender<SocketMessage>);

    async fn on_text(&mut self, text: String) -> std::result::Result<(), Error> {
        // Request
        if let Ok(message) = serde_json::from_str::<SocketMessage>(&text) {
            let Some(request_id) = &message.request_id else {
                return Err(Box::new(ClientHandlerError::MissingRequestID))
            };

            let Some(sender) = self.active_requests.remove(request_id) else {
                return Err(Box::new(ClientHandlerError::MissingSender))
            };

            sender.send(message).ok();
        }

        // Event
        if let Ok(event) = serde_json::from_str::<SocketEvent>(&text) {
            self.event_sender.send(ClientEvent::Event(event)).await.ok();
        }

        // Error
        if let Ok(error) = serde_json::from_str::<SocketError>(&text) {
            self.event_sender.send(ClientEvent::Error(error)).await.ok();
        }

        Ok(())
    }

    async fn on_binary(&mut self, _bytes: Vec<u8>) -> std::result::Result<(), Error> {
        Ok(())
    }

    async fn on_call(&mut self, call: Self::Call) -> std::result::Result<(), Error> {
        if let Some(request_id) = &call.0.request_id {
            self.active_requests.insert(request_id.to_string(), call.1);
        }

        self.handle.text(serde_json::to_string(&call.0)?)?;
        Ok(())
    }

    async fn on_connect(&mut self) -> std::result::Result<(), Error> {
        let _ = self.event_sender.send(ClientEvent::Connected).await;
        Ok(())
    }

    async fn on_disconnect(&mut self) -> std::result::Result<ClientCloseMode, Error> {
        let _ = self.event_sender.send(ClientEvent::Disconnected).await;
        Ok(ClientCloseMode::Reconnect)
    }
}

impl Streamduck {
    pub async fn new(url: Option<&str>) -> Result<Streamduck> {
        let url = url.unwrap_or("ws://127.0.0.1:42131");

        let url = Url::parse(url)?;
        let config = ClientConfig::new(url);

        let (tx, rx) = mpsc::channel::<ClientEvent>(50);

        let (handle, future) = ezsockets::connect(
            |handle| ClientHandler {
                handle,
                active_requests: Default::default(),
                event_sender: tx
            },
            config
        ).await;

        tokio::spawn(async move {
            future.await.unwrap();
        });

        Ok(Streamduck {
            client: handle,
            event_receiver: Mutex::new(rx)
        })
    }

    async fn do_request<S>(&self, value: S) -> Result<SocketMessage> where S : Serialize + StreamduckRequest {
        let id: String = rand::thread_rng()
            .sample_iter(Alphanumeric)
            .take(12)
            .map(char::from)
            .collect();

        let (tx, rx) = oneshot::channel::<SocketMessage>();
        let packet = SocketMessage::new_from(value, &id)?;
        self.client.call((packet, tx))?;

        Ok(rx.await?)
    }

    async fn send_request<S, R>(&self, value: S) -> Result<R> where S : Serialize + StreamduckRequest, R : DeserializeOwned {
        let message = self.do_request(value).await?;

        if message.data.is_object() && message.data.as_object().unwrap().contains_key("Error") {
            let error = serde_json::from_value::<SocketError>(message.data)?;
            return Err(anyhow!(ClientHandlerError::RequestError(error.error)))
        }

        Ok(serde_json::from_value(message.data)?)
    }

    async fn send_request_empty_response<S>(&self, value: S) -> Result<()> where S : Serialize + StreamduckRequest {
        self.do_request(value).await?;
        Ok(())
    }

    pub async fn wait_for_event(&self) -> Option<StreamduckEvent> {
        let mut receiver = self.event_receiver.lock().await;
        let socket_event = receiver.recv().await;

        Some(StreamduckEvent::from(socket_event?))
    }

    pub async fn core_version(&self) -> Result<String> {
        Ok(self.send_request(CoreVersion).await?)
    }

    pub async fn list_devices(&self) -> Result<Vec<Device>> {
        Ok(self.send_request(ListDevices).await?)
    }

    pub async fn set_device_autoconnect(&self, identifier: NamespacedDeviceIdentifier, autoconnect: bool) -> Result<()> {
        Ok(self.send_request_empty_response(SetDeviceAutoconnect {
            identifier,
            autoconnect,
        }).await?)
    }

    pub async fn get_device_inputs(&self, identifier: NamespacedDeviceIdentifier) -> Result<Vec<Input>> {
        Ok(self.send_request(GetDeviceInputs {
            identifier,
        }).await?)
    }

    pub async fn connect_device(&self, identifier: NamespacedDeviceIdentifier) -> Result<bool> {
        Ok(self.send_request(ConnectDevice {
            identifier
        }).await?)
    }
    
    pub async fn get_device_items(&self, identifier: NamespacedDeviceIdentifier, previews: Option<bool>) -> Result<Vec<Option<PartialScreenItem>>> {
        Ok(self.send_request(GetDeviceItems {
            identifier,
            get_previews: previews.unwrap_or(false),
        }).await?)
    }
    
    pub async fn get_device_screen_stack(&self, identifier: NamespacedDeviceIdentifier) -> Result<Vec<String>> {
        Ok(self.send_request(GetDeviceScreenStack {
            identifier
        }).await?)
    }
    
    pub async fn pop_screen(&self, identifier: NamespacedDeviceIdentifier) -> Result<bool> {
        Ok(self.send_request(PopScreen {
            identifier
        }).await?)
    }
    
    pub async fn push_new_empty_screen(&self, identifier: NamespacedDeviceIdentifier) -> Result<()> {
        Ok(self.send_request_empty_response(PushNewEmptyScreen {
            identifier
        }).await?)
    }
}