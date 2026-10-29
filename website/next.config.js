/** @type {import('next').NextConfig} */
const nextConfig = {
  output: 'export',
  basePath: '/Persica',
  images: {
    unoptimized: true,
  },
  trailingSlash: true,
};

module.exports = nextConfig;
