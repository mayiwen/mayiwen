import { app, BrowserWindow, screen } from 'electron';
import * as path from 'path';
import * as fs from 'fs';
import { ipc } from './ipc';
// import { handleUpdate } from './update';
// import { autoUpdater } from 'electron-updater'
let win: BrowserWindow;
const args = process.argv.slice(1),
  serve = args.some((val) => val === '--serve');

function createWindow(): BrowserWindow {
  const size = screen.getPrimaryDisplay().workAreaSize;

  // Create the browser window.
  win = new BrowserWindow({
    x: (size.width - 800) / 2,
    y: (size.height - 600) / 2,
    width: 800,
    height: 600,
    minWidth: 400,
    minHeight: 400,
    // titleBarStyle: 'hidden',
    // titleBarOverlay: {
    //   color: '#fff',
    //   symbolColor: 'black',
    // },
    frame: false, // 是否显示关闭按钮
    webPreferences: {
      nodeIntegration: true,
      allowRunningInsecureContent: serve,
      contextIsolation: false, // false if you want to run e2e test with Spectron
      webviewTag: true,
    },
  
  });

  if (serve) {
    const debug = require('electron-debug');
    debug();

    require('electron-reloader')(module);
    win.loadURL('http://localhost:4200');
    win.webContents.openDevTools();
  } else {
    // Path when running electron executable
    let pathIndex = './index.html';

    if (fs.existsSync(path.join(__dirname, '../dist/index.html'))) {
      // Path when running electron in local folder
      pathIndex = '../dist/index.html';
    }

    const url = new URL(path.join('file:', __dirname, pathIndex));
    win.loadURL(url.href);
  }
  ipc(win);
  // Emitted when the window is closed.
  win.on('closed', () => {
    // Dereference the window object, usually you would store window
    // in an array if your app supports multi windows, this is the time
    // when you should delete the corresponding element.
    win.close()
  });
  // 与启动无关的方法
  
  // win.webContents.openDevTools({ mode: 'detach' });
  return win;
}

try {
  // This method will be called when Electron has finished
  // initialization and is ready to create browser windows.
  // Some APIs can only be used after this event occurs.
  // Added 400 ms to fix the black background issue while using transparent window. More detais at https://github.com/electron/electron/issues/15947
  app.on('ready', () => setTimeout(createWindow, 400));

  // Quit when all windows are closed.
  app.on('window-all-closed', () => {
    // On OS X it is common for applications and their menu bar
    // to stay active until the user quits explicitly with Cmd + Q
    if (process.platform !== 'darwin') {
      app.quit();
    }
  });

  app.on('activate', () => {
    // On OS X it's common to re-create a window in the app when the
    // dock icon is clicked and there are no other windows open.
    if (win === null) {
      createWindow();
    }
  });

  // 禁止可以打开多个实例
  const gotTheLock = app.requestSingleInstanceLock();
  if (!gotTheLock) {
    app.quit();
  }
  {
    app.on('second-instance', (event, argv) => {
      if (process.platform === 'win32') {
        if (win) {
          if (win.isMinimized()) {
            win.restore();
          }
          if (win.isVisible()) {
            win.focus();
          } else {
            win.show();
          }
        }
      }
    });
  }
} catch (e) {
  console.error(e);
  throw e;
}
