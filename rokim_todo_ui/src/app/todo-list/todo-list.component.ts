// src/app/todo-list/todo-list.component.ts

import {Component, Inject, OnInit} from '@angular/core';
import {Todo, TodoControllerService} from "../services/todo";
import {MAT_DIALOG_DATA, MatDialog, MatDialogRef,} from '@angular/material/dialog';

@Component({
    selector: 'app-todo-list',
    templateUrl: './todo-list.component.html',
    styleUrls: ['./todo-list.component.css']
})
export class TodoListComponent implements OnInit {
    todos_per_category: Map<string, Todo[]> = new Map<string, Todo[]>();
    todos_per_id = new Map<number, { todo: Todo, is_expanded: boolean }>();
    categoryState: { [key: string]: { expand: boolean, show_done: boolean } } = {};
    showDoneTodos = false;

    constructor(private todoService: TodoControllerService, public dialog: MatDialog) {
    }

    ngOnInit(): void {
        this.todoService.getTodos().subscribe(
            data => {

                for (const todo of data) {
                    this.todos_per_id.set(todo.id, {todo: todo, is_expanded: false});
                    if (this.todos_per_category.has(todo.category)) {
                        this.todos_per_category.get(todo.category)?.push(todo);
                    } else {
                        this.todos_per_category.set(todo.category, [todo]);

                    }
                }
                this.todos_per_category.forEach((_, category) => {
                    this.categoryState[category] = {
                        expand: true,
                        show_done: false
                    }; // Initially, all categories are not minimized
                });
            },
            error => {
                console.error('Error fetching TODOs', error);
            }
        );
    }


    toggleDoneTodosForCategory(category: string): void {
        this.categoryState[category].show_done = !this.categoryState[category].show_done;
    }

    getFilteredTodos(category: string): Todo[] {
        const todos = this.todos_per_category.get(category) || [];
        return this.categoryState[category].show_done ? todos : todos.filter(todo => todo.done_at == undefined);
    }


    toggleCategoryExpand(category: string): void {
        this.categoryState[category].expand = !this.categoryState[category].expand;
    }

    isCategoryMinimized(category: string): boolean {
        return !this.categoryState[category].expand;
    }

    doesCategoryShowDone(category: string): boolean {
        return this.categoryState[category].show_done;
    }


    markAsDone(todoId: number): void {
        this.todoService.markTodoAsDone(todoId).subscribe(
            data => {
                const todo = this.todos_per_id.get(todoId);

                if (todo) {
                    todo.todo.done_at = data.done_at;
                } else {
                    console.error(`Todo with ID ${todoId} not found.`);
                }
            },
            error => {
                console.error('Error marking TODO as done', error);
            }
        );
    }

    isTodoDetailsMinimized(todoId: number): boolean {
        const todo = this.todos_per_id.get(todoId);

        if (todo) {
            return !todo.is_expanded;
        } else {
            console.error(`Todo with ID ${todoId} not found.`);
            return true;
        }
    }

    toggleTodoDetails(todoId: number): void {
        const todo = this.todos_per_id.get(todoId);
        if (todo) {
            todo.is_expanded = !todo.is_expanded;
        } else {
            console.error(`Todo with ID ${todoId} not found.`);
        }
    }

    markAsTodo(todoId: number): void {
        this.todoService.markTodoAsUndone(todoId).subscribe(
            data => {
                const todo = this.todos_per_id.get(todoId);

                if (todo) {
                    todo.todo.done_at = data.done_at;
                } else {
                    console.error(`Todo with ID ${todoId} not found.`);
                }
            },
            error => {
                console.error('Error marking TODO as done', error);
            }
        );
    }

    deleteTodo(todo: Todo): void {
        this.todoService.deleteTodo(todo.id).subscribe(
            data => {
                this.todos_per_id.delete(todo.id);
                let maybeTodos = this.todos_per_category.get(todo.category);
                if (maybeTodos) {
                    const index = maybeTodos.indexOf(todo);
                    if (index > -1) {
                        maybeTodos.splice(index, 1);
                    }
                }

            },
            error => {
                console.error('Error marking TODO as done', error);
            }
        );
    }

    formatDateTime(dateTime: string | undefined): string {
        if (dateTime) {
            const date = new Date(dateTime);
            return date.toLocaleString();
        } else {
            return '';
        }
    }

    openDialog(): void {
        const dialogRef = this.dialog.open(DialogOverviewExampleDialog, {
            width: '60%',
            data: {
                categories: Array.from(this.todos_per_category.keys())
            }
        });

        dialogRef.afterClosed().subscribe(result => {
            if (result) {
                console.log('The dialog was closed', result);
                this.todoService.createTodos({
                    title: result.title,
                    description: result.description,
                    category: result.category
                }).subscribe(
                    data => {
                        console.log('Todo created', data);
                        if (this.todos_per_category.has(data.category)) {
                            this.todos_per_category.get(data.category)?.push(data);
                        } else {
                            this.todos_per_category.set(data.category, [data]);
                            this.categoryState[data.category] = {
                                expand: true,
                                show_done: false
                            }; // Initially, all categories are not minimized
                        }
                        this.todos_per_id.set(data.id, {todo: data, is_expanded: false});
                    },
                    error => {
                        console.error('Error creating TODO', error);
                    }
                );
            } else {
                console.log('The dialog was closed');
            }
        });
    }
}

export interface DialogData {
    title: string;
    description: string;
    category: string;
}

export interface DialogInputData {
    categories: string[];
}

@Component({
    selector: 'dialog-overview-example-dialog',
    templateUrl: 'dialog-overview-example-dialog.html',
    styleUrls: ['./dialog-overview-example-dialog.css']
})
export class DialogOverviewExampleDialog {

    data: DialogData = {
        title: '',
        description: '',
        category: '',
    };


    constructor(
        public dialogRef: MatDialogRef<DialogOverviewExampleDialog>,
        @Inject(MAT_DIALOG_DATA) public input: DialogInputData
    ) {
    }

    onNoClick(): void {
        this.dialogRef.close();
    }
}
