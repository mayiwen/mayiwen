import {
  AfterViewInit,
  ChangeDetectorRef,
  Component,
  forwardRef,
  Input,
  OnInit,
  ViewChild,
} from '@angular/core';
import { ControlValueAccessor, NG_VALUE_ACCESSOR } from '@angular/forms';
import { TreeI } from './o/tree-item.i';
import { nanoid } from 'nanoid';
import { MindMapService } from '../../mind-map.service';
import { MywAlertService, MywMessageService } from 'mayiwen_angular';
@Component({
  selector: 'app-mind-map-tree-map',
  templateUrl: './treemap.component.html',
  styleUrls: ['./treemap.component.less'],
  providers: [
    {
      provide: NG_VALUE_ACCESSOR,
      useExisting: forwardRef(() => TreeMapComponent),
      multi: true,
    },
  ],
})
export class TreeMapComponent
  implements OnInit, AfterViewInit, ControlValueAccessor
{
  @Input() indexId = '';
  _fatherNgModel: any = {};

  constructor(
    private ms: MindMapService,
    private message: MywMessageService,
    private alert: MywAlertService
  ) {}
  get fatherNgModel() {
    return this._fatherNgModel;
  }
  set fatherNgModel(data: any) {
    // 此处动用了getf方法
    this._fatherNgModel = data;
    this.change(this.fatherNgModel);
  }
  change = (value: any) => {
    console.log('changge');
    console.log(value);
  };
  // 先定义一个方法，很重要，用于接收registerOnChange()方法里传递回来的方法，然后通过这个方法就能通知到外部组件数据更新。
  // 这个是外部给内部赋值的方法。
  writeValue(val: any): void {
    console.log('这是write value，父组件给里面的原素赋值了。');
    console.log(val);
    if (val) {
      /* 此处动调用了fatherNgModel的set法法。 */
      this.fatherNgModel = val.map((item: any) => {
        item.flagExpand = true;
        item.flagEdit = false;
        return item;
      });
    }
  }
  registerOnChange(fn: any): void {
    console.log('registerOnChange');
    this.change = fn;
  }
  registerOnTouched(fn: any): void {
    console.log('registerOnTouched');
  }
  setDisabledState?(isDisabled: boolean): void {
    console.log('setDisabledState');
  }

  ngAfterViewInit(): void {
    // throw new Error('Method not implemented.');
  }
  ngOnInit(): void {
    // throw new Error('Method not implemented.');
    console.log('ngOnInit 执行了');
  }
  /** 添加一个子元素 */
  addson(index: any) {
    console.log(this.fatherNgModel);
    if (
      this.fatherNgModel[index].children &&
      this.fatherNgModel[index].children.length > 0
    ) {
      this.fatherNgModel[index].children.push({
        v: '',
        value: '',
        uuid: nanoid(),
      });
    } else {
      this.fatherNgModel[index].children = [
        {
          v: '',
          value: '',
          uuid: nanoid(),
        },
      ];
    }
  }
  addElderBrother(index: any) {
    this.fatherNgModel.splice(index, 0, {
      v: '',
      value: '',
      uuid: nanoid(),
    });
  }
  addYoungerBrother(index: any) {
    this.fatherNgModel.splice(index + 1, 0, {
      v: '',
      value: '',
      uuid: nanoid(),
    });
  }
  addFather(index: any) {
    let copy = JSON.parse(JSON.stringify(this.fatherNgModel[index]));
    this.fatherNgModel[index].v = '';
    this.fatherNgModel[index].value = '';
    this.fatherNgModel[index].children = [copy];
    this.fatherNgModel[index].uuid = nanoid();
  }
  delete(index: any, item: any) {
    this.alert.show({
      message: '由于未支持历史记录功能，删除后将不可以恢复，请慎重操作。是否确认删除？',
      success: () => {
        this.fatherNgModel.splice(index, 1);
        this.ms.$editSubject.next({
          type: 'delete',
          item,
        });
      }
    })
    
  }
  getMessage(treei: TreeI, i: any, e: any) {
    let treeId = this.indexId ? this.indexId + '-' + i : i + '';
    let treeiCopy: TreeI = JSON.parse(JSON.stringify(treei));
    treeiCopy.treeId = treeId;
    this.ms.subject.next(treeiCopy);
    let timer = setTimeout(() => {
      console.log(e)
      e.view.focus()
      e.target.focus()
      clearTimeout(timer)
    }, 300);
  }
  dragTreeItem(item: TreeI, index: any, e: any) {
    e.stopPropagation();
    console.log('拖动开始了');
    console.log(this.indexId ? this.indexId + '-' + index : index + '');
    let drapItem = {
      indexId: this.indexId ? this.indexId + '-' + index : index + '',
      item: item,
    };
    this.ms.drapItem = JSON.parse(JSON.stringify(drapItem));
  }
  dropItem(item: TreeI, index: any, e: any) {
    e.stopPropagation();
    let dragId = this.ms.drapItem.indexId;
    let dropId = this.indexId ? this.indexId + '-' + index : index + '';
    console.log('拖动到了这里')
    // 父元素移动到子元素下是不可以的。
    if (dragId === dropId.slice(0, dragId.length)) {
      this.message.show('拖动的元素不可以放到他的子元素中');
      return false;
    }
    let drapIdSave = ''
    let dropIdSave = ''
    console.log('基础数据')
    console.log(dragId)
    console.log(dropId)
    if (dragId.split('-').length - dropId.split('-').length > 0) {
      dropIdSave = (dropId + this.addZero(dragId.split('-').length - dropId.split('-').length)).split('-').join('')
      drapIdSave = dragId.split('-').join('')
    } else if (dragId.split('-').length - dropId.split('-').length < 0) {
      dropIdSave = (dropId + this.addZero(dropId.split('-').length - dragId.split('-').length)).split('-').join('')
      drapIdSave = dragId.split('-').join('')
    } else {
      drapIdSave = dragId.split('-').join('')
      dropIdSave = dropId.split('-').join('')
    }
    console.log('这是转换后的')
    console.error(drapIdSave)
    console.error(dropIdSave)
    let flag = true
    drapIdSave.split('')
    let arrtempdrag = drapIdSave.split('')
    let arrtempdrop = dropIdSave.split('')
    for (let index = 0; index < arrtempdrag.length; index++) {
      const element = arrtempdrag[index];
      
      if (+arrtempdrag[index] > +arrtempdrop[index]) {
        flag = true
        break;
      }
      if (+arrtempdrag[index] < +arrtempdrop[index]) {
        flag = false
        break;
      }
      
    }
    let drapIndex = dragId.substr(-1);
    console.log('这是flag')
    console.log(flag)
    if (flag) {
      this.ms.$dragSubject.next({
        type: 'delete_no_index',
        obj: {
          dragIndex: +drapIndex,
          dropIndex: index,
          item: this.ms.drapItem.item,
          dragId: dragId,
          dropId: dropId,
          transDragId: this.arrTrans(dragId) ? this.arrTrans(dragId) : '',
          transDropId: this.arrTrans(dropId) ? this.arrTrans(dropId) : '',
        },
      });
    } 
    else {
      this.ms.$dragSubject.next({
        type: 'delete_edit_index',
        obj: {
          dragIndex: +drapIndex,
          dropIndex: index,
          item: this.ms.drapItem.item,
          dragId: dragId,
          dropId: dropId,
          transDragId: this.arrTrans(dragId) ? this.arrTrans(dragId) : '',
          transDropId: this.arrTrans(dropId) ? this.arrTrans(dropId) : '',
        },
      });
    }

    // 如果移动的是后面的元素，则删除后面的元素，对前面的元素可以没有任何影响，则可以直接操作。
    
    return;
  }
  addZero(num: any) {
    let str = ''
    for (let index = 0; index < num; index++) {
      str += '0'
    }
    return str
  }
  arrTrans(str: any) {
    let StrTemp = str;
    let arr = StrTemp.split('-');
    arr.splice(arr.length - 1, 1);
    return arr.join('-');
  }
}
