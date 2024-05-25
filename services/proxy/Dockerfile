FROM node:20 AS build

WORKDIR /app

COPY ./services/client/package.json ./
COPY ./services/client/package-lock.json ./
RUN npm install
COPY ./services/client/ ./
RUN npm run build

FROM nginx:mainline-alpine AS development
RUN rm /etc/nginx/conf.d/default.conf
COPY ./services/proxy/proxy.conf /etc/nginx/conf.d/default.conf

FROM nginx:mainline-alpine AS production
RUN rm /etc/nginx/conf.d/default.conf
RUN mkdir -p /etc/nginx/ssl
COPY ./services/proxy/proxy.conf /etc/nginx/conf.d/default.conf
COPY --from=build /app/build /usr/share/nginx/html

