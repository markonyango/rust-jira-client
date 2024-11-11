import { Component, computed, input } from '@angular/core';
import { MatInputModule } from '@angular/material/input';
import { IssueListComponent } from '../../../shared/issue-list.component';

@Component({
  standalone: true,
  imports: [MatInputModule, IssueListComponent],
  selector: 'filter',
  templateUrl: './filter.component.html'
})
export class FilterComponent {
  public data = input.required<any>();
  public filter = computed(() => this.data().filter);
  public issues = computed(() => this.data().issues);
}
