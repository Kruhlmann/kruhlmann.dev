FROM nginx:latest

RUN rm -rf /usr/share/nginx/html
COPY src/ /usr/share/nginx/html/
WORKDIR /usr/share/nginx/html