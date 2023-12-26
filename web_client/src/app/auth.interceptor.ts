import { HttpInterceptorFn, HTTP_INTERCEPTORS } from '@angular/common/http';
import { HTTP_HEADERS, STORAGE_KEYS } from './app.constants';

export const authInterceptor: HttpInterceptorFn = (req, next) => {
  console.log('intercepted')
  const token = localStorage.getItem(STORAGE_KEYS.BEARER_TOKEN)
  if (token) {
    const requestWithToken = req.clone({
      headers: req.headers.set(HTTP_HEADERS.AUTHORIZATION, `Bearer ${token}`)
    })
    return next(requestWithToken)
  }
  return next(req);
}
