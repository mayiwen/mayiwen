import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ReactiveFormsModule, FormsModule } from '@angular/forms';
import { RgbAndHexComponent } from './rgb-and-hex.component';
import { RgbAndHexRoutingModule } from './rgb-and-hex-routing.module';
import { MywModule } from 'mayiwen_angular';

@NgModule({
  declarations: [
    RgbAndHexComponent
  ],
  imports: [
    CommonModule,
    ReactiveFormsModule,
    FormsModule,
    RgbAndHexRoutingModule,
    MywModule
  ],
  bootstrap: [
  ],
  providers: [],
})
export class RgbAndHexModule { }
