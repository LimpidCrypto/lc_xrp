import { Component, Input } from '@angular/core';

@Component({
  selector: 'background-image-atom',
  templateUrl: './background-image.component.html',
  styleUrls: ['./background-image.component.scss']
})
export class BackgroundImageComponent {
  @Input() imgSrc!: string;
  @Input() imgAlt!: string;
}