import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { RegexpComponent } from './regexp.component'

const routes: Routes = [
  {path: '', component: RegexpComponent}
  
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class RegexpRoutingModule { }
