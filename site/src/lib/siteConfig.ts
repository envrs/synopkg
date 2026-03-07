/**
 * Site-wide metadata for structured data schemas.
 *
 * These values are used across all schema builders to ensure consistency.
 */
export const siteConfig = {
  siteName: 'Synopkg',
  siteUrl: 'https://envrs.github.io/synopkg',
  description: 'Consistent dependency versions in large JavaScript Monorepos',
  softwareDescription:
    'CLI for managing dependency versions in JavaScript monorepos. Used by AWS, Cloudflare, DataDog, Electron, Microsoft, Vercel, and others.',
  logoUrl: 'https://envrs.github.io/synopkg/logo.svg',
  npmUrl: 'https://www.npmjs.com/package/synopkg',
  githubUrl: 'https://github.com/envrs/synopkg',
  authorName: 'Md Sulaiman',
  authorUrl: 'https://github.com/envrs',
  authorImage: 'https://avatars.githubusercontent.com/u/320492?v=4',
  authorSocial: [
    'https://github.com/envrs',
    'https://bsky.app/profile/foldleft.bsky.social',
    'http://uk.linkedin.com/in/envrsleeds',
    'https://twitter.com/fold_left',
  ],
  sponsorUrl: 'https://github.com/sponsors/envrs',
} as const;
