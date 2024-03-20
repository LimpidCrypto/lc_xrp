import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { LogoComponent } from './logo/logo.component';
import { AtomsModule } from '../atoms/atoms.module';
import { TitleComponent } from './title/title.component';
import { AddressComponent } from './address/address.component';



@NgModule({
  declarations: [
    LogoComponent,
    TitleComponent,
    AddressComponent,
  ],
  imports: [
    CommonModule,
    AtomsModule,
  ],
  exports: [
    LogoComponent,
    TitleComponent,
    AddressComponent,
  ]
})
export class MoleculesModule { }
