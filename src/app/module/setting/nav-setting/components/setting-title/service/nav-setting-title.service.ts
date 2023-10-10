import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';

@Injectable()
export class NavSettingTitleService {
  constructor(private http: HttpClient) {}
  listTitle(): Observable<any> { return this.http.get('nav/title');}
  addTitle(params: any):  Observable<any> { return this.http.post('nav/title', params);}
  deleteTitle(id: any): Observable<any> { return this.http.delete('nav/title/' + id);}
  updateTitle(id: any, params: any): Observable<any> { return this.http.patch('nav/title/' + id, params);}
}
