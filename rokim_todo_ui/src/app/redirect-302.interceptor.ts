import { Injectable } from '@angular/core';
import {
  HttpInterceptor,
  HttpRequest,
  HttpHandler,
  HttpEvent,
  HttpResponse,
  HttpErrorResponse
} from '@angular/common/http';
import { Observable, throwError } from 'rxjs';
import { catchError } from 'rxjs/operators';
import { Router } from '@angular/router';

@Injectable()
export class RedirectInterceptor implements HttpInterceptor {

  constructor(private router: Router) {}

  intercept(request: HttpRequest<any>, next: HttpHandler): Observable<HttpEvent<any>> {
    return next.handle(request).pipe(
        catchError((error: any) => {
          if (error instanceof HttpErrorResponse && error.status === 302) {
            // Extract the location header from the response
            const locationHeader = error.headers.get('Location');
            if (locationHeader) {
              // Navigate to the URL specified in the location header
              this.router.navigateByUrl(locationHeader);
            } else {
              console.error('Location header not found in the response');
            }
          }
          return throwError(error);
        })
    );
  }
}
