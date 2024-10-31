import { Routes } from '@angular/router';

export const routes: Routes = [
  { path: 'issues', loadChildren: () => import('./pages/issues/issues-routes').then(m => m.routes) },
];
