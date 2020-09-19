FROM node:latest AS react

WORKDIR /usr/src/app

COPY ./package.json ./

RUN npm install

COPY . .

CMD [ "npm", "run", "build" ]

FROM nginx:latest

COPY --from=react /usr/src/app/build /usr/share/nginx/html
