import { computed, inject, Injectable, signal } from '@angular/core';
import { TauriService } from './tauri.service';
import { Router } from '@angular/router';

type State = {
  username: string | undefined,
  password: string | undefined,
  url: string | undefined
}

@Injectable({ providedIn: 'root' })
export class AuthService {
  private state = signal<State>({ username: undefined, password: undefined, url: undefined });
  private tauriService = inject(TauriService);
  private router = inject(Router);

  public loginData = computed(() => this.state());

  // public authenticated = computed(() => this.state().username != undefined && this.state().password != undefined);
  public authenticated = computed(() => true);

  public authenticate({ username, password, url }: { username: string, password: string, url: string }) {
    this.state.update(() => ({ username, password, url }));
    this.tauriService.invoke<string>('authenticate', { username, password, url }).subscribe({
      error: (e) => console.error(e)
    })
  }

  public logout() {
    this.state.update(state => ({ ...state, username: undefined, password: undefined }));
    this.tauriService.invoke<string>('logout').subscribe({ error: (e) => console.error(e) });
    this.router.navigateByUrl('/login');
  }
}
