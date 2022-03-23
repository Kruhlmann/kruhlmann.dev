FROM debian:bullseye as build

RUN apt-get update && apt-get install --no-install-recommends --assume-yes build-essential npm nodejs jq markdown

WORKDIR /usr/src/build

RUN npm install --global html-cli sass

COPY Makefile ./Makefile
COPY ./scripts ./scripts
COPY ./src ./src

FROM debian:bullseye as production

RUN apt-get update && apt-get install --no-install-recommends --assume-yes apache2

COPY ./docker/entrypoint /usr/local/bin/start-server
COPY --from=build /usr/src/build/dist /var/www/html
RUN chmod -R 775 /var/www/html

EXPOSE 80
CMD ["/usr/local/bin/start-server"]
