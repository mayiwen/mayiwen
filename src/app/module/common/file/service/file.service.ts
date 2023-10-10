import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { ElectronService } from '../../../../core/services';
@Injectable()
export class FileService {
  constructor(private http: HttpClient, private es: ElectronService) {}
  /** 获取桌面的路径 */
  pathDesktopString() {
    return this.es.pathDesktopString;
  }
  /** 获取用户文件夹 */
  pathUserDataString() {
    return this.es.pathUserDataString;
  }

  clearEditor() {
    let path = this.es.path.join(this.es.pathUserDataString, 'editor');
    this.deleteDir(path);
  }

  emptyDir(path: any) {
    const files = this.es.fs.readdirSync(path);
    console.log('这是files');
    console.log(files);
    files.forEach((file) => {
      console.log(file);
      const filePath = `${path}/${file}`;
      const stats = this.es.fs.statSync(filePath);
      if (stats.isDirectory()) {
        this.emptyDir(filePath);
      } else {
        this.es.fs.unlinkSync(filePath);
        // console.log(`删除${file}文件成功`);
      }
    });
  }
  deleteDir(url: any) {
    var files = [];
    if (this.es.fs.existsSync(url)) {
      //判断给定的路径是否存在
      files = this.es.fs.readdirSync(url); //返回文件和子目录的数组
      files.forEach( (file, index) => {
        var curPath = this.es.path.join(url, file);
        if (this.es.fs.statSync(curPath).isDirectory()) {
          //同步读取文件夹文件，如果是文件夹，则函数回调
          this.deleteDir(curPath);
        } else {
          this.es.fs.unlinkSync(curPath); //是指定文件，则删除
        }
      });
      this.es.fs.rmdirSync(url); //清除文件夹
    } else {
      console.log('给定的路径不存在！');
    }
  }
  // 如果没有这个文件夹要创建这个文件夹
  pathUserDataMayiwen() {
    let mayiwenPath = this.es.path.join(this.es.pathUserDataString);
    if (!this.es.fs.existsSync(this.es.pathUserDataString)) {
      this.es.fs.mkdirSync(this.es.pathUserDataString);
    }
  }
  // 如果没有这个文件夹要创建这个文件夹
  pathUserDataMayiwenEditor() {
    this.pathUserDataMayiwen();
    let mayiwenEditorPath = this.es.path.join(
      this.es.pathUserDataString,
      'editor'
    );
    if (!this.es.fs.existsSync(mayiwenEditorPath)) {
      this.es.fs.mkdirSync(mayiwenEditorPath);
    }
  }
  pathUserDataMayiwenEditorSingle(str: any) {
    let singlePath = this.es.path.join(
      this.es.pathUserDataString,
      'editor',
      str
    );
    if (!this.es.fs.existsSync(singlePath)) {
      this.es.fs.mkdirSync(singlePath);
    }
    return singlePath
  }
}
