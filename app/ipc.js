"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.ipc = void 0;
const electron_1 = require("electron");
const update_1 = require("./update");
const electron_updater_1 = require("electron-updater");
let path = require('path');
function ipc(win) {
    electron_1.ipcMain.handle('PING', () => 'PONG');
    // 监听子组件传过的来的事件
    // 窗口最小化
    electron_1.ipcMain.on('min', () => win.minimize());
    electron_1.ipcMain.on('max', () => {
        if (win.isMaximized()) {
            win.restore();
        }
        else {
            win.maximize();
        }
    });
    /** 窗口最大化 */
    electron_1.ipcMain.on('close', () => {
        win.close();
        electron_1.app.quit();
        electron_1.app.exit();
    });
    // 打开开发者工具
    electron_1.ipcMain.on('f12', () => {
        win.webContents.openDevTools();
    });
    // 获取图标的软件
    electron_1.ipcMain.on('imgIcon', (event, params) => __awaiter(this, void 0, void 0, function* () {
        try {
            if (params.value.includes('.lnk')) {
                let lnk = electron_1.shell.readShortcutLink(params.value);
                let data = yield electron_1.app.getFileIcon(lnk.target + '', { size: 'large' });
                event.returnValue = data.toDataURL();
                return data.toDataURL();
            }
            const data = yield electron_1.app.getFileIcon(params.value, { size: 'large' });
            event.returnValue = data.toDataURL();
            return data.toDataURL();
        }
        catch (error) {
            event.returnValue = '';
            return '';
        }
    }));
    (0, update_1.handleUpdate)(electron_updater_1.autoUpdater, win);
    // 获取路径 
    electron_1.ipcMain.on('electronAppPathUserData', (event, params) => {
        let path = electron_1.app.getPath('userData');
        win.webContents.send('electronAppPathUserDataReturn', path);
        let desktop = electron_1.app.getPath('desktop');
        win.webContents.send('electronAppPathDesktopReturn', desktop);
    });
    // 获取路径 
    electron_1.ipcMain.on('checkForUpdate', (event, params) => __awaiter(this, void 0, void 0, function* () {
        console.log('this is update message');
        // console.log(event)
        let res = yield electron_updater_1.autoUpdater.checkForUpdates();
        console.log('this is click');
        console.log(res);
    }));
    electron_1.app.whenReady().then(() => {
        // 注册一个'CommandOrControl+q，当有vscode的时候，打开vscode' 快捷键监听器
        // const ret = globalShortcut.register('CommandOrControl+q', () => {
        //   console.log('CommandOrControl+q is pressed')
        //   shell.openPath('C:\\Program Files\\Microsoft VS Code\\code.exe');
        // })
        // if (!ret) {
        //   console.log('registration failed')
        // }
        // 检查快捷键是否注册成功
        // console.log(globalShortcut.isRegistered('CommandOrControl+q'))
        let tray;
        let iconPath = path.join(__dirname, 'favicon.256x256.png');
        tray = new electron_1.Tray(iconPath);
        tray.on('click', function () {
            win.show();
        });
        let menu = electron_1.Menu.buildFromTemplate([{
                click() { win.show(); },
                label: '显示窗口',
                type: 'normal'
            }, {
                click() { electron_1.app.quit(); },
                label: '退出应用',
                type: 'normal'
            }]);
        tray.setContextMenu(menu);
    });
    // app.on('ready', () => {
    //   let tray;
    //   let iconPath = path.join(__dirname, 'favicon.256x256.png')
    //   tray = new Tray(iconPath)
    // })
    // 最小化与最大化的方法。
    win.on('maximize', () => {
        try {
            win.webContents.send('main', 'maximize!');
        }
        catch (error) {
            console.log(error);
        }
    });
    win.on('unmaximize', () => {
        try {
            win.webContents.send('main', 'unmaximize!');
        }
        catch (error) {
            console.log(error);
        }
    });
}
exports.ipc = ipc;
//# sourceMappingURL=ipc.js.map