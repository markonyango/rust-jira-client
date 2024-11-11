import { Component, inject } from '@angular/core';
import { MatTabsModule } from '@angular/material/tabs';
import { FilterService } from './services/filter.service';
import { NgFor } from '@angular/common';
import { RouterLink, RouterLinkActive, RouterOutlet } from '@angular/router';

@Component({
  selector: 'app-filters',
  standalone: true,
  imports: [MatTabsModule, NgFor, RouterOutlet, RouterLink, RouterLinkActive],
  templateUrl: './filters.component.html',
  styleUrl: './filters.component.css'
})
export class FiltersComponent {
  filterService = inject(FilterService);

  filters = this.filterService.filters;
}
