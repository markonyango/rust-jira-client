import { computed, inject, Injectable, signal } from '@angular/core';
import { TauriService } from '../../services/tauri.service';
import { tap } from 'rxjs';

type State = {
  issues: any[],
  selected_issue: number | undefined
}

@Injectable({ providedIn: 'root' })
export class IssuesService {
  public readonly state = signal<State>({ issues: [], selected_issue: undefined });

  public issues$ = computed(() => this.state().issues);
  public selected_issue$ = computed(() => this.state().selected_issue);

  private tauri_service = inject(TauriService);

  public constructor() {
    this.tauri_service.getIssueList().subscribe(response => this.update_issues(response.issues));
  }

  public select_issue(id: number) {
    this.state.update(state => ({
      ...state,
      selected_issue: id
    }))
  }

  public update_issues(issues: any[]) {
    this.state.update(state => ({
      ...state,
      issues
    }))
  }
}
