import {
  AfterViewInit,
  ChangeDetectorRef,
  Component,
  OnInit,
  ViewChild,
} from '@angular/core';
import { MindMapService } from './mind-map.service';
import { ElectronService } from '../../core/services';
import { v4 as uuidv4 } from 'uuid';
import { TreeI } from './module/treemap/o/tree-item.i';
import { WangEditorComponent } from './editor/wangeditor/wangeditor.component';
import { FileService } from '../common/file/service/file.service';
import { timeStamp } from 'console';
import { MywMessageService } from 'mayiwen_angular';
@Component({
  selector: 'app-mind-map',
  templateUrl: './mind-map.component.html',
  styleUrls: ['./mind-map.component.less'],
  providers: [MindMapService, FileService],
})
export class MindMapComponent implements OnInit, AfterViewInit {
  @ViewChild('wangeditorRef') wangeditorRef!: WangEditorComponent;
  timer: any
  treeData: TreeI[] = [
    {
      v: '1',
      value: '1',
      uuid: uuidv4(),
    },
  ];
  file = {
    name: '',
    path: '',
  };
  selectTab = '1';
  selectTreeI: TreeI = {};
  flagEditor = true
  constructor(
    private es: ElectronService,
    private ms: MindMapService,
    private fs: FileService,
    private message: MywMessageService
  ) {}
  saveAll() {
    this.saveTxt()
    this.save()
    let savePath = this.es.path.join(this.es.pathUserDataString, 'editor', this.file.name.split('.')[0])
      if (this.timer) clearTimeout(this.timer)
      this.es.compressing.tgz.compressDir(savePath, this.file.path).then((res) => {
        this.message.show('更新.myw文件成功')
      });
  }

  delete(item: any) {
    // 调用了删除的方法

  }

  dateLoop(data: TreeI[]) {
    return data.map((item) => {
      let data: TreeI = {
        v: item.v,
        value: item.value,
        uuid: item.uuid,
      };
      if (item.children && item.children.length > 0) {
        data.children = this.dateLoop(item.children);
      }
      return data;
    });
  }

  ngAfterViewInit(): void {
    this.ms.subject.subscribe({
      next: (dat: TreeI) => {
        this.saveAll()
        if (this.selectTreeI.uuid === dat.uuid) {
          this.message.show('没有触发')
          return
        }
        this.flagEditor = false
        this.flagEditor = true;
        
        
        // let timer = setTimeout(() => {
          this.selectTreeI = dat as any;
          // 判断一个文件夹是否存在。
          if (this.es.fs.existsSync(this.es.pathUserDataString)) {
            let path = this.es.path.join(this.es.pathUserDataString, 'editor', this.file.name.split('.')[0], dat.uuid + '', 'index.txt')
            if (this.es.fs.existsSync(path)) {
              this.es.fs.readFile(path, 'utf-8', (err, dat) => {
                    if (err) {
                      console.log(err);
                      return;
                    }
                    this.wangeditorRef.setEditorHtml(dat);
                  });
            } else {
              this.wangeditorRef.setEditorHtml('<p><br></p>');
            }
          }
        //   clearTimeout(timer)
        // }, 500);
      },
      error: (error) => {
        console.error(error);
      },
    });
    this.ms.$editSubject.subscribe({
      next: (data) => {
        console.log(data)
        if (data.type === 'delete') {
          this.message.show('delete')
          console.log('这是data item')
          console.log(data.item)
          
          let str = this.es.path.join(this.es.pathUserDataString, 'editor', this.file.name.split('.')[0], data.item.uuid)
          let arr: TreeI[] = []
          this.loopDelete(data.item, arr)
          console.log('这是arr')
          console.log(arr)
          console.log('这是待删除的文件夹')
          console.log(str)
          this.fs.deleteDir(str)
          arr.forEach((item: TreeI )=> {
            let deletepath = this.es.path.join(this.es.pathUserDataString, 'editor', this.file.name.split('.')[0], item.uuid + '')
            this.fs.deleteDir(deletepath)
          })
          this.selectTreeI = {};
        }
      }
    })
    this.ms.$dragSubject.subscribe({
      next: (data) => {
        if (data.type === 'delete_no_index') {
          this.message.show('delete_no_index')
          console.log('这是传过来的data数据')
          console.log(data.obj)
          this.loopTree(this.treeData, { children: this.treeData}, '', (element: any, father: any, indexFather: any, index: any) => {
            if (data.obj.dragId === indexFather) {
              father.children.splice(index, 1)
            }
          })
          this.loopTree(this.treeData, { children: this.treeData}, '', (element: any, father: any, indexFather: any, index: any) => {
            if (data.obj.dropId === indexFather) {
              father.children.splice(index, 0, data.obj.item)
            }
          })
        }
        if (data.type === 'delete_edit_index') {
          this.message.show('delete_edit_index')
          this.loopTree(this.treeData, { children: this.treeData}, '', (element: any, father: any, indexFather: any, index: any) => {
            if (data.obj.dropId === indexFather) {
              father.children.splice(index + 1, 0, data.obj.item)
            }
          })
          this.loopTree(this.treeData, { children: this.treeData}, '', (element: any, father: any, indexFather: any, index: any) => {
            if (data.obj.dragId === indexFather) {
              father.children.splice(index , 1)
            }
          })
        }
      }
    })
  }
  loopDelete(treei:TreeI, arr: TreeI[]) {
    if (treei.children && treei.children.length > 0) {
      treei.children.forEach(item => {
        arr.push(item)
        this.loopDelete(item, arr)
      })
    }
  }
  loopTree(treeIList: TreeI[], father: TreeI, indexFather: any, fn: any ) {
    for (let index = 0; index < treeIList.length; index++) {
      const element = treeIList[index];
      // 计算当前定标
      // 这是当前定标
      let treeId = indexFather ? indexFather + '-' + index : index + ''; // 计算当前的定标
      console.log('这是定标')
      console.log(treeId)
      fn(element, father, treeId, index)
      if (element.children && element.children.length > 0) {
        this.loopTree(element.children, element, treeId, fn)
      }
    }
  }
  addZero(num: any) {
    let str = ''
    for (let index = 0; index < num; index++) {
      str += '0'
    }
    return str
  }
  /** 如果是同级的要 */
  editLevelSame() {

  }
  loop(treeIList: TreeI[], indexFather: string, fn: any) {
    // fn(treeIList, indexFather)
    for (let index = 0; index < treeIList.length; index++) {
      const element = treeIList[index];
      if (element.children && element.children.length > 0) {
        this.loop(element.children, indexFather ? indexFather + '-' + index : index + '', fn)
      }
      
    }
  }

