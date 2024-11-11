import { Component, input } from '@angular/core';
import { MatTableModule } from '@angular/material/table';
import { DatePipe } from '@angular/common';
import { RouterLink } from '@angular/router';
import { Issue } from '../pages/issues/issue.type';

@Component({
  standalone: true,
  selector: 'issue-list',
  template: `
  <table mat-table [dataSource]="issues()">
    <ng-container matColumnDef="key">
      <th mat-header-cell *matHeaderCellDef="">Schlüssel</th>
      <td mat-cell *matCellDef="let issue"><a routerLink="{{ '/issues/' + issue.key }}">{{ issue.key }}</a></td>
    </ng-container>
    <ng-container matColumnDef="summary">
      <th mat-header-cell *matHeaderCellDef="">Zusammenfassung</th>
      <td mat-cell *matCellDef="let issue">{{ issue.fields.summary }}</td>
    </ng-container>
    <ng-container matColumnDef="status">
      <th mat-header-cell *matHeaderCellDef="">Status</th>
      <td mat-cell *matCellDef="let issue">{{ issue.fields.status.name }}</td>
    </ng-container>
    <ng-container matColumnDef="created">
      <th mat-header-cell *matHeaderCellDef="">Erstellt</th>
      <td mat-cell *matCellDef="let issue">{{ issue.fields.created | date:'d.MM.YYYY' }}</td>
    </ng-container>
    <ng-container matColumnDef="updated">
      <th mat-header-cell *matHeaderCellDef="">Aktualisiert</th>
      <td mat-cell *matCellDef="let issue">{{ issue.fields.updated | date:'d.MM.YYYY' }}</td>
    </ng-container>
    
  
    <tr mat-header-row *matHeaderRowDef="displayedColumns"></tr>
    <tr mat-row *matRowDef="let row; columns: displayedColumns;"></tr>
  </table>
  `,
  imports: [MatTableModule, DatePipe, RouterLink]
})
export class IssueListComponent {
  public issues = input.required<Issue[]>();
  protected readonly displayedColumns = ['key', 'summary', 'status', 'created', 'updated'];
}
