import { ComponentFixture, TestBed } from '@angular/core/testing';

import { Rotate180Component } from './rotate180.component';

describe('Rotate180Component', () => {
  let component: Rotate180Component;
  let fixture: ComponentFixture<Rotate180Component>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [Rotate180Component]
    })
    .compileComponents();
    
    fixture = TestBed.createComponent(Rotate180Component);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
