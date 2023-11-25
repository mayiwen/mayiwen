import { Component, OnInit } from '@angular/core';
import { ElectronService } from './core/services';
import { TranslateService } from '@ngx-translate/core';
import { APP_CONFIG } from '../environments/environment';
@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent implements OnInit {
  listeners: any
  constructor(
    private electronService: ElectronService,
    private translate: TranslateService
  ) {
    // this.translate.setDefaultLang('en');
    console.log('APP_CONFIG', APP_CONFIG);

    if (electronService.isElectron) {
      console.log(process.env);
      console.log('Run in electron');
      console.log('Electron ipcRenderer', this.electronService.ipcRenderer);
      console.log('NodeJS childProcess', this.electronService.childProcess);
    } else {
      console.log('Run in browser');
    }
  }
  ngOnInit(): void {
    this.listeners={
      dark:(mediaQueryList: any )=>{
        if(mediaQueryList.matches){
          console.log('您切换到深色模式了！')
          window.document.documentElement.setAttribute('data-myw-theme', 'black');
        }
      },
      light:(mediaQueryList: any)=>{
        if(mediaQueryList.matches){
          console.log('您切换到浅色模式了！')
          window.document.documentElement.setAttribute('data-myw-theme', 'white');

        }
      }
    }
    if (window.matchMedia('(prefers-color-scheme)').media === 'not all') {
      console.log('Browser doesn\'t support dark mode');
    } else {
      if(window.matchMedia('(prefers-color-scheme: dark)').matches){
        window.document.documentElement.setAttribute('data-myw-theme', 'black');
      } else {
        window.document.documentElement.setAttribute('data-myw-theme', 'white');
      }
    }
    window.matchMedia('(prefers-color-scheme: dark)').addListener(this.listeners.dark)
    window.matchMedia('(prefers-color-scheme: light)').addListener(this.listeners.light)

  }
}
