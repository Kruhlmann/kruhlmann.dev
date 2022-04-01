FROM debian:bullseye as build

RUN apt-get update \
    && apt-get install --no-install-recommends --assume-yes build-essential npm nodejs jq markdown curl \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*
RUN npm install --global html-cli sass

RUN mkdir -p /usr/src/build
WORKDIR /usr/src/build

COPY Makefile ./Makefile
COPY ./scripts ./scripts
COPY ./src ./src
COPY ./static ./static

RUN make

FROM debian:bullseye as production

RUN apt-get update && apt-get install --no-install-recommends --assume-yes apache2

COPY ./docker/entrypoint /usr/local/bin/start-server
COPY --from=build /usr/src/build/dist /var/www/html
RUN chmod -R 775 /var/www/html

EXPOSE 80
CMD ["/usr/local/bin/start-server"]
