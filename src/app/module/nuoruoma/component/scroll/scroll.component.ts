import {
  AfterViewInit,
  ChangeDetectorRef,
  Component,
  ElementRef,
  OnInit,
  ViewChild,
  ViewChildren,
} from '@angular/core';
import { Router } from '@angular/router';
@Component({
  selector: 'app-nuoruoma-scroll',
  templateUrl: './scroll.component.html',
  styleUrls: ['./scroll.component.scss'],
})
export class NuoruomaScrollComponent implements OnInit, AfterViewInit {
  arr = [
    { value: 0, show: false },
    { value: 1, show: false },
    { value: 2, show: false },
    { value: 3, show: false },
    { value: 4, show: false },
    { value: 5, show: false },
    { value: 6, show: false },
    { value: 7, show: false },
    { value: 8, show: false },
    { value: 9, show: false },
    { value: 10, show: false },
    { value: 11, show: false },
    { value: 12, show: false },
    { value: 13, show: false },
    { value: 14, show: false },
    { value: 15, show: false },
  ];

  @ViewChildren('hello') list!: ElementRef[];
  constructor(private router: Router, private cdr: ChangeDetectorRef) {}
  ngAfterViewInit() {
    console.log('这是after view init');
    console.log(this.list);
    // throw new Error('Method not implemented.');
    //io 为 IntersectionObserver对象 - 由IntersectionObserver()构造器创建
    var io = new IntersectionObserver((entries) => {
      //entries 为 IntersectionObserverEntry对像数组
      entries.forEach((item) => {
        //item 为 IntersectionObserverEntry对像
        // isIntersecting是一个Boolean值，判断目标元素当前是否可见
        console.log('--------开始-----------')
        console.log(item)
        // if (item.isIntersecting) {
        //   console.log('可以看到的');
        //   console.log(item)
        //   console.log(item.target)
        //   console.log(item.target.attributes);
        //   console.log(item.target.attributes['data-index']);
        //   console.log(item.target.attributes['data-index']['value']);
        //   let index = +item.target.attributes['data-index']['value']

        //   this.arr[index].show = true;
        // } else {
        //   console.log('不可以看到的');
        //   console.log(item)
        //   console.log(item.target)
        //   console.log(item.target.attributes);
        //   console.log(item.target.attributes['data-index']);
        //   console.log(item.target.attributes['data-index']['value']);
        //   let index = +item.target.attributes['data-index']['value']
        //   this.arr[index].show = false;
        // }
        this.cdr.detectChanges();
        console.log('--------------结束')
      });
    }); //不传options参数，默认根元素为浏览器视口
    console.log('这是this.list');
    console.log(this.list);
    this.list.forEach((item) => {
      console.log(item);
    });
    console.log('-----------');

    console.log(this.list.map((item) => item.nativeElement));
    this.list
      .map((item) => item.nativeElement)
      .forEach((div) => io.observe(div)); // 遍历监听所有div DOM节点
  }
  ngOnInit(): void {
    // throw new Error('Method not implemented.');
  }
}
