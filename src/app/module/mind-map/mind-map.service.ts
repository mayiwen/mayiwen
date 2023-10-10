import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Subject } from 'rxjs';
import { DropInterface } from './module/treemap/o/tree-item.i';

@Injectable()
export class MindMapService {
  subject: Subject<any> = new Subject()
  $editSubject: Subject<any> = new Subject()
  $dragSubject: Subject<any> = new Subject()

  drapItem!: DropInterface
  /** 杀死递归 */
  flagKillLoop = true;
  constructor(private http: HttpClient) { }
}
