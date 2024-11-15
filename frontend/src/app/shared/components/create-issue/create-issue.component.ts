import { Component } from '@angular/core';
import { MatButtonModule } from '@angular/material/button';
import { MatCardModule } from '@angular/material/card';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatSelectModule } from '@angular/material/select';
import { MatDividerModule } from '@angular/material/divider';
import { MatInputModule } from '@angular/material/input';
import { MatDialogModule } from '@angular/material/dialog';

@Component({
  standalone: true,
  templateUrl: './create-issue.component.html',
  styleUrl: './create-issue.component.css',
  imports: [MatCardModule, MatInputModule, MatFormFieldModule, MatSelectModule, MatButtonModule, MatDividerModule, MatDialogModule]
})
export class CreateIssueComponent { }
