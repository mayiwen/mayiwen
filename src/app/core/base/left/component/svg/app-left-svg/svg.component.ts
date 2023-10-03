import { AfterViewInit, Component, ElementRef, Input, ViewChild } from '@angular/core';
import { LeftLink } from '../../../../o/left/left';
@Component({
  selector: 'app-base-left-svg',
  templateUrl: './svg.component.html',
  styleUrls: ['./svg.component.scss']
})
export class BaseModuleLeftSvgComponent implements AfterViewInit {
  @ViewChild('a') a!: ElementRef
  @Input('selectUrl') set url(e: any) {
    if (e) {
      this.selectUrl = e
    } else {
      this.selectUrl = 'home'
    }

  }
  @Input('link') link!: LeftLink

  selectUrl: string = ''
  get eauile() {
    return this.selectUrl === this.link.url
  }
  ngAfterViewChecked() {
  }
  constructor() {
  }

  ngAfterViewInit(): void {
  }
  ngOnInit(): void {
  }

}
