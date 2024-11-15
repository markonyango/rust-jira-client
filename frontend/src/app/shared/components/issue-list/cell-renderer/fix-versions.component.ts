import { NgFor, NgIf } from '@angular/common';
import { Component, effect, input } from '@angular/core';
import { MatIconModule } from '@angular/material/icon';

@Component({
  standalone: true,
  selector: 'fix-versions-cell-renderer',
  template: '<ng-container *ngFor="let version of fixVersions()"><div><span>{{ version.name }}</span><mat-icon *ngIf="version.released">check_circle_outline</mat-icon></div></ng-container>',
  imports: [NgFor, NgIf, MatIconModule]
})
export class FixVersionsComponent {
  public fixVersions = input.required<any[]>();
}
