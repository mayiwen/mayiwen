import {
  AfterViewInit,
  ChangeDetectorRef,
  Component,
  OnInit,
  ViewChild,
} from '@angular/core';
import { Router } from '@angular/router';
import { ElectronService } from '../../core/services';
@Component({
  selector: 'app-module-util',
  templateUrl: './util.component.html',
  styleUrls: ['./util.component.less'],
})
export class UtilModuleUtilComponent implements OnInit, AfterViewInit {
  constructor(private router: Router) {

  }
  ngAfterViewInit(): void {

  }
  ngOnInit(): void {
    this.hello(0)
  }
  hello(e: any) {
    if (e === 0) {
      this.router.navigate(['/util/regExp'])
    }
    if (e === 1) {
      // alert('rga与十六进制')
      // regExp
      // this.router.navigate(['/regExp/'],{queryParams:{id:key}})
      console.log(this.router)
      this.router.navigate(['/util/reg-and-hex'])
    }
    if (e === 2) {
      // alert('rga与十六进制')
      // regExp
      // this.router.navigate(['/regExp/'],{queryParams:{id:key}})
      console.log(this.router)
      this.router.navigate(['/util/video'])
    }
  }
  select() {
    alert('1111')
  }
}
