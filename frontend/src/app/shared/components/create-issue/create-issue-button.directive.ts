import { Directive, HostListener, inject } from '@angular/core';
import { MatDialog } from '@angular/material/dialog';
import { CreateIssueComponent } from './create-issue.component';

@Directive({
  standalone: true,
  selector: '[create-issue]'
})
export class CreateIssueButtonDirective {
  @HostListener('click', ['$event']) buttonClicked(event: Event) {
    event.preventDefault();
    event.stopPropagation();
    console.log('button clicked');
    this.dialog.open(CreateIssueComponent, { width: '800px' });
  }

  private dialog = inject(MatDialog)
}
