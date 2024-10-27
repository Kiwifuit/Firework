FROM oven/bun:alpine AS build

WORKDIR /app
COPY package.json .
RUN bun install

COPY . .
RUN bun run build

FROM oven/bun:alpine

WORKDIR /app
COPY --from=build /app/.output .

EXPOSE 3000
CMD [ "bun", "server/index.mjs" ]