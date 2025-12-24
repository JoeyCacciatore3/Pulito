# Pulito Marketing Site

This is the standalone marketing website for Pulito, completely separate from the desktop application.

## Overview

The marketing site is a static SvelteKit application that can be deployed independently to GitHub Pages or any static hosting service. It contains:

- Landing page (`/`)
- Features page (`/features`)
- Download page (`/download`)
- FAQ page (`/faq`)
- Privacy policy (`/privacy`)
- Terms of service (`/terms`)

## Development

```bash
# Install dependencies
npm install

# Start development server
npm run dev

# Build for production
npm run build

# Preview production build
npm run preview
```

## Deployment

The marketing site builds to the `build/` directory and can be deployed to:

- **GitHub Pages**: Deploy the `build/` directory
- **Netlify**: Connect the `marketing/` folder
- **Vercel**: Connect the `marketing/` folder
- **Any static host**: Upload the `build/` directory contents

## Structure

```
marketing/
├── src/
│   ├── routes/          # Marketing pages
│   ├── lib/             # Marketing utilities
│   ├── app.css          # Styles
│   └── app.html         # HTML template
├── static/              # Static assets
├── package.json         # Marketing dependencies
└── svelte.config.js     # SvelteKit config
```

## Dependencies

This site uses minimal dependencies:
- Svelte 5
- SvelteKit
- Tailwind CSS
- TypeScript

**No Tauri dependencies** - this is a pure web application.

## Build Output

The build process creates a static site in `build/` with all routes prerendered for optimal performance and SEO.
