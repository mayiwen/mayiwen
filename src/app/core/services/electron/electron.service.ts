import { Injectable } from '@angular/core';

// If you import a module but never use any of the imported values other than as TypeScript types,
// the resulting javascript file will look as if you never imported the module at all.
import { ipcRenderer, webFrame, app, shell } from 'electron';
import * as childProcess from 'child_process';
import * as fs from 'fs';
import * as zlib from 'zlib'
import * as path from 'path'
import * as compressing from 'compressing'
import {autoUpdater} from 'electron-updater'
@Injectable({
  providedIn: 'root'
})
export class ElectronService {

  ipcRenderer!: typeof ipcRenderer;
  webFrame!: typeof webFrame;
  shell!: typeof shell;
  app!: typeof app;
  childProcess!: typeof childProcess;
  fs!: typeof fs;
  zlib!: typeof zlib;
  path!: typeof path;
  autoUpdater!: typeof autoUpdater;
  /** 获取用户文件夹 */
  pathUserDataString = ''
  pathDesktopString = ''
  compressing!: typeof compressing
  constructor() {
    // Conditional imports
    if (this.isElectron) {
      this.ipcRenderer = window.require('electron').ipcRenderer;
      this.webFrame = window.require('electron').webFrame;
      this.app = window.require('electron').app;
      this.shell = window.require('electron').shell;
      this.fs = window.require('fs');
      this.zlib = window.require('zlib');
      this.path = window.require('path');
      this.compressing = window.require('compressing')
      this.childProcess = window.require('child_process');
      this.autoUpdater = window.require('electron-updater')
      this.childProcess.exec('node -v', (error, stdout, stderr) => {
        if (error) {
          console.error(`error: ${error.message}`);
          return;
        }
        if (stderr) {
          console.error(`stderr: ${stderr}`);
          return;
        }
        console.log(`stdout:\n${stdout}`);
      });
      const returnValue = this.ipcRenderer.send('electronAppPathUserData', {});
      this.ipcRenderer.on('electronAppPathUserDataReturn', (event: any, message: any) => {
        console.log('这是返回的message')
        console.log(message)
        this.pathUserDataString = message;
      })
      this.ipcRenderer.on('electronAppPathDesktopReturn', (event: any, message: any) => {
        console.log('pathDesktopString')
        console.log(message)
        this.pathDesktopString = message;
      })
      this.ipcRenderer.on('update', (event:any, message:any) => {
        console.log('这是返回的数据。')
        console.log(event)
        console.log(message)
      })

      // Notes :
      // * A NodeJS's dependency imported with 'window.require' MUST BE present in `dependencies` of both `app/package.json`
      // and `package.json (root folder)` in order to make it work here in Electron's Renderer process (src folder)
      // because it will loaded at runtime by Electron.
      // * A NodeJS's dependency imported with TS module import (ex: import { Dropbox } from 'dropbox') CAN only be present
      // in `dependencies` of `package.json (root folder)` because it is loaded during build phase and does not need to be
      // in the final bundle. Reminder : only if not used in Electron's Main process (app folder)

      // If you want to use a NodeJS 3rd party deps in Renderer process,
      // ipcRenderer.invoke can serve many common use cases.
      // https://www.electronjs.org/docs/latest/api/ipc-renderer#ipcrendererinvokechannel-args
    }
  }

  get isElectron(): boolean {
    return !!(window && window.process && window.process.type);
  }
}
