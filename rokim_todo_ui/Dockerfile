FROM node:16-alpine AS build

WORKDIR /webapp

COPY . .

RUN npm install

RUN npm run build

FROM nginx:alpine

COPY --from=build /webapp/dist/rokim_todo_ui/ /usr/share/nginx/html