import {
  AfterViewInit,
  ChangeDetectorRef,
  Component,
  OnInit,
  ViewChild,
} from '@angular/core';
import { title } from 'process';
import { LocalStorage } from '../../common/local-stroge';
import { ElectronService } from '../../core/services';
import { DesktopService } from './desktop.service';
import { DesktopArrInterface } from './vo/link';
@Component({
  selector: 'app-desktop',
  templateUrl: './desktop.component.html',
  styleUrls: ['./desktop.component.less'],
  providers: [DesktopService, LocalStorage],
})
export class DesktopComponent implements OnInit, AfterViewInit {
  /** 当前选中的下标 */
  selectTab = '1';
  desktopArr: DesktopArrInterface[] = [
    // 
    // { 
    //   v: '默认',
    //   children: [
    //     {
    //       v: '微信',
    //       value: 'C:\\Program Files (x86)\\Tencent\\WeChat\\WeChat.exe',
    //       img: '111',
    //       type: 'm',
    //       flagShowDelete: false,
    //     },
    //   ],
    // },
  ];
  /** 选中的下标 */
  selectIndex = 0;
  dragStartSaveObj = {} as any;
  constructor(private es: ElectronService, private localStorage: LocalStorage, private cdr: ChangeDetectorRef) {}
  ngAfterViewInit(): void {
    this.getIcon();
  }
  async ngOnInit() {
    this.desktopArr = this.localStorage.getObject('desktopArr');
    this.showDeleteFalse();
    console.log('这是desktopArr');
    console.log(this.desktopArr);
    if (!this.desktopArr || this.desktopArr.length === 0) {
      this.desktopArr = [{
        v: '默认',
        children: []
      }];
    }
  }
  
  getIcon() {
    setTimeout(() => {
      this.desktopArr.forEach((tab) => {
        tab.children.forEach((item) => {
          const returnValue = this.es.ipcRenderer.sendSync('imgIcon', {
            value: item.value,
          });
          item.img = returnValue;
        });
      });
    }, 1000);
    
  }
  /** 用于屏弊拖动的默认事件 */
  dragDefault(e: any) {
    e.preventDefault();
  }
  /** 最大的div框的拖动 */
  containerDrop(e: any) {
    console.log('方法调用了');
    e.preventDefault();
    console.log(e);
    const files = [] as any;
    [].forEach.call(
      e.dataTransfer.files,
      (file) => {
        files.push(file);
      },
      false
    );
    console.log('这是files');
    console.log(files);
    if (!files || files.length === 0 || files.some((item: any) => !item.path)) {
      return false;
    }
    console.log('这是文件');
    console.log(files);
    files.forEach(async (item: any) => {
      this.desktopArr[0].children.push({
        v: item.name,
        value: item.path,
        img: '',
        type: 's',
        flagShowDelete: false
      });
    });
    this.getIcon();
    this.localStorageSet();
  }

  deleteItem(e: any, item: any, tabIndex: any, index: any) {
    e.preventDefault();
    console.log('这是删除的对象');
    console.log(item);
    console.log(tabIndex);
    console.log(index);
    this.desktopArr[tabIndex].children.splice(index, 1);
    this.localStorageSet();
  }
  changeSelect(e: any) {
    console.log('这是select切换的');
    console.log(e);
  }
  /** 调用electron打开文件的 */
  openFile(item: any) {
    this.es.shell.openPath(item.value);
  }
  /** 数值变化后，修改local storage */
  localStorageSet() {
    console.log('调用了存储的方法')
    console.log(this.desktopArr)
    // let desktopArrTemp = this.desktopArr.map(tab => {
    //   if (tab.children && tab.children.length > 0) {
    //     let map = tab.children.map(item => {
    //       delete item.img
    //       return item
    //     })
    //     tab.children = map
    //   }
    //   return tab
    // })
    this.localStorage.setObject('desktopArr', this.desktopArr);
  }

  /** 当点击删除的时候，会把别的显示的删除的图标清除 */
  showDeleteFalse() {
    this.desktopArr.forEach((tabItemLoop, tabIndexLoop) => {
      tabItemLoop.children.forEach((itemLoop, itemIndexLoop) => {
          itemLoop.flagShowDelete = false;
      })
    })
    this.localStorageSet();
  }
  /** 当点击删除的时候，会把别的显示的删除的图标清除 */
  deleteButtonShowOrHidden(item: any, tabIndex: any, index: any) {
    this.desktopArr.forEach((tabItemLoop, tabIndexLoop) => {
      tabItemLoop.children.forEach((itemLoop, itemIndexLoop) => {
        if (tabIndexLoop === tabIndex && itemIndexLoop === index) {
          itemLoop.flagShowDelete = !itemLoop.flagShowDelete
        } else {
          itemLoop.flagShowDelete = false;
        }
      })
    })
    this.localStorageSet();
  }

  dragStart(item: any, index: any, tabIndex: any){
    this.dragStartSaveObj = {
      item,
      index,
      tabIndex
    };
    console.log('这是start');
    console.log(this.dragStartSaveObj);
  }
  linkDrop(e: any, item: any, index: any, tabIndex: any) {
    e.preventDefault();
    // let saveObj = this.dragStartSaveObj.tabIndex.children[this.dragStartSaveObj.index];
    this.desktopArr[this.dragStartSaveObj.tabIndex].children.splice(this.dragStartSaveObj.index, 1);
    this.desktopArr[tabIndex].children.splice(index, 0, this.dragStartSaveObj.item);
    console.log('这是最后打印的11111111111');
    console.log(this.desktopArr);
    this.localStorageSet();
  }
}
