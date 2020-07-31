import { Component, OnInit, Input } from '@angular/core';
import { Pager } from 'src/app/class/pager';

@Component({
  selector: 'pagernav',
  templateUrl: './pagernav.component.html',
  styleUrls: ['./pagernav.component.scss']
})
export class PagernavComponent implements OnInit {

  @Input() data: Pager<any>;

  constructor() { }

  ngOnInit() { }

}
