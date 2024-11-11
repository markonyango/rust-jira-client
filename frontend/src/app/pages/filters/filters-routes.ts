import { ResolveFn, Routes } from '@angular/router'
import { FiltersComponent } from './filters.component';
import { FilterComponent } from './components/filter.component';
import { inject } from '@angular/core';
import { TauriService } from '../../services/tauri.service';
import { map, switchMap } from 'rxjs';
import { Filter } from './filter.type';
import { JqlResponse } from '../issues/issue.type';

const filterResolver: ResolveFn<any> = (route, state) => {
  const id = Number.parseInt(route.paramMap.get('id')!);
  const tauriService = inject(TauriService);

  return tauriService.invoke<Filter>('get_filter', { id }).pipe(
    switchMap(filterData => tauriService.invoke<JqlResponse>('search', { query: filterData.jql}).pipe(
      map(queryData => ({ filter: filterData, issues: queryData.issues }))
    ))
  );
}

export const routes: Routes = [
  {
    path: '',
    component: FiltersComponent,
    children: [
      {
        path: ':id',
        component: FilterComponent,
        resolve: {
          data: filterResolver
        }
      }
    ]
  },
  
]
