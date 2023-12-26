import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms'
import { AuthService } from '../auth.service';
import { BrowserstorageService } from '../../browserstorage.service';
import { HTTP_HEADERS, STORAGE_KEYS } from '../../app.constants';

@Component({
  selector: 'app-login',
  standalone: true,
  imports: [
    FormsModule,
  ],
  templateUrl: './login.component.html',
})
export class LoginComponent {

  username!: string
  password!: string

  constructor(
    private authService: AuthService,
    private store: BrowserstorageService
  ) {}

  newAccount() {
    this.authService.createAccount()
      .subscribe(r => console.log(r))
  }

  login() {
    console.log(this.username)
    console.log(this.password)
    this.authService.login(this.username, this.password)
      .subscribe(
        response => {
          console.log('Login successful:', response)
          const bearerToken = response.headers.get(HTTP_HEADERS.AUTHORIZATION)
          if (bearerToken) {
            console.log('Bearer Token:', bearerToken)
            this.store.setString(STORAGE_KEYS.BEARER_TOKEN, bearerToken)
          }
        },
        error => {
          console.error('Login error:', error)
        }
      )
  }

  account() {
    console.log('account request made')
    this.authService.accountDetail().subscribe(r => console.log(r))
  }
}
