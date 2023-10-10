import {
  AfterViewInit,
  ChangeDetectorRef,
  Component,
  OnInit,
  ViewChild,
} from '@angular/core';
import { FormBuilder, Validators } from '@angular/forms';
import { createFalse } from 'typescript/lib/tsserverlibrary';
import { BackendNavService } from '../../../../../backend/nav';
import { NavSettingTitleService } from './service/nav-setting-title.service';
import { TitleDto, UpdateTitleDto } from './dto/title.dto'
import { MywAlertService, MywMessageService } from 'mayiwen_angular';
// import { NzModalRef, NzModalService } from 'ng-zorro-antd/modal';

@Component({
  selector: 'app-setting-title',
  templateUrl: './setting-title.component.html',
  styleUrls: ['./setting-title.component.less'],
  providers: [BackendNavService, NavSettingTitleService],
})
export class SettingTitleComponent implements OnInit, AfterViewInit {
  modal = false;
  modalDelete = false;
  myContext = { $implicit: 'World', localSk: 'Svet' };
  deleteSave = {} as any;
  title = 'mayiwen';
  tableHead = [
    {
      value: 'id',
      v: '序号',
      w: 100,
    },
    {
      value: 'editRef',
      v: '操作',
      w: 140,
    },
    {
      value: 'title',
      v: '标题',
      w: 200,
    },
  ]

  addOrEditForm = this.fb.group({
    id: [{value:'', disabled: true}],
    title: ['', Validators.required],
  });
  modalAddFlag = false;
  modalupdateFlag = false;
  public alertTitle = '';
  public alertTitleUpdate = '';
  public alertTitleid: any;
  // private confirmModal;
  public tableDate = []; // 表格的数据
  /**
   * 是否是添加 true的时候是添加，false的时候是修改
   */
  flagAddOrUpdate = true;

  constructor(
    // private backendNavService: BackendNavService,
    private cdr: ChangeDetectorRef,
    private fb: FormBuilder,
    private message: MywMessageService,
    private alert: MywAlertService,
    private service: NavSettingTitleService
  ) {}
  ngAfterViewInit(): void {
    // throw new Error('Method not implemented.');
  }
  async ngOnInit() {
    this.getTableData();
  }
  /** 列表表格数据 */
  getTableData() {
    this.service.listTitle().subscribe((res) => {
      this.tableDate = res;
    });
  }

  edit1(item: any) {
    console.log(item);
    this.modal = true;
    this.flagAddOrUpdate = false;
    this.addOrEditForm.patchValue({
      id: item.id,
      title: item.title,
    });
  }
  delete(item: any) {
    console.log(item);
    this.alert.show({
      title: '是否确认删除',
      message: `当前的id：“${item.id}”，当前的title：“${item.title}”`,
      success: () => {
        this.service
        .deleteTitle(item.id)
        .subscribe((res) => {
          console.log(res);
          this.getTableData();
        });
      }
    })
    
  }
  close() {
    this.modal = false;
  }
  /** 添加之前 */
  addBefore() {
    this.modal = true;
    this.flagAddOrUpdate = true;
    this.addOrEditForm.patchValue({
      id: null,
      title: ''
    })
  }
  addItem() {
    let a  = this.addOrEditForm as any
    if (!a.getRawValue().title.trim()) {
      this.message.show('标题不可为空')
      return;
    }
    let title = new TitleDto() ;
    title.title = a.getRawValue().title.trim()


    this.service
      .addTitle(title)
      .subscribe((res) => {
        console.log(res);
        if (res && res.title) {
          this.message.show('添加 ' + res.title + ' 成功！')
        } else {
          this.message.show('添加失败！')
        }
        this.getTableData();
        this.modal = false;
      });
  }

  editItem() {
    if(this.addOrEditForm.invalid) {
      this.message.show('不可为空')
    }
    let updateTitleDto = new UpdateTitleDto()
    let a = this.addOrEditForm as any
    updateTitleDto.id = +a.getRawValue().id
    updateTitleDto.title = a.getRawValue().title
    this.service
      .updateTitle(updateTitleDto.id, updateTitleDto)
      .subscribe((res) => {
        console.log(res);
        if (res && res.affected && res.affected > 0) {
          this.message.show('修改成功')
        }
        this.getTableData();
        this.modal = false;
      });
  }
}
