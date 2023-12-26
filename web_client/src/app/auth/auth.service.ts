import { Injectable } from '@angular/core';
import { HttpClient, HttpResponse } from '@angular/common/http';
import { Observable } from 'rxjs'
import { Account, AccountDetail } from '../types';

@Injectable({
  providedIn: 'root'
})
export class AuthService {

  constructor(private client: HttpClient) { }

  createAccount(): Observable<Account> {
    const url = "/new";
    return this.client.get<Account>(url)
  }

  login(username: string, password: string): Observable<HttpResponse<unknown>> {
    const url = "/login";
    const body = {
      username,
      password
    }
    return this.client.post(url, body, { observe: 'response' })
  }

  accountDetail(): Observable<AccountDetail> {
    const url = "/account";
    return this.client.get<AccountDetail>(url)
  }
}
