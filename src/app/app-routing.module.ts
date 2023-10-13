import { NgModule } from '@angular/core';
import { Routes, RouterModule } from '@angular/router';
import { PageNotFoundComponent } from './shared/components';

import { HomeRoutingModule } from './home/home-routing.module';
import { MywModule } from 'mayiwen_angular';

const routes: Routes = [
  {
    path: '',
    redirectTo: 'home',
    pathMatch: 'full'
  },
  {
    path: 'nav-setting',
    loadChildren: () => import('./module/setting/nav-setting/nav-setting.module').then((x) => x.NavSettingModule),
  },
  {
    path: 'desktop',
    loadChildren: () => import('./module/desktop/desktop.module').then((x) => x.DesktopModule)
  },
  {
    path: 'mind-map',
    loadChildren: () => import('./module/mind-map/mind-map.module').then((x) => x.MindMapModule)
  },
  {
    path: 'util',
    loadChildren: () => import('./module/util/util.module').then((x) => x.UtilModule)
  },
  {
    path: 'nuoruoma',
    loadChildren: () => import('./module/nuoruoma/nuoruoma.module').then((x) => x.NuoruomaModule)
  },
  {
    path: '**',
    component: PageNotFoundComponent
  }
];

@NgModule({
  imports: [
    RouterModule.forRoot(routes, {}),
    HomeRoutingModule,
    // DetailRoutingModule
  ],
  exports: [RouterModule]
})
export class AppRoutingModule { }
