import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ReactiveFormsModule, FormsModule } from '@angular/forms';
import { RegexpRoutingModule } from './regexp-routing.module';
import { RegexpComponent } from './regexp.component'
import { MywModule } from 'mayiwen_angular';

@NgModule({
  declarations: [
    RegexpComponent
  ],
  imports: [
    CommonModule,
    ReactiveFormsModule,
    FormsModule,
    RegexpRoutingModule,
    MywModule,
  ],
  bootstrap: [
  ],
  providers: [],
})
export class RegexpModule { }
