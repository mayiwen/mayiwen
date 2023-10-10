import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { NavSettingComponent } from './nav-setting.component';
const routes: Routes = [
  {
    path: '',
    component: NavSettingComponent,
    children: [
      {
        path: '',
        redirectTo: 'login',
        pathMatch: 'full',
      },
      {
        path: 'login',
        loadChildren: () =>
          import('../../../module/login/login.module').then(
            (x) => x.LoginModule
          ),
      },
    ],
  },
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule],
})
export class NavSettingRoutingModule {}
// { path: '', component: UtilModuleUtilComponent, children: [
//   {
//     path: '',
//     redirectTo: 'regExp',
//     pathMatch: 'full'
//   },
//   {
//     path: 'regExp',
//     loadChildren: () => import('./module/regexp/regexp.module').then((x) => x.RegexpModule)
//   },
// ]},
