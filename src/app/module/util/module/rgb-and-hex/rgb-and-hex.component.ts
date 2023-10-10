import { AfterViewInit, Component, OnInit } from '@angular/core';
import { MywMessageService } from 'mayiwen_angular';
@Component({
  selector: 'app-regexp',
  templateUrl: './rgb-and-hex.component.html',
  styleUrls: ['./rgb-and-hex.component.less'],
  providers: [],
})
export class RgbAndHexComponent implements OnInit, AfterViewInit {
  r = '0';
  g = '0';
  b = '0';
  rth: any;
  rgbstr: any;
  str: any;
  constructor(private message: MywMessageService){}
  ngAfterViewInit(): void {
    // throw new Error('Method not implemented.');
  }
  ngOnInit(): void {
    // throw new Error('Method not implemented.');
    console.log('这是打印的颜色');
  }

  rgbToHex() {
    console.log('这是转换的内容。')
    if (!this.r || !this.g || !this.b) {
      this.message.show('每个输入框要为 0 到 255 的数字，不可以为空。')
      return false;
    }
    try {
      let r = this.r
      let g = this.g;
      let b = this.b;
      let rs = r.length > 1 ? r : '0' + r;
      let gs = g.length > 1 ? g : '0' + g;
      let bs = b.length > 1 ? b : '0' + b;
      let rx = (+rs)
      let gx = (+gs)
      let bx = (+bs)
      let rStr = rx > 15 ? rx.toString(16) : '0' + rx.toString(16)
      let gStr = gx > 15 ? gx.toString(16) : '0' + gx.toString(16)
      let bStr = bx > 15 ? bx.toString(16) : '0' + bx.toString(16)
      console.log('这是计算的内容')
      this.rth = '#' + rStr + gStr + bStr
      // this.rth = '#' + (+rs).toString(16): number + (+gs).toString(16) + (+bs).toString(16);
    } catch (error) {
      this.message.show('出现了错误')
      console.error(error);
    }
  }
  hexToRgb() {
    if (this.str && this.str.length !== 6) {
      this.message.show('输入6位的 0 到 9，或 a 到 f')
      return false
    }
    try {
      let arr = this.str.split('');
      console.log(arr);
      let r = '0x' + arr[0] + arr[1];
      let g = '0x' + arr[2] + arr[3];
      let b = '0x' + arr[4] + arr[5];
      console.log(r, g, b);
      this.rgbstr =
        'rgb(' + Number(r) + ',' + Number(g) + ',' + Number(b) + ')';
    } catch (error) {
      this.message.show('出现了错误')
      console.error(error);
    }
  }
}
