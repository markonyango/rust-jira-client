import { Component, input } from '@angular/core';
import { MatCardModule } from '@angular/material/card';
import { Issue } from '../../issue.type';

@Component({
  standalone: true,
  selector: 'issue-details',
  template: `
    <mat-card>
      <mat-card-header>
        <mat-card-title>Details</mat-card-title>
      </mat-card-header>
      <mat-card-content>
        <span
          >Typ: <img src="{{ issue().fields.issuetype.iconUrl }}" />
          {{ issue().fields.issuetype.name }}</span
        >
        <span>Priorität: {{ issue().fields.priority.name }}</span>
        <span>Stichwörter: {{ issue().fields.labels.join(', ') }}</span>
        <span>Lösung: {{ issue().fields.resolution }}</span>
      </mat-card-content>
    </mat-card>
  `,
  styles: [
    `
      mat-card-content {
        display: flex;
        flex-direction: column;
        gap: 1em;
      }
    `,
  ],
  imports: [MatCardModule],
})
export class DetailsComponent {
  public issue = input.required<Issue>();
}
