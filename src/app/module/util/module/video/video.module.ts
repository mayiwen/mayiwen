import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ReactiveFormsModule, FormsModule } from '@angular/forms';
import { VideopComponent } from './video.component';
import { VideoRoutingModule } from './video-routing.module';
import { MywModule } from 'mayiwen_angular';

@NgModule({
  declarations: [
    VideopComponent
  ],
  imports: [
    CommonModule,
    ReactiveFormsModule,
    FormsModule,
    VideoRoutingModule,
    MywModule,
  ],
  bootstrap: [
  ],
  providers: [],
})
export class VideoModule { }
