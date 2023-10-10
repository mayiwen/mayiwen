import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';
@Injectable()
export class BackendNavService {
  constructor(private http: HttpClient) { }
  linkTitle(): Observable<any> { return this.http.get('link'); }
  linkTitlePost(title: any): Observable<any> { return this.http.post('link', { title }); };
  linkTitleDelete(id: any): Observable<any> { return this.http.delete('link/' + id,); };
  linkTitlePacch(id: any, title: any): Observable<any> { return this.http.patch('link/' + id, { title }); };
  linkLink(id: any): Observable<any> { return this.http.get('link/link?id=' + id); }
  linkLinkSave(data: any): Observable<any> { return this.http.post('link/link', data); }
  linkLinkDeleteById(id: any): Observable<any> { return this.http.delete('link/link/' + id); }
  patchLinkAll(list: any): Observable<any> { return this.http.post('link/link/all', list); };
}

