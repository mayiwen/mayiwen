import {
  AfterViewInit,
  ChangeDetectorRef,
  Component,
  OnInit,
  ViewChild,
} from '@angular/core';
import { ElectronService } from '../../../core/services';
import { timeStamp } from 'console';
@Component({
  selector: 'app-mind-map-download-template',
  templateUrl: './download-template.component.html',
  styleUrls: ['./download-template.component.less'],
  providers: [],
})
export class DownloadTemplateMindMapComponent implements OnInit, AfterViewInit {
  src = ''
  constructor(private cdr: ChangeDetectorRef) {
    
  }
  ngOnInit(): void {
  }
  ngAfterViewInit(): void {
    this.src = 'assets/icons/yiwei.png'
    this.cdr.detectChanges()
    
  }
}
