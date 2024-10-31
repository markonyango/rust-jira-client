import { Component, inject } from '@angular/core';
import { IssuesService } from './issues.service';
import { NgFor } from '@angular/common';

@Component({
  standalone: true,
  selector: 'issues',
  templateUrl: './issues.component.html',
  imports: [NgFor]
})
export class IssuesComponent {
  public issues$ = inject(IssuesService).issues$;
}
