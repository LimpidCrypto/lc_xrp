import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ComponentsModule } from './components/components.module';
import { LayoutsModule } from './layouts/layouts.module';



@NgModule({
  declarations: [],
  imports: [
    CommonModule,
    ComponentsModule,
    LayoutsModule
  ],
  exports: [
    ComponentsModule,
    LayoutsModule
  ]
})
export class SharedModule { }
