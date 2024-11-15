import { Component, input } from '@angular/core';
import { MatTableModule } from '@angular/material/table';
import { DatePipe } from '@angular/common';
import { RouterLink } from '@angular/router';
import { Issue } from '../../../pages/issues/issue.type';
import { FixVersionsComponent } from "./cell-renderer/fix-versions.component";
import { IssueTypeComponent } from "./cell-renderer/issue-type.component";

@Component({
  standalone: true,
  selector: 'issue-list',
  template: `
  <table mat-table [dataSource]="issues()">
    <ng-container matColumnDef="issuetype">
      <th mat-header-cell *matHeaderCellDef="">Typ</th>
      <td mat-cell *matCellDef="let issue"><issue-type-cell-renderer [issueType]="issue.fields.issuetype" /></td>
    </ng-container>
    <ng-container matColumnDef="key">
      <th mat-header-cell *matHeaderCellDef="">Schlüssel</th>
      <td mat-cell *matCellDef="let issue"><a routerLink="{{ '/issues/' + issue.key }}">{{ issue.key }}</a></td>
    </ng-container>
    <ng-container matColumnDef="summary">
      <th mat-header-cell *matHeaderCellDef="">Zusammenfassung</th>
      <td mat-cell *matCellDef="let issue">{{ issue.fields.summary }}</td>
    </ng-container>
    <ng-container matColumnDef="priority">
      <th mat-header-cell *matHeaderCellDef="">Priorität</th>
      <td mat-cell *matCellDef="let issue">{{ issue.fields.priority.name }}</td>
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
    <ng-container matColumnDef="fixVersions">
      <th mat-header-cell *matHeaderCellDef="">Lösungsversion(en)</th>
      <td mat-cell *matCellDef="let issue"><fix-versions-cell-renderer [fixVersions]="issue.fields.fixVersions" /></td>
    </ng-container>
    <ng-container matColumnDef="aggregateestimate">
      <th mat-header-cell *matHeaderCellDef="">Ursprüngliche Schätzung</th>
      <td mat-cell *matCellDef="let issue">{{ issue.fields.aggregatetimeestimate }}</td>
    </ng-container>
    <ng-container matColumnDef="aggregateprogress">
      <th mat-header-cell *matHeaderCellDef="">Verbleibende Schätzung</th>
      <td mat-cell *matCellDef="let issue">{{ issue.fields.aggregateprogress.total }}</td>
    </ng-container>
    <ng-container matColumnDef="timespent">
      <th mat-header-cell *matHeaderCellDef="">Benötigte Zeit</th>
      <td mat-cell *matCellDef="let issue">{{ issue.fields.timespent }}</td>
    </ng-container>
    
  
    <tr mat-header-row *matHeaderRowDef="displayedColumns"></tr>
    <tr mat-row *matRowDef="let row; columns: displayedColumns;"></tr>
  </table>
  `,
  imports: [MatTableModule, DatePipe, RouterLink, FixVersionsComponent, IssueTypeComponent]
})
export class IssueListComponent {
  public issues = input.required<Issue[]>();
  protected readonly displayedColumns = ['issuetype','key', 'summary', 'created', 'updated', 'priority', 'status', 'fixVersions', 'aggregateestimate', 'aggregateprogress', 'timespent'];
}
