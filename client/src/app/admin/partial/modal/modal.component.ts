import { Component, Input, OnInit, OnChanges, HostListener } from '@angular/core';
import { Model } from 'src/app/class/model';

@Component({
  selector: 'modal',
  templateUrl: './modal.component.html',
  styleUrls: ['./modal.component.scss']
})
export class ModalComponent implements OnChanges {
  // @Input() isLoading: boolean;
  // @Input() isOk: boolean;
  // @Input() model: Model<any>;
  // @Input() name?: string = 'Mentés';

  isActive: boolean = false;

  @HostListener('document:keydown.esc')
  close() {
    this.isActive = false;
  }

  open() {
    this.isActive = true;
  }

  ngOnChanges() { }
}
