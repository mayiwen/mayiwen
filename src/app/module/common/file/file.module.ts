import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { ReactiveFormsModule } from '@angular/forms';
// import { NzToolTipModule } from 'ng-zorro-antd/tooltip';
import { FileService } from './service/file.service'
@NgModule({
  declarations: [
  ],
  imports: [
    CommonModule,
    FormsModule,
    ReactiveFormsModule,
  ],
  bootstrap: [
  ],
  providers: [],
  exports: []
})
export class FileModule { }
