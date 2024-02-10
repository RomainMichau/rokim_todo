import {NgModule} from '@angular/core';
import {BrowserModule} from '@angular/platform-browser';
import {HTTP_INTERCEPTORS, HttpClientModule} from "@angular/common/http";
import {AppRoutingModule} from './app-routing.module';
import {AppComponent} from './app.component';
import {DialogOverviewExampleDialog, TodoListComponent} from './todo-list/todo-list.component';
import {ApiModule, Configuration} from "./services/todo";
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { MatButtonModule } from '@angular/material/button';
import {MatDialogModule} from "@angular/material/dialog";
import {MatInputModule} from "@angular/material/input";
import {MatFormFieldModule} from "@angular/material/form-field";
import { FormsModule } from '@angular/forms';
import {MatAutocompleteModule} from "@angular/material/autocomplete";
import {MatRadioModule} from "@angular/material/radio";
import {MatSlideToggleModule} from "@angular/material/slide-toggle";
import {MatIconModule} from "@angular/material/icon";
import {RedirectInterceptor} from "./redirect-302.interceptor";
@NgModule({
    declarations: [
        AppComponent,
        TodoListComponent,
        DialogOverviewExampleDialog
    ],
    imports: [
        BrowserModule,
        MatInputModule,
        MatFormFieldModule,
        MatDialogModule,
        MatIconModule,
        MatButtonModule,
        AppRoutingModule,
        FormsModule,
        HttpClientModule,
        ApiModule.forRoot(() => {
            return new Configuration({
                basePath: ``,
            })
        }),
        BrowserAnimationsModule,
        MatAutocompleteModule,
        MatRadioModule,
        MatSlideToggleModule,
    ],
    providers: [
        {
            provide: HTTP_INTERCEPTORS,
            useClass: RedirectInterceptor,
            multi: true
        }
    ],
    bootstrap: [AppComponent]
})
export class AppModule {
}
