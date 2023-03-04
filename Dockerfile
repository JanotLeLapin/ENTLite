FROM node:17-alpine

RUN npm i -g pnpm

COPY package.json pnpm-lock.yaml ./
RUN pnpm i

COPY . .
RUN pnpm build

CMD [ "pnpm", "start" ]