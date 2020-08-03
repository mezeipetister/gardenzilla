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
  isSearchUser: boolean = false;

  egCustomers: Pager<any> = new Pager([
    { name: 'Example 1', address: '4522 Nyírtass, Alkotmány út 55.', taxnumber: '2346543-1-23' },
    { name: 'Example 2', address: '4522 Nyírtass, Alkotmány út 55.', taxnumber: '2346543-1-23' },
    { name: 'Example 3', address: '4522 Nyírtass, Alkotmány út 55.', taxnumber: '2346543-1-23' },
    { name: 'Example 4', address: '4522 Nyírtass, Alkotmány út 55.', taxnumber: '2346543-1-23' },
    { name: 'Example 5', address: '4522 Nyírtass, Alkotmány út 55.', taxnumber: '2346543-1-23' },
    { name: 'Example 6', address: '4522 Nyírtass, Alkotmány út 55.', taxnumber: '2346543-1-23' },
    { name: 'Example 7', address: '4522 Nyírtass, Alkotmány út 55.', taxnumber: '2346543-1-23' },
    { name: 'Example 8', address: '4522 Nyírtass, Alkotmány út 55.', taxnumber: '2346543-1-23' },
    { name: 'Example 9', address: '4522 Nyírtass, Alkotmány út 55.', taxnumber: '2346543-1-23' },
    { name: 'Example 10', address: '4522 Nyírtass, Alkotmány út 55.', taxnumber: '2346543-1-23' },
    { name: 'Example 11', address: '4522 Nyírtass, Alkotmány út 55.', taxnumber: '2346543-1-23' },
    { name: 'Example 12', address: '4522 Nyírtass, Alkotmány út 55.', taxnumber: '2346543-1-23' },
    { name: 'Example 13', address: '4522 Nyírtass, Alkotmány út 55.', taxnumber: '2346543-1-23' },
    { name: 'Example 14', address: '4522 Nyírtass, Alkotmány út 55.', taxnumber: '2346543-1-23' },
    { name: 'Example 15', address: '4522 Nyírtass, Alkotmány út 55.', taxnumber: '2346543-1-23' },
    { name: 'Example 16', address: '4522 Nyírtass, Alkotmány út 55.', taxnumber: '2346543-1-23' },
    { name: 'Example 17', address: '4522 Nyírtass, Alkotmány út 55.', taxnumber: '2346543-1-23' },
    { name: 'Example 18', address: '4522 Nyírtass, Alkotmány út 55.', taxnumber: '2346543-1-23' },
    { name: 'Example 19', address: '4522 Nyírtass, Alkotmány út 55.', taxnumber: '2346543-1-23' },
  ]);

  egProducts: Pager<any> = new Pager([
    { name: 'Bordóilé neo SC', quantity: 250, unit: 'ml', net_retail_price: 3900.0, stock_ok: 3, stock_injured: 1, stock_global: 0 },
    { name: 'Bordóilé neo SC', quantity: 1, unit: 'l', net_retail_price: 1500.12, stock_ok: 24, stock_injured: 0, stock_global: 0 },
    { name: 'Bordóilé neo SC', quantity: 5, unit: 'l', net_retail_price: 9500.0, stock_ok: 0, stock_injured: 1, stock_global: 0 },
    { name: 'Bordóilé neo SC', quantity: 20, unit: 'l', net_retail_price: 32900.0, stock_ok: 0, stock_injured: 1, stock_global: 5 },
    { name: 'Gardena kerti locsoló + tömlő', quantity: 25, unit: 'm', net_retail_price: 11700.27, stock_ok: 1, stock_injured: 0, stock_global: 0 },
  ]);

  egUpls: any[] = [
    { id: 1231234, sku_name: 'Kerti lombseprű', quantity: 1, unit: 'kg', gross_retail_price: 7900.12, best_before: '2022-04-15' },
    { id: 1231234, sku_name: 'Kerti lombseprű', quantity: 1, unit: 'kg', gross_retail_price: 7900.12, best_before: '2022-04-15' },
    { id: 1231234, sku_name: 'Kerti lombseprű', quantity: 1, unit: 'kg', gross_retail_price: 7900.12, best_before: '2022-04-15' },
    { id: 1231234, sku_name: 'Kerti lombseprű', quantity: 1, unit: 'kg', gross_retail_price: 7900.12, best_before: '2022-04-15' },
    { id: 1231234, sku_name: 'Kerti lombseprű', quantity: 1, unit: 'kg', gross_retail_price: 7900.12, best_before: '2022-04-15' },
    { id: 1231234, sku_name: 'Kerti lombseprű', quantity: 1, unit: 'kg', gross_retail_price: 7900.12, best_before: '2022-04-15' },
    { id: 1231234, sku_name: 'Kerti lombseprű', quantity: 1, unit: 'kg', gross_retail_price: 7900.12, best_before: '2022-04-15' },
    { id: 1231234, sku_name: 'Kerti lombseprű', quantity: 1, unit: 'kg', gross_retail_price: 7900.12, best_before: '2022-04-15' },
    { id: 1231234, sku_name: 'Kerti lombseprű', quantity: 1, unit: 'kg', gross_retail_price: 7900.12, best_before: '2022-04-15' }
  ];

  message(msg: string) {
    alert(msg);
  }

  checkSearchType() {
    if (this.search.charAt(0) == '@') {
      this.isSearchUser = true;
    } else {
      this.isSearchUser = false;
    }
  }

  clearSearch() {
    this.search = '';
  }

  demo = () => {
    alert('Search is ' + this.search);
  }
}