import { ChangeDetectorRef, Component, OnInit } from '@angular/core';
import { ElectronService } from '../../../../../core/services';
const packageJson = require('../../../../../../../package.json');
import { autoUpdater } from 'electron-updater'
import { MywMessageService } from 'mayiwen_angular';
import { UpdateService } from './service/update.service';

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
  fileList: File[] = []
  filePath: string = 'C:\\Users\\Administrator\\Desktop\\nginx-1.23.1\\html\\update'
  /** 表格数据定义 */
  tableData: any = [
  ]
  /** 表头定义 */
  tableCol = [
    {
      v: '路径',
      value: 'path',
      w: 400
    },
    {
      v: '操作',
      value: 'edit',
      w: 100
    },
  ]
  /** 访问的路径 */
  parentPath: string = 'C:\\Users\\Administrator\\Desktop\\nginx-1.23.1\\html\\update'
  constructor(
    private cdr: ChangeDetectorRef,
    private es: ElectronService,
    private message: MywMessageService,
    private updateService: UpdateService
  ) {
  }
  ngOnInit(): void {
  }

  getFileList() {
    let path = this.parentPath
    this.updateService.getChildByPath(path).subscribe(res => {
      console.log(res)
      // this.tableData = res.map((item: any) =>{path:item})
      this.tableData = res.map((item: any) => {
        return {
          path: item.path,
          fatherPath: path,
          type: item.type
        }
      })
      console.log(this.tableData)
    })
  }
  update() {
    this.message.show('调用了')
    this.es.ipcRenderer.send('checkForUpdate', 'asdad');
  }

  upload() {
    console.log('上传文件的方法调用了')
    console.log(this.fileList)
    if (this.fileList && this.fileList.length > 0) {
      const formData = new FormData();
      console.log(this.fileList[0])
      formData.append("file", this.fileList[0]);
      formData.append("path", this.filePath);
      this.updateService.updateUpload(formData).subscribe(res => {
        console.log(res)
        this.message.show(res.message)
      })
    } else {
      this.message.show('请选择文件')
    }

  }
  uploadFileChange(e: any) {
    console.log('这是打印的file')
    console.log(e)
    console.log(e.target.files)
    this.fileList = e.target.files
  }

  deletePath(item: any) {
    console.log(item)
    this.updateService.delete(item.fatherPath + '\\' + item.path).subscribe(res => {
      console.log(res)
      console.log(this.tableData)
      this.getFileList()
    })
  }
  enter(item: any) {
    this.parentPath = item.fatherPath + '\\' + item.path
    this.getFileList()
  }
  top() {
    // parentPath
    console.log(this.parentPath)
    let res = this.es.path.join(this.parentPath, '../')
    console.log(res)
    console.log()
    this.parentPath = res
    this.getFileList()

  }
}
