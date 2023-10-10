import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { ReactiveFormsModule } from '@angular/forms';
import { NavSettingRoutingModule } from './nav-setting-routing.mdule';
import { NavSettingComponent } from './nav-setting.component';
import { SettingTitleComponent } from './components/setting-title/setting-title.component';
import { SettingLinkComponent } from './components/setting-link/setting-link.component';
// import { NzToolTipModule } from 'ng-zorro-antd/tooltip';
import { Test1000Component } from './components/test1000/setting-title/test1000.component';
import { AppCardComponent } from './components/test1000/app-card/app-card.component';
import { AboutComponent } from './components/about/about.component';
import { RouterModule } from '@angular/router';
import { LoginModule } from '../../login/login.module';
import { UpdateComponent } from './components/update/update.component';
import { MywModule } from 'mayiwen_angular';
@NgModule({
  declarations: [
    NavSettingComponent,
    SettingTitleComponent,
    SettingLinkComponent,
    Test1000Component,
    AppCardComponent,
    AboutComponent,
    UpdateComponent
  ],
  imports: [
    CommonModule,
    FormsModule,
    ReactiveFormsModule,
    NavSettingRoutingModule,
    MywModule,
    RouterModule,
    LoginModule,
    
  ],
  bootstrap: [
    NavSettingComponent
  ],
  providers: [],
})
export class NavSettingModule { }
