FROM node:20.3.1-alpine

WORKDIR /app

COPY package.json tsconfig.json package-lock.json postcss.config.js tailwind.config.js vite.config.ts index.html ./
COPY src/ src/

RUN npm install

ARG ENVIRONMENT=production
ARG NODE_ENV=production

ENV ENVIRONMENT=$ENVIRONMENT
ENV NODE_ENV=$NODE_ENV

CMD npm start
