FROM node:22.11.0-alpine

WORKDIR /app

COPY package.json tsconfig.json package-lock.json postcss.config.js vite.config.ts index.html ./
COPY src/ src/

RUN npm install

ARG ENVIRONMENT=production
ARG NODE_ENV=production

ENV ENVIRONMENT=$ENVIRONMENT
ENV NODE_ENV=$NODE_ENV

CMD npm start
