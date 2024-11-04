import { Component, input } from '@angular/core';
import { CommentWrapper } from './comment.type';
import { DatePipe, NgFor } from '@angular/common';
import { MatExpansionModule } from '@angular/material/expansion';
import { CommentComponent } from "./comment.component";

@Component({
  standalone: true,
  selector: 'comment-list',
  imports: [NgFor, MatExpansionModule, CommentComponent],
  template: `
    <mat-expansion-panel>
      <mat-expansion-panel-header>Kommentare ({{ comment().total}})</mat-expansion-panel-header>
      <ng-container *ngFor="let _comment of comment().comments">
        <comment [comment]="_comment" />
      </ng-container>
    </mat-expansion-panel>
  `,
})
export class CommentListComponent {
  public comment = input.required<CommentWrapper>();
}
