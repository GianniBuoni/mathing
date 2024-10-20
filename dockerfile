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

FROM base AS build
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --frozen-lockfile
COPY . .
RUN pnpm run build 

FROM base AS production
ENV NODE_ENV=production
RUN --mount=type=cache,id=pnpm,target=/pnpm/store pnpm install --frozen-lockfile

FROM base AS runtime
USER node
COPY --from=build /app/public ./public/
COPY --from=build /app/dist/ ./dist/
COPY --from=production /app/node_modules ./node_modules/
CMD ["node", "dist/server/entry.mjs"]
