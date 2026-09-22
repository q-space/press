/** @type {import('next').NextConfig} */
const nextConfig = {
  // BB26090904: the production Dockerfile copies only .next/standalone +
  // .next/static + public into the runtime image -- without this, `next
  // start` needs the full node_modules tree, which the multi-stage build
  // deliberately doesn't carry into the final image.
  output: 'standalone',
};

export default nextConfig;
