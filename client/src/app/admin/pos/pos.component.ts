import { Component, OnInit, HostListener } from '@angular/core';
import { Pager } from 'src/app/class/pager';
import { Profile } from 'src/app/class/profile';
import { HttpClient } from '@angular/common/http';
import { ActivatedRoute, Router } from '@angular/router';
import { Title } from '@angular/platform-browser';
import { Observable, from, interval, of, BehaviorSubject } from 'rxjs';
import { map } from 'rxjs/operators';

@Component({
  selector: 'app-user',
  templateUrl: './pos.component.html',
  styleUrls: ['./pos.component.css']
})
export class PosComponent implements OnInit {

  counter$: Observable<number>;
  customers$: Observable<Profile[]>;

  constructor(private http: HttpClient, private route: ActivatedRoute, private router: Router, private title: Title) {
    this.counter$ = interval(1000);
  }

  getCustomers(): Observable<Profile[]> {
    return this.http.get<Profile[]>("/user/all");
  }

  ngOnInit() {
    this.customers$ = this.getCustomers().pipe(map(val =>
      val.sort((a, b) => a.date_created > b.date_created ? 1 : -1)));
  }
}