  save() {
    const exportData = this.dateLoop(this.treeData);
    this.fs.pathUserDataMayiwenEditor();
    let signalPath = this.fs.pathUserDataMayiwenEditorSingle(this.file.name.split('.')[0]);
    if (!this.es.fs.existsSync(signalPath)) {
      this.es.fs.mkdirSync(signalPath);
    }
    let jsonPath = this.es.path.join(signalPath, 'index.json');
    this.es.fs.writeFileSync(jsonPath, JSON.stringify(exportData));
    console.log(this.message.show('更新树数据成功'))
  }

  /** 保存下方的内容 */
  saveTxt() {
    if (!this.selectTreeI.uuid) return;
      const txt = this.wangeditorRef.getEditorHtml();
      let signalPath = this.fs.pathUserDataMayiwenEditorSingle(
        this.file.name.split('.')[0]
      );
      if (!this.es.fs.existsSync(signalPath)) {
        this.es.fs.mkdirSync(signalPath);
      }
      let uuidPath = this.es.path.join(signalPath, this.selectTreeI.uuid)
      if (!this.es.fs.existsSync(uuidPath)) {
        this.es.fs.mkdirSync(uuidPath);
      }
      console.log('写入文件')
      console.log(txt)
      this.es.fs.writeFileSync(this.es.path.join(uuidPath, 'index.txt'), txt);
      this.message.show('保存文件成功    ' + this.selectTreeI.v + '    富文本文档')
  }
  ngOnInit(): void {
    // throw new Error('Method not implemented.');
  }

  async dropFile(e: any) {
    e.preventDefault();
    console.log(e.dataTransfer.files);
    let ediotr = this.es.path.join(this.es.pathUserDataString, 'editor');
    let fileList: FileList = e.dataTransfer.files;
    if (fileList.length > 1 || fileList[0].name.split('.')[1] !== 'myw') {
      return alert('暂不支持打开多个文件或不是.myw结尾的文件');
    }
    if (this.file.path) {
      alert('不支持打开多个文件')
      return;
    }
    console.log(fileList[0]);
    // 获取到文件的位置
    this.file.name = fileList[0].name;
    this.file.path = fileList[0].path;
    this.fs.clearEditor();
    // 把文件放到编辑区，以解压编辑
    this.fs.pathUserDataMayiwenEditor();
    this.fs.pathUserDataMayiwenEditorSingle(this.file.name);
    let fileSave = this.es.path.join(
      this.es.pathUserDataString,
      'editor',
      this.file.name,
      'file.gz'
    );
    this.es.fs.copyFileSync(this.file.path, fileSave);
    await this.es.compressing.tgz.uncompress(
      fileSave,
      this.es.path.join(this.es.pathUserDataString, 'editor')
    );
    let fileSaveIndex = this.es.path.join(
      this.es.pathUserDataString,
      'editor',
      this.file.name.split('.')[0],
      'index.json'
    );
    // 读取当前的文件夹
    let res = this.es.fs.readFileSync(fileSaveIndex, 'utf8');
    console.log(res);
    this.treeData = JSON.parse(res);
    this.treeData.length = 0
    JSON.parse(res).forEach((item: any) => {
      this.treeData.push(item)
    })
    console.log('加载文件后打印的数据')
    console.log(this.treeData)
  }
  changeHtml(e: any) {
    // this.saveTxt();
  }
  resetFile() {
    this.file = { name: '', path: ''}
    this.treeData = []
    this.selectTreeI = {}
  }
}
