import { AfterViewInit, ChangeDetectorRef, Component, OnInit } from '@angular/core';
import { Router, ActivatedRoute, ParamMap, NavigationEnd } from '@angular/router';
import { LeftLink } from '../o/left/left';

import { Location } from '@angular/common';
import { filter, Observable } from 'rxjs';
@Component({
  selector: 'app-left',
  templateUrl: './left.component.html',
  styleUrls: ['./left.component.scss']
})
export class LeftComponent implements OnInit, AfterViewInit {
  selectUrl = 'home'
  linkList: LeftLink[] = [
    {
      url: 'home',
      title: 'mayiwen.com',
      icon: 'icon-home'
    },
    {
      url: 'desktop',
      title: '我的桌面',
      icon: 'icon-desktop'
    },
    {
      url: 'mind-map',
      title: '思维导图',
      icon: 'icon-heart'
    },
    {
      url: 'util',
      title: '工具',
      icon: 'icon-code'
    },
    {
      url: 'nuoruoma',
      title: '诺若码',
      icon: 'icon-star'
    },
    {
      url: 'myw',
      title: '组件',
      icon: 'icon-wrench'
    },
    {
      url: 'nav-setting',
      title: '系统设置',
      icon: 'icon-setting'
    },
  
  ]
  constructor(private route: ActivatedRoute, private location: Location, private router: Router, private cdr: ChangeDetectorRef) { 
      (this.router.events.pipe(filter(event => event instanceof NavigationEnd)) as Observable<NavigationEnd>).subscribe(router => {     
      this.selectUrl = router.url.split('/')[1]
      })
    }
  ngAfterViewInit(): void {
    this.selectUrl = 'home'
    this.cdr.detectChanges()
  }
  name: any
  ngOnInit(): void {
    this.route.queryParams.subscribe(params => {
      console.log('这是路由的内容')
      this.name = params['name'];
    });
  }
}
