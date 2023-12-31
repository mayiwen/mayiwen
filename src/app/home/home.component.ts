import { AfterViewInit, ChangeDetectorRef, Component, OnInit, ViewChild } from '@angular/core';
import { ElementRef } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { ElectronService } from '../core/services';
@Component({
  selector: 'app-nav',
  templateUrl: './home.component.html',
  styleUrls: ['./home.component.scss'],
  providers: []
})
export class HomeComponent implements OnInit, AfterViewInit {

  public navList = []; // 所有的列表
  public activeIndex = 0; // 当前的下标
  public searchE = 'baidu'; // 搜索引擎
  public searchText = ''; // 搜索引擎
  public liWidth = '100%';
  public titleList = [] as any;
  public linkList = [] as any;
  @ViewChild('navRef') navRef!: ElementRef;
  @ViewChild('searchRef') searchRef!: ElementRef;
  constructor(
    private cdr: ChangeDetectorRef,
    private http: HttpClient,
    private electron: ElectronService,
  ) {
    console.log(this.searchE);
  }
  get liWidthComputer() {
    return this.liWidth;
  }
  get activeNavList() {
    return this.navList && this.navList.length > 0 ? this.navList[this.activeIndex]['list'] : [];
  }
  async ngOnInit() {
    console.log('获取后端的代码');
    // this.navService.backendTitle();
    await this.getTitle();
  }
  async getTitle() {
    try {
      console.log('开始获取')
      const title = await this.http.get('nav/title');
      console.log('获取后的内容')
      console.log(title);
      title.subscribe({next:res => {
        if (Array.isArray(res) && res.length > 0) {
          console.log(res);
          this.titleList = res as any;
          this.getLinkById(res[0].id);
        }
      } });
    } catch (err) {
      console.log(err);
    }
  }
  getLinkById(id: any) {
    this.http.get('nav/link/' + id, {}).subscribe(inner => {
      console.log('这是里面打印的');
      console.log(inner);
      this.linkList = inner as any;
    });
  }
  ngAfterViewInit() {
    this.resize();
    window.onresize = () => {
      this.resize();
    };
    this.searchRef.nativeElement.focus()
  }
  resize() {
    console.log('窗口的大小打开了');
    console.log('这是界面完成调用的');
    console.log(this.navRef.nativeElement.getBoundingClientRect().width);
    if (this.navRef.nativeElement.getBoundingClientRect().width < 200) {
      this.liWidth = '100%';
    } else if (this.navRef.nativeElement.getBoundingClientRect().width < 300) {
      this.liWidth = '50%';
    } else if (this.navRef.nativeElement.getBoundingClientRect().width < 600) {
      this.liWidth = '33.33%';
    } else if (this.navRef.nativeElement.getBoundingClientRect().width < 750) {
      this.liWidth = '25%';
    } else if (this.navRef.nativeElement.getBoundingClientRect().width < 850) {
      this.liWidth = '20%';
    } else if (this.navRef.nativeElement.getBoundingClientRect().width < 1000) {
      this.liWidth = '16.66%';
    } else {
      this.liWidth = '14.28%';
    }
    console.log(this.liWidth);
    console.log('---------');
    this.cdr.markForCheck();
    this.cdr.detectChanges();
  }

  search() {
    console.log(this.searchText);
    let searchUrl = '';
    const searchKeyValue = this.searchText;
    switch (this.searchE) {
      case 'baidu':
        searchUrl = 'http://www.baidu.com/s?' + 'wd=' + searchKeyValue;
        break;
      case 'sogou':
        searchUrl = 'http://www.sogou.com/web' + '?' + 'query' + '=' + searchKeyValue;
        break;
      case 'bing':
        searchUrl = 'https://cn.bing.com/search' + '?' + 'q' + '=' + searchKeyValue;
        break;
      case '360':
        searchUrl = 'https://www.so.com/s' + '?' + 'q' + '=' + searchKeyValue;
        break;
      case 'google':
        searchUrl = 'http://www.google.com/search?hl=en' + '?' + 'q' + '=' + searchKeyValue;
        break;
      case 'baiduImg':
        searchUrl = 'http://image.baidu.com/i?tn=baiduimage&ct=201326592&cl=2&lm=-1&fr=&pv=&ic=0&z=0&word=' + searchKeyValue + '&s=0';
        break;
    }
    this.electron.shell.openExternal(searchUrl);
  }
  sizeChange(e: any) {
    console.log('大小改变了');
    console.log(e);
  }
  openOnBrowser(e: any) {
    console.log('这是打印的')
    console.log(e)
    console.log(this.electron.shell)
    this.electron.shell.openExternal(e);
  }

  getLink(id:any) {
    this.getLinkById(id);
  }
}
