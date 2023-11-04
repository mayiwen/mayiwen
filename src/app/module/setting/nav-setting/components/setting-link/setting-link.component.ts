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
  mywForm = this.fb.group({
    id: ['', Validators.required],
    title_id: [0, Validators.required],
    title: ['', Validators.required],
    src: ['', Validators.required],
    indexLink: [0, Validators.required],
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

  tableCol = [
    {
      v: '序号',
      value: 'id',
      w: 100
    },
    {
      v: '操作',
      value: 'edit',
      w: 120
    },
    {
      value: 'title_id',
      v: '标题id',
      w: 60,
    },
    {
      value: 'title',
      v: '标题',
      w: 200,
    },
    {
      value: 'src',
      v: '链接地址',
      w: 500,
    },

  ]
  tableData = []
  deleteObj = {} as any
  modal: boolean = false // 弹出窗是否显示
  constructor(
    private cdr: ChangeDetectorRef,
    private fb: FormBuilder,
    private titleService: NavSettingTitleService,
    private linkService: NavSettingLinkService,
    private alert: MywAlertService,
    private message: MywMessageService,

  ) { }
  async ngOnInit() {
    this.getTableData();
  }
  ngAfterViewInit(): void {
    window.onresize = () => { };
  }
  getTableData() {
    console.log('这是打印的内容')
    this.titleService.listTitle().subscribe((res: any) => {
      console.log('这是打印的res')
      console.log(res)
      this.tableDate = res.map((item: any) => { return { value: item.id, v: item.title } });

      console.log('这是初始化的tableData')
      console.log(this.tableDate)
      // this.addOrEditForm.controls.selectObj.patchValue({
      //  v: this.tableDate[0]['v'],
      //  value: this.tableDate[0]['value'],
      // })
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
      this.tableData = res
    });
  }
  changeSelect(e: any) {
    console.log('这是切换的');
    console.log(e);
    this.getLinkById(e.value);
  }

  /**
   * 重置添加的数据
   */
  reset() {
    this.addOrEditForm.patchValue({
      title: '',
      url: ''
    })
  }
  /** 这是添加的方法 */
  add() {
    console.log(this.addOrEditForm);
    if (this.addOrEditForm.invalid) {
      return alert('不可为空。');
    }
    console.log(this.addOrEditForm.getRawValue())
    if (this.addOrEditForm && this.addOrEditForm.getRawValue() && this.addOrEditForm.getRawValue().selectObj && this.addOrEditForm.getRawValue().selectObj.value) {
      let a = this.addOrEditForm as any
      let titleId = +a.getRawValue().selectObj.value as any
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

  deleteFn(item: any) {
    console.log('这是待删除的对象');
    console.log(item);
    this.deleteObj = item
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
  /** 点击修改按钮执行的方法 */
  editBefore(e: any) {
    console.log('调用了')
    console.log(e)
    this.modal = true
    this.mywForm.setValue({
      id: e.id,
      title_id: e.title_id,
      title: e.title,
      src: e.src,
      indexLink: e.index_link,
    })
    console.log(this.mywForm)
  }
  /** 在修改弹窗点击确认执行的方法 */
  editClick() {
    console.log(this.mywForm)

    this.linkService.updateLink(this.mywForm.getRawValue().id, {
      title: this.mywForm.getRawValue().title,
      titleId: this.mywForm.getRawValue().title_id,
      src: this.mywForm.getRawValue().src,
      indexLink: this.mywForm.getRawValue().indexLink
    }).subscribe({
      next: data => {
        console.log(data)
        if (data && data.affected) {
          this.message.show('操作成功')
          this.modal = false
          this.getLinkById(this.mywForm.getRawValue().title_id)
          return
        }
        this.message.show('操作失败')

      },
      error: error => {
        console.log(error)
        this.message.show('操作异常')
      }
    })
  }

  optionSelect(e: any) {
    console.log('这是获取的option select 外部')
    console.log(e)
  }
  print() {
    console.log(this.addOrEditForm)
  }
  dragafter(e: any) {
    console.log('拖动的执行了')
    console.log(e)
    this.dropInner(e.dragIndex, e.dropIndex)
  }
  dropInner(dragIndex: number, dropIndex: number) {
    console.log('进入了里面')
    console.log(dragIndex)
    console.log(dropIndex)
    const itemSave = this.tableData[dragIndex];
    this.tableData.splice(dragIndex, 1);
    this.tableData.splice(dropIndex, 0, itemSave);
    console.log('这是打印的 list之前')
    let list = [...this.tableData].map((itemi: any, index) => {
      return {
        indexLink: index,
        titleId: itemi.title_id,
        title: itemi.title,
        src: itemi.src,
        id: itemi.id,
      };
    });
    console.log(list)
    this.linkService.updateLinkList(list).subscribe((res: any) => {
      console.log(res);
      this.getLinkById(this.selectObj.value);
    });
  }

  show() {
    console.log(this.addOrEditForm)
  }
}
