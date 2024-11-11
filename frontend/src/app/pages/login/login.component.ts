import { Component, inject } from '@angular/core';
import { MatInputModule } from '@angular/material/input';
import { AuthService } from '../../services/auth.service';
import { FormsModule } from '@angular/forms';
import { MatButtonModule } from '@angular/material/button';

@Component({
  standalone: true,
  template: `
    <form (ngSubmit)="login()">
      <mat-form-field>
        <mat-label>URL</mat-label>
        <input matInput type="text" [(ngModel)]="form.url" name="url" required />
      </mat-form-field>
      <mat-form-field>
        <mat-label>Username</mat-label>
        <input matInput type="text" [(ngModel)]="form.username" name="username" required />
      </mat-form-field>
      <mat-form-field>
        <mat-label>Password</mat-label>
        <input matInput type="password" [(ngModel)]="form.password" name="password" required />
      </mat-form-field>
      <button mat-raised-button type="submit">Submit</button>
    </form>
  `,
  styles:[`
    :host {
      display: flex;
      align-content: center;
      justify-content: center;
      align-items: center;
      height: 100%;
    }

    form {
     display: flex;
     flex-direction: column;
     gap: 2em;
    }
  `],
  imports: [MatInputModule, FormsModule, MatButtonModule]
})
export class LoginComponent {
  private authService = inject(AuthService);

  public form: { username: string | undefined, password: string | undefined, url: string | undefined } = { username: undefined, password: undefined, url: undefined };

  public login() {
    const { username, password, url } = this.form;

    if (username == undefined || password == undefined || url == undefined) {
      throw Error('Login fields must be filled out!');
    }

    this.authService.authenticate({ username, password, url });
    history.back()
  }
}
