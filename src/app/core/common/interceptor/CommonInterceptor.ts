import { Injectable } from '@angular/core';
import {
  HttpInterceptor,
  HttpEvent,
  HttpHandler,
  HttpRequest,
  HttpResponse,
} from '@angular/common/http';
import { Observable } from 'rxjs';
import { AppService } from '../../../app.service';
@Injectable()
export class CommonInterceptor implements HttpInterceptor {
  constructor(private appService: AppService) { }
  intercept(
    req: HttpRequest<any>,
    next: HttpHandler
  ): Observable<HttpEvent<any>> {
    let url = `http://localhost:3000/${req.url}`
    // let url = `http://121.4.117.203:3000/${req.url}`
    let apiReq;
    if (this.appService.token) {
      apiReq = req.clone({
        url,
        headers: req.headers.set('Authorization', 'Bearer ' + this.appService.token),
      });
    } else {
      apiReq = req.clone({ url });
    }
    // const apiReq = req.clone({ url: `http://121.4.117.203:3000/${req.url}` });
    return next.handle(apiReq);
  }
}
