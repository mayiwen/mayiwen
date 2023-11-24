import { AfterViewInit, ChangeDetectorRef, Component, OnDestroy, OnInit } from '@angular/core';
import { Subject, takeUntil } from 'rxjs';
import { ElectronService } from '../../core/services';
const packageJson = require('../../../../package.json');
@Component({
  selector: 'app-top',
  templateUrl: './top.component.html',
  styleUrls: ['./top.component.less']
})
export class TopComponent implements AfterViewInit, OnDestroy {
  isMax = false;
  version = ''; // 设置当前的版本
  private $destroy = new Subject();
  constructor(private electron: ElectronService, private cdr: ChangeDetectorRef,) { }
  ngAfterViewInit(): void {
    console.log('这是package.json');
    this.version = packageJson.version.replaceAll('"', '');
  }
  ngOnDestroy(): void {
    this.$destroy.next(null);
    this.$destroy.unsubscribe();
  }
  openF12() {
    this.electron.ipcRenderer.send('f12');
  }
  windowEdit(type: 'min' | 'max' | 'close' | 'f12') {
    this.electron.ipcRenderer.send(type);
  }
  changeTheme() {
    console.log(window.document.documentElement.getAttribute('data-myw-theme'))
    if (window.document.documentElement.getAttribute('data-myw-theme') === 'black') {
      window.document.documentElement.setAttribute('data-myw-theme', 'white');
    } else {
      window.document.documentElement.setAttribute('data-myw-theme', 'black');
    }
  }
}
