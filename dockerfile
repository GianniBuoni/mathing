FROM node:22-slim AS base
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable
WORKDIR /app
COPY package*.json ./
COPY pnpm-lock.yaml ./
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --frozen-lockfile
COPY . .

FROM base AS dev
ENV HOST=0.0.0.0
ENV PORT=4321
EXPOSE 4321
CMD [ "pnpm", "start", "--host"]

FROM base AS build
RUN pnpm run build && touch ./dist/.htaccess && echo "ErrorDocument 404 /404.html"> ./dist/.htaccess

FROM httpd:2.4 AS runtime
WORKDIR /usr/local/apache2/conf
RUN sed -i 's/AllowOverride None/AllowOverride all/' httpd.conf
COPY --from=build /app/dist /usr/local/apache2/htdocs/
EXPOSE 80
