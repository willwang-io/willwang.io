/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
async headers() {
    return [
      {
        source: '/blog/:slug',
        headers: [
          {
            key: 'x-slug',
            value: ':slug', // Matched parameters can be used in the value
          },
          {
            key: 'x-slug-:slug', // Matched parameters can be used in the key
            value: 'my other custom header value',
          },
        ],
      },
    ]
  },  async headers() {

  }
}

module.exports = {
  async headers() {
    return [
      {
        source: '/leecode/:problemId',
        headers: [
          {
            key: 'x-slug',
            value: ':problemId', // Matched parameters can be used in the value
          },
          {
            key: 'x-slug-:problemId', // Matched parameters can be used in the key
            value: 'my other custom header value',
          },
        ],
      },
    ]
  },
}