import { Component, OnInit, HostListener } from '@angular/core';
import { Pager } from 'src/app/class/pager';
import { Profile } from 'src/app/class/profile';
import { HttpClient } from '@angular/common/http';
import { ActivatedRoute, Router } from '@angular/router';
import { Title } from '@angular/platform-browser';
import { Observable, from, interval, of, BehaviorSubject, Subject } from 'rxjs';
import { map } from 'rxjs/operators';

@Component({
  selector: 'app-user',
  templateUrl: './pos.component.html',
  styleUrls: ['./pos.component.scss']
})
export class PosComponent implements OnInit {

  constructor(private http: HttpClient, private route: ActivatedRoute, private router: Router, private title: Title) { }
  ngOnInit() { }

  search: string = '';
  life: number = 42;

  message(msg: string) {
    alert(msg);
  }

  demo = () => {
    alert('Search is ' + this.search);
  }
}