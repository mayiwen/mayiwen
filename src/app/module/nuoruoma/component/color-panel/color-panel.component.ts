import { AfterViewInit, Component, OnInit } from '@angular/core';
@Component({
  selector: 'app-color-panel',
  templateUrl: './color-panel.component.html',
  styleUrls: ['./color-panel.component.scss'],
})
export class ColorPanelComponent implements OnInit, AfterViewInit {
  arr = [
    '0',
    '1',
    '2',
    '3',
    '4',
    '5',
    '6',
    '7',
    '8',
    '9',
    'a',
    'b',
    'c',
    'd',
    'e',
    'f',
  ];
  arr2 = [];
  arr3 = [] as any;
  arr4 = [] as any;
  ngOnInit() {
    let arr1: any = [];
    for (let index = 0; index < this.arr.length; index++) {
      console.log('打印' + index);
      for (let j = 0; j < this.arr.length; j++) {
        arr1.push(this.arr[index] + '' + this.arr[j]);
      }
    }
    console.log(arr1);

    let arr2 = arr1.map((item: any) => {
      return item + item + item;
    });
    console.log(arr2);

    this.arr2 = arr2;

    let arr3: any[] = [];
    let arr4: any[] = [];
    arr2.forEach((item: string, index: number) => {
      if (index < arr2.length / 2) {
        arr3.push(item);
      } else {
        arr4.unshift(item);
      }
    });
    console.log('arr3 arr4');
    console.log(arr3);
    console.log(arr4);
    this.arr3 = arr3;
    this.arr4 = arr4;
  }
  ngAfterViewInit(): void {
    // throw new Error('Method not implemented.');
  }
}
