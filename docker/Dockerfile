FROM node:14.17.3-alpine3.14 as dependencies

WORKDIR /usr/dependencies
COPY yarn.lock .
COPY package.json .
RUN yarn install --production

FROM node:14.17.3-alpine3.14 as build

WORKDIR /usr/build
RUN mkdir config
RUN apk add --no-cache make git

COPY yarn.lock .
COPY package.json .
COPY make ./make
COPY Makefile .
RUN make node_modules

COPY svelte.config.js .
COPY rollup.config.js .
COPY babel.config.js .
COPY tsconfig.json .
COPY typings ./typings
COPY config/rollup-plugins.js ./config
COPY config/language_colors.json ./config
COPY config/config.json ./config
COPY .git ./.git
COPY src ./src
RUN make build
RUN rm -rf ./config

FROM node:14.17.3-alpine3.14

WORKDIR /usr/app

COPY --from=dependencies /usr/dependencies/node_modules ./node_modules
COPY --from=build /usr/build/__sapper__ ./__sapper__
COPY static ./static

EXPOSE 3000
ENTRYPOINT ["node", "__sapper__/build"]
