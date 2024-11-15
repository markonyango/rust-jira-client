import { Component, computed, inject } from '@angular/core';
import { RouterLink, RouterOutlet } from '@angular/router';
import { AuthService } from './services/auth.service';
import { NgIf } from '@angular/common';
import { CreateIssueButtonDirective } from './shared/components/create-issue/create-issue-button.directive';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [RouterOutlet, RouterLink, NgIf, CreateIssueButtonDirective],
  templateUrl: './app.component.html',
  styleUrl: './app.component.css'
})
export class AppComponent {
  private authService = inject(AuthService);

  public authenticated = this.authService.authenticated;
  public username = computed(() => this.authService.loginData().username);

  public logout() {
    this.authService.logout();
  }
}
