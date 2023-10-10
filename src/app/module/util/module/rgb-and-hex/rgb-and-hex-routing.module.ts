import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { RgbAndHexComponent } from './rgb-and-hex.component'

const routes: Routes = [
  {path: '', component: RgbAndHexComponent}
  
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class RgbAndHexRoutingModule { }
