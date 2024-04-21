/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

mod ui;

use std::sync::{Arc, Condvar};
use streamduck_rust_client::event::StreamduckEvent;
use streamduck_rust_client::Streamduck;
use tokio::sync::mpsc;
use streamduck_rust_client::api::{Device, Input};
use streamduck_rust_client::base::NamespacedDeviceIdentifier;
use crate::ui::{ui_main, UIMessage};

#[tokio::main]
async fn main() {
    let streamduck = Arc::new(Streamduck::new(None).await.unwrap());

    let (api_tx, api_rx) = mpsc::channel::<APIMessage>(50);
    let (ui_tx, mut ui_rx) = mpsc::channel::<UIMessage>(50);

    let (waker, waiter) = mpsc::channel::<()>(1);
    let waker_copy = waker.clone();

    let (streamduck_copy, api_tx_copy) = (streamduck.clone(), api_tx.clone());
    let receive_events = async move {
        while let Some(event) = streamduck.wait_for_event().await {
            if match event {
                StreamduckEvent::DeviceConnected(device) => {
                    api_tx.send(APIMessage::ConnectedDevice(device)).await.ok();
                    true
                }
                StreamduckEvent::DeviceDisconnected(device) => {
                    api_tx.send(APIMessage::DisconnectedDevice(device)).await.ok();
                    true
                }
                StreamduckEvent::DeviceAppeared(device) => {
                    api_tx.send(APIMessage::NewDevice(device)).await.ok();
                    true
                }
                StreamduckEvent::DeviceDisappeared(device) => {
                    api_tx.send(APIMessage::DeviceGone(device)).await.ok();
                    true
                }
                StreamduckEvent::Other(_) => false,
                StreamduckEvent::ClientConnected => {
                    println!("Connected!");
                    false
                }
                StreamduckEvent::ClientDisconnected => {
                    println!("Disconnected!");
                    false
                }
                StreamduckEvent::SocketError(error) => {
                    println!("Error from socket! {}", error);
                    false
                }
            } {
                waker.send(()).await.ok();
            }
        }
    };

    let receive_ui_messages = async move {
        let devices = streamduck_copy.list_devices().await.unwrap();
        api_tx_copy.send(APIMessage::DeviceList(devices)).await.ok();
        waker_copy.send(()).await.ok();

        while let Some(message) = ui_rx.recv().await {
            match message {
                UIMessage::SetDeviceAutoconnect { identifier, autoconnect } => {
                    streamduck_copy.set_device_autoconnect(identifier, autoconnect).await.ok();
                }
                UIMessage::ConnectDevice(identifier) => {
                    streamduck_copy.connect_device(identifier).await.ok();
                }
                UIMessage::GetGrid(identifier) => {
                    match streamduck_copy.get_device_inputs(identifier).await {
                        Ok(grid) => {
                            api_tx_copy.send(APIMessage::InputGrid(grid)).await.ok();
                        }
                        Err(error) => {
                            println!("Error while trying to get inputs! {error}")
                        }
                    }
                }
            }
        }
    };

    tokio::spawn(receive_events);
    tokio::spawn(receive_ui_messages);

    ui_main(ui_tx, api_rx, waiter)
}

pub enum APIMessage {
    DeviceList(Vec<Device>),

    NewDevice(Device),
    DeviceGone(NamespacedDeviceIdentifier),

    ConnectedDevice(Device),
    DisconnectedDevice(NamespacedDeviceIdentifier),

    InputGrid(Vec<Input>)
}