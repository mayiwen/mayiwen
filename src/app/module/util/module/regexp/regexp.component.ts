import {
  AfterViewInit,
  Component,
  OnInit,
} from '@angular/core';
@Component({
  selector: 'app-regexp',
  templateUrl: './regexp.component.html',
  styleUrls: ['./regexp.component.less'],
  providers: [],
})
export class RegexpComponent implements OnInit, AfterViewInit {
  reg = ''
  v = ''
  res = ''
  error = ''
  ngAfterViewInit(): void {
    // throw new Error('Method not implemented.');
  }
  ngOnInit(): void {
    // throw new Error('Method not implemented.');
  }
  check() {
    console.log('这是check')
    this.error = ''
    try {
      let reg = eval(this.reg)
      let res = reg.test(this.v);
      if (res) {
        this.res = '通过';
      } else {
        this.res = '不通过'
      }
    } catch (error) {
      console.error(error)
      this.error = '不符合正则'
      this.res = ''
    }
  }
  change() {
    this.error = ''
    this.check()
  }
  reset() {
    this.res = ''
    this.error = ''
    this.v = ''
    this.reg = ''
  }
}
