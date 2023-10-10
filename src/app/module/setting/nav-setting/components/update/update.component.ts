import { ChangeDetectorRef, Component, OnInit } from '@angular/core';
import { ElectronService } from '../../../../../core/services';
const packageJson = require('../../../../../../../package.json');
import { autoUpdater } from 'electron-updater'
import { MywMessageService } from 'mayiwen_angular';

@Component({
  selector: 'app-update',
  templateUrl: './update.component.html',
  styleUrls: ['./update.component.less']
})
/**
 * 这是一个帮助的页面
 */
export class UpdateComponent implements OnInit {
  version = '';
  file: any
  fileList?: FileList
  constructor(
    private cdr: ChangeDetectorRef,
    private es: ElectronService,
    private message: MywMessageService,
    ) {
  }
  ngOnInit(): void {
  }
  update() {
    this.message.show('调用了')
    this.es.ipcRenderer.send('checkForUpdate', 'asdad');
  }

  upload() {
    console.log('上传文件的方法调用了')
    console.log(this.fileList)
  }
  uploadFileChange(e: any) {
    console.log('这是打印的file')
    console.log(e)
    console.log(e.target.files)
    this.fileList = e.target.files
  }
}
