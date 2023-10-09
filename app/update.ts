import { app, ipcMain, shell, globalShortcut } from 'electron';

 // 更新操作
export function handleUpdate(autoUpdater: any, win: any){
  console.log('handleUpdate is run')
  const returnData = {
    error: {status: -1, msg: '检测更新查询异常'},
    checking: {status: 0, msg: '正在检查应用程序更新...'},
    updateAva: {status: 1, msg: '检测到新版本，正在下载,请稍后'},
    updateNotAva: {status: -1, msg: '您现在使用的版本为最新版本,无需更新!'},
    down: {status: 2, msg: '更新完成'},
  };

  console.log('check')
  // 和之前package.json 配置一样
  autoUpdater.setFeedURL('http://121.4.117.203/update/');

  // 更新错误
  autoUpdater.on('error', function(error: any){
    sendUpdateMessage({...returnData.error, error}, win)
  })

  // 检查中
  autoUpdater.on('checking-for-update', function(){
    sendUpdateMessage(returnData.checking, win)
  })

  // 发现新版本
  autoUpdater.on('update-available', function(info: any){
    sendUpdateMessage(returnData.updateAva, win)
  })

  // 当前版本为最新版本
  autoUpdater.on('update-not-available', function(info: any){
    setTimeout(()=>{
      sendUpdateMessage(returnData.updateNotAva, win)
    }, 1000)
  })
  // 当前版本为最新版本
  autoUpdater.on('update-downloaded', function(info: any){
    setTimeout(()=>{
      autoUpdater.quitAndInstall() 
      app.quit()

      sendUpdateMessage(returnData.down, win)
    }, 1000)
  })

  
  // 有更新包下载触发方法
  autoUpdater.on('download-progress', function(progressObj: any){
    win.webContents.send('downloadProgress', progressObj)
  })
  autoUpdater.checkForUpdates();
}
 // 通过main进程发送事件给renderer进程，提示更新
 function sendUpdateMessage(text: any, win: any){
  console.log(text)
  console.log('this is updateReturn Before')
  win.webContents.send('updateReturn', text)
}
