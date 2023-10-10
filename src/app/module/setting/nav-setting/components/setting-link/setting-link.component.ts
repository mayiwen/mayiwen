import {
  AfterViewInit,
  ChangeDetectorRef,
  Component,
  OnInit,
  ViewChild,
  ElementRef,
} from '@angular/core';
import { FormBuilder, Validators } from '@angular/forms';
import { BackendNavService } from '../../../../../backend/nav';
import { NavSettingLinkService } from './service/nav-setting-link.service';
import { NavSettingTitleService } from '../setting-title/service/nav-setting-title.service';
import { MywAlertService, MywMessageService } from 'mayiwen_angular';

@Component({
  selector: 'app-setting-link',
  templateUrl: './setting-link.component.html',
  styleUrls: ['./setting-link.component.less'],
  providers: [BackendNavService, NavSettingLinkService, NavSettingTitleService],
})
export class SettingLinkComponent implements OnInit, AfterViewInit {
  @ViewChild('navRef') navRef!: ElementRef;
  addOrEditForm = this.fb.group({
    title: ['', Validators.required],
    url: ['', Validators.required],
    selectObj: this.fb.group({
      value: '',
      v: ''
    })
  });
  dropSaveIndex = 0;
  public tableDate = []; // 表格的数据
  public linkList = [] as any;
  public selectObj = {} as any;
  tableSetting = {
    tableHead: [
      {
        value: 'id',
        v: '序号',
        w: 100,
      },
      {
        value: 'title',
        v: '标题',
        w: 200,
      },
      {
        value: 'edit',
        v: '操作',
        w: 200,
        type: 'content',
        typeKey: '.hello',
      },
    ],
  };
  deleteObj = {} as any
  modal: boolean =  true // 弹出窗是否显示
  constructor(
    private cdr: ChangeDetectorRef,
    private fb: FormBuilder,
    private titleService: NavSettingTitleService,
    private linkService: NavSettingLinkService,
    private alert: MywAlertService,
    private message: MywMessageService,
    
  ) {}
  async ngOnInit() {
    this.getTableData();
  }
  ngAfterViewInit(): void {
    window.onresize = () => {};
  }
  getTableData() {
    this.titleService.listTitle().subscribe((res: any) => {
      this.tableDate = res.map((item: any) => { return {value: item.id, v: item.title}});
      console.log('这是初始化的tableData')
      console.log(this.tableDate)
      this.addOrEditForm.controls.selectObj.setValue({
       v: this.tableDate[0]['v'],
       value: this.tableDate[0]['value'],
      })
      this.cdr.detectChanges()
      console.log(this.addOrEditForm)
      const titleid = this.tableDate[0]['value'];
      this.getLinkById(titleid);
    });
  }
  getLinkById(titleid: any) {
    this.linkService.findOneLink(titleid).subscribe((res: any) => {
      console.log('这是打印的');
      console.log(res);
      this.linkList = res;
    });
  }
  changeSelect(e: any) {
    console.log('这是切换的');
    console.log(e);
    this.getLinkById(e.value);
  }
  dragStart(item: any, index: any, flag?: boolean) {
    console.log('drag start');
    console.log(item);
    console.log(index);
    this.dropSaveIndex = index;
    // this.drapType = 'm';
  }
  dragenter(e: any) {
    e.preventDefault();
  }
  dragleave(e: any) {
    e.preventDefault();
  }
  dragover(e: any) {
    e.preventDefault();
  }
  drop(e: any) {
    e.preventDefault();
    console.log('drop');
    // console.log(e.dataTransfer);
    // console.log(e.dataTransfer.files);
    // const files = [];
    // [].forEach.call(
    //   e.dataTransfer.files,
    //   function (file) {
    //     files.push(file);
    //   },
    //   false
    // );
    // console.log('这是文件');
    // console.log(files);
    // files.forEach(async (item) => {
    //   let dat = await imgIcon(item.path);
    //   item.img = dat.toDataURL();
    //   this.appList.push({
    //     /** 链接的名字，用于图标下的展示 */
    //     name: item.name,
    //     /** 链接的位置，用于点击的时候，打开的位置 */
    //     path: item.path,
    //     /** 图片的路径 */
    //     img: item.img,
    //     /** type 类型 m multiple，多个的，点击弹出单个路径  s single单个的，点击直接跳转 */
    //     type: 's',
    //   });
    //   this.localStorageSet();
    // });
  }
  dropInner(e: any, item: any, index: any) {
    e.preventDefault();
    console.log('这是拖进的');
    console.log(item, index);
    console.log('这是拖动的');
    console.log(this.dropSaveIndex);
    console.log('进入了1');
    console.log('dropInner');
    console.log(e, item, index);
    const itemSave = this.linkList[this.dropSaveIndex];
    this.linkList.splice(this.dropSaveIndex, 1);
    this.linkList.splice(index, 0, itemSave);
    let list = [...this.linkList].map((itemi: any, index) => {
      return {
        indexLink: index,
        titleId: itemi.title_id,
        title: itemi.title,
        src: itemi.src,
        id: itemi.id,
      };
    });
    this.linkService.updateLinkList(list).subscribe((res: any) => {
      console.log(res);
      this.getLinkById(this.selectObj.value);
    });
  }
  /**
   * 重置添加的数据
   */
  reset(){
    this.addOrEditForm.patchValue({
      title: '',
      url: ''
    })
  }
  /** 这是添加的方法 */
  add() {
    console.log(this.addOrEditForm);
    if(this.addOrEditForm.invalid) {
      return alert('不可为空。');
    }
    console.log(this.addOrEditForm.getRawValue())
    if (this.addOrEditForm && this.addOrEditForm.getRawValue() && this.addOrEditForm.getRawValue().selectObj && this.addOrEditForm.getRawValue().selectObj.value) {
      let a = this.addOrEditForm as any 
      let titleId =  +a.getRawValue().selectObj.value as any
      const params = {
        titleId,
        title: this.addOrEditForm.getRawValue().title,
        src: this.addOrEditForm.getRawValue().url,
        indexLink: 999999999
      }
      console.log(params);
      this.linkService.addLink(params).subscribe((res: any) => {
        this.getLinkById(this.addOrEditForm.getRawValue().selectObj.value);
      })
    }
   
  }

  delete(item: any) {
    console.log('这是待删除的对象');
    console.log(item);
    this.deleteObj = item
    // this.modalDelete = true;
    this.alert.show({
      title: '是否确认删除',
      message: item.src,
      success: () => {
        console.log('这是弹出的')
        this.linkService.deleteLink(this.deleteObj.id).subscribe(
          {
            next: (res: any) => {
              console.log('这是res打印的内容')
              console.log(res)
              this.getLinkById(this.addOrEditForm.getRawValue().selectObj.value);
            },
            error: (error: any) => {
              console.log('这是打印的error')
              console.log(error)
              this.message.show(error.message)
            } 
          }
        )
      }
    })
  }

  deleteClick() {
    
  }
  editBefore(e: any) {}

  optionSelect(e: any) {
    console.log('这是获取的option select 外部')
    console.log(e)
  }
  print() {
    console.log(this.addOrEditForm)
  }
}
