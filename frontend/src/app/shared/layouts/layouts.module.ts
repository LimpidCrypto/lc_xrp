import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MasterLayout } from './master/master.component';
import { OrganismsModule } from '../components/organisms/organisms.module';



@NgModule({
  declarations: [
    MasterLayout
  ],
  imports: [
    CommonModule,
    OrganismsModule
  ],
  exports: [
    MasterLayout
  ]
})
export class LayoutsModule { }
