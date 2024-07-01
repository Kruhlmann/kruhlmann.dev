FROM debian:bullseye AS build

RUN apt-get update \
    && apt-get install --no-install-recommends --assume-yes build-essential npm nodejs jq markdown curl \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/* \
    && npm install --global html-cli sass \
    && mkdir -p /usr/src/build

WORKDIR /usr/src/build

COPY Makefile ./Makefile
COPY ./scripts ./scripts
COPY ./src ./src
COPY ./static ./static

RUN make

FROM debian:bullseye

RUN apt-get update \
    && apt-get install --no-install-recommends --assume-yes python3 python3-pip \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

COPY ./docker/entrypoint /usr/local/bin/start-server
COPY --from=build /usr/src/build/dist /var/www/html
RUN chmod -R 775 /var/www/html

WORKDIR /var/www/html
EXPOSE 8987
CMD ["/usr/local/bin/start-server"]