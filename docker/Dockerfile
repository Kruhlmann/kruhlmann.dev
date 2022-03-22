FROM debian:bullseye

RUN apt-get update && apt-get install --no-install-recommends --assume-yes apache2

COPY ./docker/entrypoint /usr/local/bin/start-server
ADD ./dist /var/www/html/
RUN chmod -R 775 /var/www/html

EXPOSE 80
CMD ["/usr/local/bin/start-server"]
