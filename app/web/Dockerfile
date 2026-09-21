# Production image -- BB26090904. Multi-stage: install once, build once,
# ship only the standalone Next.js output (no dev deps, no pnpm store, no
# bind mounts). Dockerfile.dev stays the local-dev image (BB26090903) --
# this file is a genuinely separate artifact, not a variant of it.
FROM node:20-slim AS deps
RUN corepack enable && corepack prepare pnpm@12.3.4 --activate
WORKDIR /app
COPY package.json pnpm-workspace.yaml pnpm-lock.yaml .npmrc ./
COPY app/web/package.json app/web/package.json
COPY packages/shared/package.json packages/shared/package.json
RUN pnpm install --frozen-lockfile

FROM node:20-slim AS builder
RUN corepack enable && corepack prepare pnpm@12.3.4 --activate
WORKDIR /app
COPY --from=deps /app/node_modules ./node_modules
COPY --from=deps /app/app/web/node_modules ./app/web/node_modules
COPY . .
WORKDIR /app/app/web
RUN pnpm build

FROM node:20-slim AS runner
WORKDIR /app
ENV NODE_ENV=production
# .next/standalone already contains a trimmed node_modules (only the
# packages actually reachable at runtime, per Next.js's own tracing) plus
# a generated server.js entrypoint -- no pnpm/workspace tooling needed
# in this final stage at all.
COPY --from=builder /app/app/web/.next/standalone ./
COPY --from=builder /app/app/web/.next/static ./app/web/.next/static
COPY --from=builder /app/app/web/public ./app/web/public
EXPOSE 3000
CMD ["node", "app/web/server.js"]
