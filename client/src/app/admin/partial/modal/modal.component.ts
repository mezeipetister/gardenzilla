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
  @Input() title?: string = '';
  @Input() hasCancel?: boolean = false;

  @Input() callbackSubmit?: [string, Function];

  isActive: boolean = false;

  @HostListener('document:keydown.esc')
  close() {
    this.isActive = false;
  }

  open() {
    this.isActive = true;
  }

  clickSave() {
    if (this.callbackSubmit) {
      if (this.callbackSubmit[1]) {
        this.callbackSubmit[1]();
      }
    }
  }

  ngOnChanges() { }
}
