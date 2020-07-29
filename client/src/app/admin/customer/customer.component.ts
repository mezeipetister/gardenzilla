import { Component, OnInit } from '@angular/core';
import { Pager } from 'src/app/class/pager';
import { Profile } from 'src/app/class/profile';
import { HttpClient } from '@angular/common/http';

@Component({
  selector: 'app-user',
  templateUrl: './customer.component.html',
  styleUrls: ['./customer.component.css']
})
export class CustomerComponent implements OnInit {
  filter: string = "";
  users: Pager<Profile> = new Pager([], 10);
  buffer: Profile[] = null;

  filterSubmit() {
    /**
     * Filter data
     */
    this.users.data = this.buffer.filter((c) =>
      c.name.toUpperCase().includes(this.filter.toUpperCase()));
    /**
     * Reset pagination
     */
    this.users.navigate_to(1);
  }

  constructor(private http: HttpClient) { }

  ngOnInit() {
    this.http.get<Profile[]>("/user/all").subscribe((val) => {
      val = val.sort((a, b) => a.date_created > b.date_created ? 1 : -1);
      this.users.set_data(val);
      this.buffer = val;
    });
  }
}
