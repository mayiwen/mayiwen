import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';

@Injectable()
export class NavSettingLinkService {
  constructor(private http: HttpClient) {}
  listLink(): Observable<any> { return this.http.get('nav/link');}
  findOneLink(id: any): Observable<any> { return this.http.get('nav/link/' + id);}
  addLink(params: any):  Observable<any> { return this.http.post('nav/link', params);}
  deleteLink(id: any): Observable<any> { return this.http.delete('nav/link/' + id);}
  updateLink(id: any, params: any): Observable<any> { return this.http.patch('nav/link/' + id, params);}
  updateLinkList(params: any): Observable<any> { return this.http.post('nav/link/update_list', params);}
}
