import { Component, input } from '@angular/core';

@Component({
  standalone: true,
  selector: 'issue-type-cell-renderer',
  template: '<img [src]="issueType().iconUrl" /> {{ issueType().name }}'
})
export class IssueTypeComponent {
  public issueType = input.required<any>();
}
