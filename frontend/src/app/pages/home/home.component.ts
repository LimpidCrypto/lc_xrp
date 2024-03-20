import { Component } from '@angular/core';
import { SharedModule } from '../../shared/shared.module';
import { MoleculesModule } from "../../shared/components/molecules/molecules.module";

@Component({
  selector: 'home-page',
  standalone: true,
  templateUrl: './home.component.html',
  styleUrl: './home.component.scss',
  imports: [
    SharedModule,
    MoleculesModule
  ]
})
export class HomePage {

}
