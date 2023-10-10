import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { ReactiveFormsModule } from '@angular/forms';
import { DesktopRoutingModule } from './desktop-routing.mdule';
import { DesktopComponent } from './desktop.component';
import { MywModule } from 'mayiwen_angular';
@NgModule({
  declarations: [
    DesktopComponent
  ],
  imports: [
    CommonModule,
    FormsModule,
    ReactiveFormsModule,
    DesktopRoutingModule,
    MywModule,
  ],
  bootstrap: [
  ],
  providers: [],
})
export class DesktopModule { }
