import { Injectable } from '@angular/core';

@Injectable({
  providedIn: 'root'
})
export class BrowserstorageService {

  constructor() { }

  setString(key: string, value: string) {
    localStorage.setItem(key, value)
  }

  getString(key: string): string | null {
    return localStorage.getItem(key)
  }
}
