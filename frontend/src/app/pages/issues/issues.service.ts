import { inject, Injectable, signal } from '@angular/core';
import { IssuesApiService } from './services/issues-api.service';
import { catchError, EMPTY, map, switchMap } from 'rxjs';
import { toObservable } from '@angular/core/rxjs-interop';

@Injectable({ providedIn: 'root' })
export class IssuesService {
  private query = signal<string>('');
  private query$ = toObservable(this.query);
  private selected_issue$ = signal<string | undefined>(undefined);

  public issues$ = this.query$.pipe(
    switchMap((query) =>
      this.issues_api_service.search(query).pipe(
        catchError(() => EMPTY),
        map((response) => response.issues)
      )
    )
  );

  private issues_api_service = inject(IssuesApiService);

  public search(query: string) {
    this.query.set(query);
  }

  public select_issue(id: string) {
    this.selected_issue$.set(id);
  }
}
