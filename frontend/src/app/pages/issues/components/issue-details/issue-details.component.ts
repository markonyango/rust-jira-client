import { DatePipe, JsonPipe, NgFor } from '@angular/common';
import { Component, computed, input } from '@angular/core';
import { Issue } from '../../services/issues-api.service';
import { CommentListComponent } from '../comments/comment-list.component';
import { DescriptionComponent } from '../description/description.component';
import { DetailsComponent } from '../details/details.component';
import { TimeTrackingComponent } from '../time-tracking/time-tracking.component';

@Component({
  standalone: true,
  templateUrl: './issue-details.component.html',
  imports: [
    JsonPipe,
    NgFor,
    DatePipe,
    CommentListComponent,
    DescriptionComponent,
    DetailsComponent,
    TimeTrackingComponent,
  ],
  styleUrl: './issue-details.component.scss',
})
export class IssueDetailsComponent {
  public issue = input.required<Issue>();
}
