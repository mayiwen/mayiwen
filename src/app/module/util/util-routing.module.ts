import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { UtilModuleUtilComponent } from './util.component';

const routes: Routes = [
  { path: '', component: UtilModuleUtilComponent, children: [
    {
      path: '',
      redirectTo: 'regExp',
      pathMatch: 'full'
    },
    {
      path: 'regExp',
      loadChildren: () => import('./module/regexp/regexp.module').then((x) => x.RegexpModule)
    },
    {
      path: 'reg-and-hex',
      loadChildren: () => import('./module/rgb-and-hex/rgb-and-hex.module').then((x) => x.RgbAndHexModule)
    },
    {
      path: 'video',
      loadChildren: () => import('./module/video/video.module').then((x) => x.VideoModule)
    },
  ]},
 
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class UtilRoutingModule { }
