import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { ReactiveFormsModule } from '@angular/forms';
import { RouterModule } from '@angular/router';
import { NuoruomaComponent } from './nuoruoma.component';
import { NuoruomaRoutingModule } from './nuoruoma-routing.module';
import { NuoruomaFont1500Component } from './component/font1500/font1500.component';
import { NuoruomaScrollComponent } from './component/scroll/scroll.component';
import { MywModule } from 'mayiwen_angular';
import { NuoruomaDesignComponent } from './component/design/design.component';
@NgModule({
  declarations: [
    NuoruomaComponent,
    NuoruomaFont1500Component,
    NuoruomaScrollComponent,
    NuoruomaDesignComponent
  ],
  imports: [
    CommonModule,
    FormsModule,
    ReactiveFormsModule,
    RouterModule,
    NuoruomaRoutingModule,
    MywModule
  ],
  exports: [NuoruomaComponent],
  bootstrap: [NuoruomaComponent
  ],
  providers: [],
})
export class NuoruomaModule { }
