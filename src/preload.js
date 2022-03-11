import {contextBridge, ipcRenderer} from 'electron';
import streamduck from 'streamduck-node-client'

const Store = require('electron-store');
const fs = require('fs');

const store = new Store()

contextBridge.exposeInMainWorld('preload', {
    maximize: () => ipcRenderer.send('controlButton', {action: 'maximize'}),
    minimize: () => ipcRenderer.send('controlButton', {action: 'minimize'}),
    close: () => ipcRenderer.send('controlButton', {action: 'close'}),
})

contextBridge.exposeInMainWorld('store', {
    set(name, value) {
        store.set(name, value);
    },
    get(name) {
        return store.get(name);
    },
    clear(name) {
        store.delete(name);
    }
})

let dialog_response;

ipcRenderer.on('folder-dialog-response', (ev, data) => {
    dialog_response(data);
})

contextBridge.exposeInMainWorld('dialog', {
    folder() {
        ipcRenderer.send("folder-dialog");

        return new Promise((resolve) => {
            dialog_response = resolve;
        })
    }
})

contextBridge.exposeInMainWorld('fs', {
    is_folder_valid(path) {
        try {
            let stat = fs.statSync(path);
            return stat.isDirectory();
        } catch (e) {
            return false;
        }
    }
})

function connectStreamduck() {
    let client = streamduck.newUnixClient({timeout: 15000});

    let proxy = {};

    Object.getOwnPropertyNames(Object.getPrototypeOf(client)).forEach(name => {
        if (name !== "constructor") {
            proxy[name] = function () {
                let output = client[name](...(arguments));

                if (typeof output === "object") {
                    return new Promise((resolve, reject) => {
                        output.then(data => resolve(data)).catch(reason => {
                            console.log(`Failed ${name} request: ${reason}`);
                            reject(`Failed ${name} request: ${reason}`)
                        })
                    });
                } else {
                    return output;
                }
            }
        }
    });

    contextBridge.exposeInMainWorld('sd', proxy)
}

connectStreamduck();

window.addEventListener("mouseup", (e) => {
    if (e.button === 3 || e.button === 4)
        e.preventDefault();
});

console.log(`preload loaded`);
