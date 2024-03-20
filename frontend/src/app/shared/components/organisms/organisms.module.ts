import { FooterComponent } from './core/footer/footer.component';
import { HeaderComponent } from './core/header/header.component';
import { ImprintComponent } from './imprint/imprint.component';
import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MoleculesModule } from '../molecules/molecules.module';
import { AtomsModule } from '../atoms/atoms.module';
import { ApiDocumentationComponent } from './api-documentation/api-documentation.component';


@NgModule({
  declarations: [
    FooterComponent,
    HeaderComponent,
    ImprintComponent,
    ApiDocumentationComponent,
  ],
  imports: [
    CommonModule,
    AtomsModule,
    MoleculesModule
  ],
  exports: [
    FooterComponent,
    HeaderComponent,
    ImprintComponent,
    ApiDocumentationComponent,
  ]
})
export class OrganismsModule { }
