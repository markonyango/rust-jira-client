import { inject, Injectable, signal } from '@angular/core';
import { toSignal } from '@angular/core/rxjs-interop';
import { TauriService } from '../../../services/tauri.service';
import { Filter } from '../filter.type';

@Injectable({ providedIn: 'root' })
export class FilterService {
  private selectedFilterId = signal<string | undefined>(undefined);

  private tauriService = inject(TauriService);

  private filters$ = this.tauriService.invoke<Filter[]>('get_filters');
  filters = toSignal(this.filters$, { initialValue: undefined });

  public setFilterId(id: string) {
    this.selectedFilterId.set(id);
  }
}
