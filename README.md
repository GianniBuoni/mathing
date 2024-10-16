# Gianni's Astro Starter Kit: Minimal

This is my minimal project starter for Astro ✨  
I'm working on configuring it just the way I want to easily get up and running on a new project.

## 🚀 Extra Configurations

- Typescript import aliases
- Astro Content Collections
- Astro Icon, Sitemap, RSS Inegrations
- Enabling prefetching
- Post CSS & Tailwind setup
- Multistage Dockerfile & Compose
- Astro Font Setup
- Basic components that I end up reusing all the time
- A branch for SSR configuration (that defaults to Astro's hybrid mode)

## 🐳 Docker

The docker-compose file is split up into a three different profiles that target different my use cases.

- `dev`: sets up a network exposed node server (with pnpm enabled); good for quick development and debugging/viewing on different machines
- `test`: spins up an apache server that uses local build files; gives me a good preview to catch any build issues that don't show up in the dev environment
- `runtime`: packages everything up; for use on the server only really

## ✅ To Do

- [x] Find a Next-like Font solution
- [x] Expand `dockerfile` to include an SSR build environment (probably gonna stick with node)
- [ ] Switch font solution. Relying on pulling fonts from google is a little slow in a production environment.

## 👀 Resources

- [Astro Documentation](https://docs.astro.build)
- [Tailwind CSS](https://tailwindcss.com/docs/installation)
- [Daisy UI](https://daisyui.com/components/)
- [Astro Fonts](https://github.com/rishi-raj-jain/astro-font)
