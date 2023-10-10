import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { NuoruomaComponent } from './nuoruoma.component';
const routes: Routes = [
  {
    path: '',
    component: NuoruomaComponent,
  },
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule],
})
export class NuoruomaRoutingModule {}
