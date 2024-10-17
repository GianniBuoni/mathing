FROM node:22-slim AS base
ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable
WORKDIR /app
ENV HOST=0.0.0.0
ENV PORT=4321
EXPOSE 4321
COPY package*.json ./
COPY pnpm-lock.yaml ./
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --frozen-lockfile
COPY . .

FROM base AS runtime
RUN pnpm run build 
CMD ["node", "dist/server/entry.mjs"]
