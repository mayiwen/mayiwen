import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { ReactiveFormsModule } from '@angular/forms';
import { UtilRoutingModule } from './util-routing.module';
import { UtilModuleUtilComponent } from './util.component';
import { MywModule } from 'mayiwen_angular';
@NgModule({
  declarations: [
    UtilModuleUtilComponent
  ],
  imports: [
    CommonModule,
    FormsModule,
    ReactiveFormsModule,
    UtilRoutingModule,
    MywModule
  ],
  bootstrap: [
    UtilModuleUtilComponent
  ],
  providers: [],
})
export class UtilModule { }
