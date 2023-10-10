import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ReactiveFormsModule, FormsModule } from '@angular/forms';
import { TreeMapComponent } from './treemap.component';
import { MywModule } from 'mayiwen_angular';
@NgModule({
  declarations: [
    TreeMapComponent
  ],
  imports: [
    CommonModule,
    ReactiveFormsModule,
    FormsModule,
    MywModule
  ],
  exports: [
    TreeMapComponent
  ],
  bootstrap: [
    TreeMapComponent
  ],
  providers: [],
})
export class TreeMapModule { }
