FROM nginx:mainline-alpine AS development
RUN rm /etc/nginx/conf.d/default.conf
COPY proxy.conf /etc/nginx/conf.d/default.conf

FROM nginx:mainline-alpine AS production
RUN rm /etc/nginx/conf.d/default.conf
RUN mkdir -p /etc/nginx/ssl
COPY proxy.conf /etc/nginx/conf.d/default.conf
