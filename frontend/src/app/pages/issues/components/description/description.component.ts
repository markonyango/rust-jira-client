import { Component, input } from '@angular/core';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';

@Component({
  standalone: true,
  selector: 'issue-description',
  template: `
    <mat-form-field>
      <textarea matInput>{{ description() }}</textarea>
    </mat-form-field>
  `,
  imports: [MatFormFieldModule, MatInputModule]
})
export class DescriptionComponent {
  public description = input.required<string>();
}
