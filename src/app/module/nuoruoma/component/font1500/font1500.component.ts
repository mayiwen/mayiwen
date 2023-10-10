import {
  AfterViewInit,
  ChangeDetectorRef,
  Component,
  OnInit,
  ViewChild,
} from '@angular/core';
import { Router } from '@angular/router';
import { arr } from './mock/data.ts/arr';
import { MywMessageService } from 'mayiwen_angular';
@Component({
  selector: 'app-nuoruoma-font1500',
  templateUrl: './font1500.component.html',
  styleUrls: ['./font1500.component.scss'],
})
export class NuoruomaFont1500Component implements OnInit, AfterViewInit {
  tableHead = [
    {
      value: 'font',
      v: '字',
      w: 100,
    },
    {
      value: 'index',
      v: '字频',
      w: 140,
    },
    {
      value: 'm1',
      v: '1码',
      w: 50,
    },
    {
      value: 'm1',
      v: '2码',
      w: 50,
    }, 
    {
      value: 'm1',
      v: '3码',
      w: 50,
    },
    {
      value: 'm1',
      v: '4码',
      w: 50,
    },
    {
      value: 'm1',
      v: '5码',
      w: 50,
    },
    {
      value: 'm1',
      v: '字根',
      w: 50,
    },
    {
      value: 'm1',
      v: '笔画码',
      w: 80,
    },
    {
      value: 'show2',
      v: '显示',
      w: 80,
    },
  ]
  arr = [] as any
  page = {
    total: 0,
    page: 1,
    pageSize: 50
  }
  constructor(private router: Router, private message: MywMessageService) {
    this.pageChange()
  }
  pageChange() {
    console.log('这是打印的arr')
    console.log((this.page.page - 1) * this.page.pageSize)
    console.log(this.page.pageSize)
    this.arr = arr.slice((this.page.page - 1) * this.page.pageSize , this.page.page * this.page.pageSize)
    this.page.total = arr.length
    console.log(this.arr)
  }
  changePageFn(e: any) {
    this.page.page = e
    this.page = {...this.page}
    this.pageChange()

  }
  ngAfterViewInit(): void {
    // throw new Error('Method not implemented.');
  }
  ngOnInit(): void {
    // throw new Error('Method not implemented.');
  }
  showtest(item: any) {
    this.message.show(item.index + '' + item.font)
  }
}
