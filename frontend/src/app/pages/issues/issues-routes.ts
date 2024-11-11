import { ResolveFn, Routes } from '@angular/router';
import { IssuesComponent } from './issues.component';
import { IssueDetailsComponent } from './components/issue-details/issue-details.component';
import { IssuesApiService } from './services/issues-api.service';
import { inject } from '@angular/core';
import { map } from 'rxjs';
import { Issue } from './issue.type';

const issueResolver: ResolveFn<Issue> = (route, state) => {
  const id = route.paramMap.get('id')!;
  return inject(IssuesApiService).getIssue(id);
}

const issuesResolver: ResolveFn<Issue[]> = (route, state) => {
  const query = route.queryParamMap.get('query') ?? '';
  console.log('Resolving issues route with query:', query);

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

