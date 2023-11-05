import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';

@Injectable()
export class UpdateService {
  constructor(private http: HttpClient) {}
  getChildByPath(path: string): Observable<any> { return this.http.post('file/list', {path});}
  delete(path: string): Observable<any> { return this.http.post('file/delete', {path});}
}
