import { Component, Input, ViewEncapsulation } from '@angular/core';

@Component({
  selector: 'image-atom',
  templateUrl: './image.component.html',
  styleUrls: ['./image.component.scss'],
  encapsulation: ViewEncapsulation.None,
})
export class ImageComponent {
  @Input() src!: string;
  @Input() alt!: string;
}
