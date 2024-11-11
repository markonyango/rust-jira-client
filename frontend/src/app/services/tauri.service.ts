import { Injectable } from '@angular/core';
import { invoke } from '@tauri-apps/api/core';
import { from, map, Observable, take, tap } from 'rxjs';

@Injectable({ providedIn: 'root' })
export class TauriService {
  public invoke<T = unknown>(fn: string, parameters?: Record<string, unknown>): Observable<T> {    
    return from(invoke<string>(fn, parameters)).pipe(
      map(res => JSON.parse(res))
    );
  }
}
