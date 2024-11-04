import { Component, input } from '@angular/core';
import { Comment } from './comment.type';
import { DatePipe } from '@angular/common';

@Component({
  standalone: true,
  selector: 'comment',
  imports: [DatePipe],
  template: `{{ comment().created | date }}: {{ comment().author.name }} -
    {{ comment().body }}`,
})
export class CommentComponent {
  public comment = input.required<Comment>();
}
