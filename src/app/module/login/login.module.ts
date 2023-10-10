import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';

import { LoginRoutingModule } from './login-routing.module';

import { LoginComponent } from './login.component';
import { LoginService } from './service/login.service';
import { ReactiveFormsModule } from '@angular/forms';
import { MywModule } from 'mayiwen_angular';

@NgModule({
  declarations: [LoginComponent],
  imports: [CommonModule, LoginRoutingModule,  ReactiveFormsModule, MywModule],
  providers: [LoginService]
})
export class LoginModule {}
