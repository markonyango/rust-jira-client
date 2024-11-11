import { inject } from '@angular/core';
import { CanActivateFn, Router, Routes } from '@angular/router';
import { AuthService } from './services/auth.service';
import { LoginComponent } from './pages/login/login.component';

const isAuthenticated: CanActivateFn = (route, state) => {
  const isAuthenticated = inject(AuthService).authenticated();

  if (isAuthenticated) {
    return true;
  }

  return inject(Router).parseUrl('/login');
}

export const routes: Routes = [
  {
    path: 'login',
    component: LoginComponent
  },
  {
    path: 'filters',
    loadChildren: () => import('./pages/filters/filters-routes').then(m => m.routes),
    canActivate: [isAuthenticated]
  },
  {
    path: 'issues',
    loadChildren: () => import('./pages/issues/issues-routes').then(m => m.routes),
    canActivate: [isAuthenticated]
  },
];

