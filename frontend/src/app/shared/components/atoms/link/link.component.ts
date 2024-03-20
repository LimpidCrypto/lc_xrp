import { Component, Input } from '@angular/core';

export type LinkTarget = '_blank' | '_self' | '_parent' | '_top' | string;

@Component({
  selector: 'link-atom',
  templateUrl: './link.component.html',
  styleUrls: ['./link.component.scss']
})
export class LinkComponent {
  @Input() href!: string;
  @Input() target: LinkTarget = '_self'
}