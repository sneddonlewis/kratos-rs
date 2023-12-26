import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms'
import { RouterOutlet } from '@angular/router';
import { HttpClientModule } from '@angular/common/http';

import { LoginComponent } from './auth/login/login.component'

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [
    CommonModule,
    RouterOutlet,
    HttpClientModule,
    LoginComponent,
    FormsModule,
  ],
  templateUrl: './app.component.html',
  styleUrl: './app.component.scss'
})
export class AppComponent {
  title = 'web_client';
}
