import { computed, inject, Injectable, signal } from '@angular/core';
import { TauriService } from './tauri.service';
import { Router } from '@angular/router';

type State = {
  username: string | undefined,
  password: string | undefined,
}

@Injectable({ providedIn: 'root' })
export class AuthService {
  private state = signal<State>({ username: undefined, password: undefined });
  private tauriService = inject(TauriService);
  private router = inject(Router);

  public loginData = computed(() => this.state());

  public authenticated = computed(() => this.state().username != undefined && this.state().password != undefined);

  public authenticate({username, password }: {username: string, password: string}) {
    this.state.update(() => ({ username, password }));
    this.tauriService.invoke<string>('authenticate', { username, password }).subscribe({
      error: (e) => console.error(e)
    })
  }

  public logout() {
    this.state.update(() => ({ username: undefined, password: undefined }));
    this.tauriService.invoke<string>('logout').subscribe({ error: (e) => console.error(e) });
    this.router.navigateByUrl('/login');
  }
}
