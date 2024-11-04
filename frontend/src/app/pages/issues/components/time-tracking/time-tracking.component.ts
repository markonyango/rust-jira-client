import { Component, input } from '@angular/core';
import { MatCardModule } from '@angular/material/card';

@Component({
  standalone: true,
  selector: 'time-tracking',
  template: `
    <mat-card>
      <mat-card-header
        ><mat-card-title>Zeitverfolgung</mat-card-title></mat-card-header
      >
      <mat-card-content>
        <span>Original Geschätzte: {{ time_tracking().originalEstimate }}</span>
        <span>Verbleibende: {{ time_tracking().remainingEstimate }}</span>
        <span>Protokolliert: {{ time_tracking().timeSpent }} </span>
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
export class TimeTrackingComponent {
  public time_tracking = input.required<any>();
  // "originalEstimateSeconds": 115200,
  // "remainingEstimateSeconds": 113400,
  // "timeSpentSeconds": 1800
}
