import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { ReactiveFormsModule } from '@angular/forms';
import { MindMapComponent } from './mind-map.component';
import { MindMapRoutingModule } from './mind-map-routing.module';
import { WangEditorComponent } from './editor/wangeditor/wangeditor.component';
import { TreeMapModule } from './module/treemap/treemap.module';
import { DownloadTemplateMindMapComponent } from './component/download-template.component'
import { MywModule } from 'mayiwen_angular';
@NgModule({
  declarations: [
    MindMapComponent,
    WangEditorComponent,
    DownloadTemplateMindMapComponent
  ],
  imports: [
    CommonModule,
    FormsModule,
    ReactiveFormsModule,
    MindMapRoutingModule,
    MywModule,
    TreeMapModule
  ],
  bootstrap: [
    MindMapComponent
  ],
  providers: [],
})
export class MindMapModule { }
