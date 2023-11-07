import { Injectable } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Observable } from 'rxjs';

@Injectable()
export class UpdateService {
  constructor(private http: HttpClient) {}
  getChildByPath(path: string): Observable<any> { return this.http.post('file/list', {path});}
  delete(path: string): Observable<any> { return this.http.post('file/delete', {path});}
  updateUpload(file: FormData): Observable<any> { 
    const headers = new HttpHeaders();
    headers.set('Content-Type', 'multipart/form-data')
    return this.http.post('update/upload', file, {headers});
  }
}
