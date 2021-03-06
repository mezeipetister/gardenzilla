import { async, ComponentFixture, TestBed } from '@angular/core/testing';

import { RepositoryNewComponent } from './repository-new.component';

describe('RepositoryNewComponent', () => {
  let component: RepositoryNewComponent;
  let fixture: ComponentFixture<RepositoryNewComponent>;

  beforeEach(async(() => {
    TestBed.configureTestingModule({
      declarations: [ RepositoryNewComponent ]
    })
    .compileComponents();
  }));

  beforeEach(() => {
    fixture = TestBed.createComponent(RepositoryNewComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
