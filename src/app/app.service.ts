import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';

@Injectable()
export class AppService {
  public token = ''
  constructor(private http: HttpClient) { }
}
