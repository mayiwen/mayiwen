import {
  AfterViewInit,
  ChangeDetectorRef,
  Component,
  ElementRef,
  Injector,
  OnInit,
  ViewChild,
} from '@angular/core';
import { MywAlertService, MywMessageService } from 'mayiwen_angular';
// import { NzModalRef, NzModalService } from 'ng-zorro-antd/modal';

@Component({
  selector: 'app-test-1000',
  templateUrl: './test1000.component.html',
  styleUrls: ['./test1000.component.less'],
  providers: [],
})
export class Test1000Component implements AfterViewInit {
  @ViewChild('canvas') canvas!: ElementRef;
  message: string = 'message'
  index = 0
  constructor(cdr: ChangeDetectorRef, injector: Injector, public popup: MywMessageService, public alert: MywAlertService) {
    // const PopupElement = createCustomElement(MayiwenMessageComponent, {injector});
    // customElements.define('popup-element', PopupElement)
  }
  ngAfterViewInit(): void {
    console.log('这是打印的canvas');
    console.log(this.canvas.nativeElement);
    const canvas = this.canvas.nativeElement as any;
    canvas.style.height = 400;
    canvas.style.width = 400;
    const ctx = canvas.getContext('2d');
    ctx.moveTo(0,0);
    ctx.lineTo(30,30);
    ctx.moveTo(30,0);
    ctx.lineTo(0,30);
    ctx.stroke();
  }
  test() {
    this.popup.show( ++this.index + 'a受方酒受泛滥殊方a就殊则a受怎看受在a;受另在a受雷方a殊拉可酒殊则可酒殊则看受怎了看受怎刻酒受')
  }
  test2() {
    this.popup.show( ++this.index + '')
  }
  test3() {
    this.alert.show({
      message: '是否确认删除11111111111111111111111111111111111111111？',
      success: () => {
        this.popup.show('你删除了元素')
      }
    })
  }
}
