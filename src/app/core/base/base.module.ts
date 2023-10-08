import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { TopComponent } from './top/top.component';
import { LeftComponent } from './left/left.component';
import { BaseModuleLeftSvgComponent } from './left/component/svg/app-left-svg/svg.component';
import { MywModule } from 'mayiwen_angular';
const components = [BaseModuleLeftSvgComponent, TopComponent, LeftComponent];
@NgModule({
  imports: [CommonModule, MywModule],
  declarations: [...components],
  exports: [...components]
})
export class BaseModule { }
