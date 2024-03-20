import { Component, Input } from '@angular/core';

export type ButtonType = 'button' | 'submit' | 'reset';

@Component({
  selector: 'button-atom',
  templateUrl: './button.component.html',
  styleUrls: ['./button.component.scss']
})
export class ButtonComponent {
  @Input() type: ButtonType = 'button';
  @Input() disabled: boolean = false;
}