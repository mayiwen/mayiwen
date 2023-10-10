import {
  AfterViewInit,
  Component,
  OnInit,
} from '@angular/core';
import { URL } from 'url';
@Component({
  selector: 'app-video',
  templateUrl: './video.component.html',
  styleUrls: ['./video.component.less'],
  providers: [],
})
export class VideopComponent implements OnInit, AfterViewInit {
  src = ''
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
  dragenter(e: any) {
    e.preventDefault();
  }
  dragleave(e: any) {
    e.preventDefault();
  }
  dragover(e: any) {
    e.preventDefault();
  }
  async drop(e: any) {
    console.log(e)
    e.preventDefault();
    console.log(e.dataTransfer.files);
    let fileList: File[] = e.dataTransfer.files as File[];
    console.log(fileList[0])
    // this.src = URL.createObjectURL(fileList[0])
  }
}
