import { inject, Injectable } from '@angular/core';
import { TauriService } from '../../../services/tauri.service';
import { Issue, JqlResponse } from '../issue.type';

@Injectable({ providedIn: 'root' })
export class IssuesApiService {

  private tauriService = inject(TauriService);

  public search(query: string) {
    return this.tauriService.invoke<JqlResponse>('search', { query });
  }

  public getIssue(id: string) {
    return this.tauriService.invoke<Issue>('get_issue', { id });
  }
}

