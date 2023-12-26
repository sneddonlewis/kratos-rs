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

  login(card_number: string, pin: string): Observable<HttpResponse<unknown>> {
    const url = "/login";
    const body = {
      card_number,
      pin
    }
    return this.client.post(url, body, { observe: 'response' })
  }

  accountDetail(): Observable<AccountDetail> {
    const url = "/account";
    return this.client.get<AccountDetail>(url)
  }
}
