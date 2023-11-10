import { AfterViewInit, ChangeDetectorRef, Component, OnInit, ViewChild } from '@angular/core';
import { ElementRef } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { LoginService } from './service/login.service';
import { FormBuilder, Validators } from '@angular/forms';
import { AppService } from '../../app.service';
import { MywMessageService } from 'mayiwen_angular';
@Component({
  selector: 'app-module-login-component',
  templateUrl: './login.component.html',
  styleUrls: ['./login.component.scss'],
  providers: []
})
export class LoginComponent implements OnInit, AfterViewInit {
  constructor(
   private service: LoginService,
   public appService: AppService,
   private message: MywMessageService,
   private fb: FormBuilder,
  ) {
  }
  type:string = 'password'
  addOrEditForm = this.fb.group({
    name: ['mayiwen', Validators.required],
    password: ['MA@yiwen', Validators.required],
    // code: ['', Validators.required],
  });
  ngAfterViewInit(): void {
  }
  ngOnInit(): void {
  }
  login() {
    if (this.addOrEditForm.invalid) {
      this.message.show('未填写完整')
      return false;
    }
    console.log('调用了登录的方法')
    this.message.show('调用了登录的方法')
    console.log(this.addOrEditForm)
    let params = {
      username: this.addOrEditForm.getRawValue().name,
      password: this.addOrEditForm.getRawValue().password
    }
    this.service.login(params).subscribe({
      next: dat => {
        console.log(dat)
        if (dat && dat.access_token) {
          this.appService.token = dat.access_token
          this.message.show('登录成功！')
        } else {
          this.message.show('登录失败！')
        }
      },
      error: err => {
        console.error(err)
        this.message.show(err.message)
      }
    })
  }
  
}
