import { Routes } from '@angular/router';
import { LoginComponent } from './auth/login/login.component';
import { AuthGuard } from './auth.guard';
import { HomePageComponent } from './home-page/home-page.component';
import { PageNotFoundComponent } from './page-not-found/page-not-found.component';

export const routes: Routes = [
  { path: 'login', component: LoginComponent },
  { path: '', component: HomePageComponent, canActivate: [ AuthGuard ] },
  { path: '**', component: PageNotFoundComponent },
];
