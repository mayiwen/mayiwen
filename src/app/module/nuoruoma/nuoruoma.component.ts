import {
  AfterViewInit,
  ChangeDetectorRef,
  Component,
  OnInit,
  ViewChild,
} from '@angular/core';
import { Router } from '@angular/router';
@Component({
  selector: 'app-nuoruoma',
  templateUrl: './nuoruoma.component.html',
  styleUrls: ['./nuoruoma.component.scss'],
})
export class NuoruomaComponent implements OnInit, AfterViewInit {

  constructor(private router: Router) {

  }
  ngAfterViewInit(): void {
    // throw new Error('Method not implemented.');
  }
  ngOnInit(): void {
    // throw new Error('Method not implemented.');
  }
}
