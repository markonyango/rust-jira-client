import { Component, inject } from '@angular/core';
import { IssuesService } from './issues.service';
import { AsyncPipe, DatePipe, NgFor, NgIf } from '@angular/common';
import { FormControl, FormGroup, ReactiveFormsModule } from '@angular/forms';
import { IssueDetailsComponent } from './components/issue-details/issue-details.component';
import { MatInputModule } from '@angular/material/input';
import { MatButtonModule } from '@angular/material/button';
import { MatTableModule } from '@angular/material/table';
import { RouterLink } from '@angular/router';
import { IssueListComponent } from "../../shared/components/issue-list/issue-list.component";

@Component({
  standalone: true,
  selector: 'issues',
  templateUrl: './issues.component.html',
  imports: [
    NgFor,
    NgIf,
    ReactiveFormsModule,
    DatePipe,
    AsyncPipe,
    IssueDetailsComponent,
    MatInputModule,
    MatButtonModule,
    MatTableModule,
    RouterLink,
    IssueListComponent
],
})
export class IssuesComponent {
  public service = inject(IssuesService);

  public issues$ = inject(IssuesService).issues$;

  public displayedColumns = ['key', 'summary', 'status', 'created', 'updated'];

  protected formGroup = new FormGroup({
    query: new FormControl('', { nonNullable: true }),
  });

  public search() {
    this.service.search(this.formGroup.controls.query.value);
  }
}
