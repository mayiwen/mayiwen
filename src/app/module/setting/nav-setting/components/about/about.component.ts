import { ChangeDetectorRef, Component, OnInit } from '@angular/core';
import { ElectronService } from '../../../../../core/services';
const packageJson = require('../../../../../../../package.json');
import { autoUpdater } from 'electron-updater'
import { MywMessageService } from 'mayiwen_angular';

@Component({
  selector: 'app-setting-abuot',
  templateUrl: './about.component.html',
  styleUrls: ['./about.component.less']
})
/**
 * 这是一个帮助的页面
 */
export class AboutComponent implements OnInit {
  version = '';
  percent: number =  0;
  constructor(
    private cdr: ChangeDetectorRef,
    private es: ElectronService,
    private message: MywMessageService,
    ) {
  }
  ngOnInit(): void {
    this.version =  packageJson.version.replaceAll('"', '');
    // throw new Error('Method not implemented.');
     // 更新进度
     this.es.ipcRenderer.on('downloadProgress', (event, data)=>{
      console.log('这是更新进度')
      console.log(event)
      console.log(data)
      if (data.percent) {
        this.percent = data.percent
      }
    });
     this.es.ipcRenderer.on('downloadProgress', (event, data)=>{
      console.log('这是更新进度')
      console.log(event)
      console.log(data)
    });

    this.es.ipcRenderer.on('updateReturn', (e, data) => {
      console.log('updateReturn')
      console.log(data)
      if (data && data.msg) {
        this.message.show(data.msg)
      }
    })
  }
  /** 跳转页面 */
  toMayiwen(e = 'http://mayiwen.com') {
    this.es.shell.openExternal(e);
  }
  update() {
    this.message.show('调用了')
    this.es.ipcRenderer.send('checkForUpdate', 'asdad');
  }
}
