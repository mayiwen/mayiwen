import { app, ipcMain, shell, globalShortcut, Tray, Menu } from 'electron';
import { handleUpdate } from './update';
import { autoUpdater } from 'electron-updater';
let path = require('path')

export function ipc(win: any) {
  ipcMain.handle('PING', () => 'PONG');

  // 监听子组件传过的来的事件
	// 窗口最小化
  ipcMain.on('min', () => win.minimize());
  ipcMain.on('max', () => {
    if (win.isMaximized()) {
      win.restore();
    } else {
      win.maximize();
    }
  });
	/** 窗口最大化 */
  ipcMain.on('close', () => {
    win.close();
    app.quit();
    app.exit();
  });
  // 打开开发者工具
  ipcMain.on('f12', () => {
    win.webContents.openDevTools();
  });
  // 获取图标的软件
  ipcMain.on('imgIcon', async (event: any, params: any) => {
    try {
      if (params.value.includes('.lnk')) {
        let lnk = shell.readShortcutLink(params.value);
        let data = await app.getFileIcon(lnk.target + '', {size: 'large'})
        event.returnValue = data.toDataURL();
        return data.toDataURL();
      }
      const data = await app.getFileIcon(params.value, {size: 'large'})
      event.returnValue = data.toDataURL();
      return data.toDataURL();
    } catch (error) {
      event.returnValue = '';
      return ''
    }
  });
  handleUpdate(autoUpdater, win)
  // 获取路径 
  ipcMain.on('electronAppPathUserData',  (event: any, params: any) => {
    let path =  app.getPath('userData');
    win.webContents.send('electronAppPathUserDataReturn', path)
    let desktop = app.getPath('desktop')
    win.webContents.send('electronAppPathDesktopReturn', desktop)
  });
   // 获取路径 
  ipcMain.on('checkForUpdate', async (event: any, params: any) => {
    console.log('this is update message')
    // console.log(event)
    let res = await autoUpdater.checkForUpdates();
    console.log('this is click')
    console.log(res)
  });
  
  

  app.whenReady().then(() => {
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
    let iconPath = path.join(__dirname, 'favicon.256x256.png')
    tray = new Tray(iconPath)
    tray.on('click', function() {
      win.show()
    })
    let menu = Menu.buildFromTemplate([{
      click() { win.show() },
      label: '显示窗口',
      type: 'normal'
    }, {
      click() { app.quit() },
      label: '退出应用',
      type: 'normal'
    }])
    tray.setContextMenu(menu)
  })

 
  // app.on('ready', () => {
  //   let tray;
  //   let iconPath = path.join(__dirname, 'favicon.256x256.png')
  //   tray = new Tray(iconPath)
  // })

  // 最小化与最大化的方法。
  win.on('maximize', () => {
    try {
      win.webContents.send('main', 'maximize!')
    } catch (error) {
      console.log(error)
    }
    
  })
  win.on('unmaximize', () => {
    try {
      win.webContents.send('main', 'unmaximize!')
    } catch (error) {
      console.log(error)
    }
  })
}
