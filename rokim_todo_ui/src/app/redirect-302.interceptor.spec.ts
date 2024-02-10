import { TestBed } from '@angular/core/testing';

import { Redirect302Interceptor } from './redirect-302.interceptor';

describe('Redirect302Interceptor', () => {
  beforeEach(() => TestBed.configureTestingModule({
    providers: [
      Redirect302Interceptor
      ]
  }));

  it('should be created', () => {
    const interceptor: Redirect302Interceptor = TestBed.inject(Redirect302Interceptor);
    expect(interceptor).toBeTruthy();
  });
});
