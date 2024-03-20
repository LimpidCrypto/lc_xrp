import { Component, Input, OnInit } from '@angular/core';

@Component({
  selector: 'modal-atom',
  templateUrl: './modal.component.html',
  styleUrls: ['./modal.component.scss']
})
export class ModalComponent implements OnInit {
  @Input() id!: string;
  @Input() closeOnClickOutside: boolean = true;
  private modal?: HTMLDialogElement;

  ngOnInit() {
    this.modal = document.querySelector(`#${this.id}`) as HTMLDialogElement;
    if (this.closeOnClickOutside) {
      if (this.modal) {
        this.modal.addEventListener('click', (event) => {
          if (event.target !== this.modal) {
            this.close();
          }
        });
      }
    }
  }

  public open() {
    if (this.modal) {
      this.modal.showModal()
    }
  }

  public close() {
    if (this.modal) {
      this.modal.close()
    }
  }
}