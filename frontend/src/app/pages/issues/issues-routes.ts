import { ResolveFn, Routes } from '@angular/router';
import { IssuesComponent } from './issues.component';
import { IssueDetailsComponent } from './components/issue-details/issue-details.component';
import { Issue, IssuesApiService, JqlResponse } from './services/issues-api.service';
import { inject } from '@angular/core';
import { map } from 'rxjs';

const issueResolver: ResolveFn<Issue> = (route, state) => {
  const id = route.paramMap.get('id')!;
  return inject(IssuesApiService).getIssue(id);
}

const issuesResolver: ResolveFn<Issue[]> = (route, state) => {
  const query = route.queryParamMap.get('query') ?? '';
  console.log('Resolving issues route with query:', query);
  inject(IssuesApiService).fetchAll().subscribe({
    next: res => console.log(res),
    error: err => console.error(err)
  });
  return inject(IssuesApiService).search(query).pipe(map(response => response.issues));
}

export const routes: Routes = [
  {
    path: '',
    component: IssuesComponent,
    resolve: { issues$: issuesResolver }
  },
  {
    path: ':id',
    component: IssueDetailsComponent,
    resolve: { issue: issueResolver }
  }
]

