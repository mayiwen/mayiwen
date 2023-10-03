import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { TopComponent } from './top/top.component';
import { LeftComponent } from './left/left.component';
import { RouterModule } from '@angular/router';
import { BaseModuleLeftSvgComponent } from './left/component/svg/app-left-svg/svg.component';
import { MywModule } from 'mayiwen_angular';
const privateComponent = [BaseModuleLeftSvgComponent]
const components = [TopComponent, LeftComponent];
@NgModule({
  imports: [CommonModule, RouterModule, MywModule],
  declarations: [...privateComponent, ...components],
  exports: [...components]
})
export class BaseModule { }
