import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { VideopComponent } from './video.component';

const routes: Routes = [
  {path: '', component: VideopComponent}
  
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class VideoRoutingModule { }
