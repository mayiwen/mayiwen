import {
  AfterViewInit,
  ChangeDetectorRef,
  Component,
  OnInit,
  ViewChild,
} from '@angular/core';
import { Router } from '@angular/router';
import { NavSettingService } from './nav-setting.service';
@Component({
  selector: 'app-nav-setting',
  templateUrl: './nav-setting.component.html',
  styleUrls: ['./nav-setting.component.less'],
  providers: [NavSettingService],
})
export class NavSettingComponent implements OnInit, AfterViewInit {
  tabIndex = 1
  constructor(private router: Router) {

  }

  selectTab = '1';
  ngAfterViewInit(): void {
    // throw new Error('Method not implemented.');
    this.selectTab = '2'
  }
  ngOnInit(): void {
    // throw new Error('Method not implemented.');
  }
  toroute() {
    console.log('这打印了')
    // this.router.navigate(['login'])
    console.log(this.router)
  }
  changeTabOutput(e: any) {
    console.log(e)
  }
  test() {
    console.log('调用了test的方法')
    alert('调用了test的方法')
  }
  tabClick(e: any) {
    console.log('这是tabClick')
    console.log(e)
  }
}
