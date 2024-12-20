# 🛠️ Tooloxide

> [!IMPORTANT]
> ❌ This is not finished. This is in early development. This __**currently**__ does **not** work - at all.

## 💻 What is this?

[Tooloxide](https://github.com/hozhai/tooloxide) is an open-source website serving as a collection of online tools that are designed to be free to use with no ads thanks to only needing a serverless environment, meaning you can host your own fork on platforms such as [Vercel](https://vercel.com/) or [Netlify](https://netlify.com/) (you will have to modify the serverless functions in that case), et cetera.

## ⁉️ But why?
The main motive for the creation of this project is to solve the lack of *competent** online tool websites that are easily found within a reasonable timeframe.

By *competent* I mean the posession of the following simple characteristics:
- **Free** (preferably both in *freedom* and *no cost*, but fine if just *no cost*)
- **No ads** (we all hate it, especially if you block adblockers)
- **No invasion of privacy** (if the product is free, you are the product. This aims to solve that)
- **Fast** (I can't tell you the amount of times I've sat in front of my computer waiting for something to say *uploading* and then *processing* just for it to take ages)
- **Unlimiting** (if I want to download a YouTube video, don't limit me to only 720p, that's so 2017)
- **Good UX** (I'm a programmer not a designer, but even I know that not giving user feedback on a long process like during a file conversation is going to make the user doubt it's even working)
- And most importantly, **working** (for example, having an online gif editor give you an error after you want to export it)

Some might say that this cannot be expected from most online tool sites and that there are already sites that do a good job at what they do (like [CloudConvert](https://cloudconvert.com/), but even they offer their stuff as a SaaS), and you are not wrong.

I just want to offer a FOSS solution that is self-hostable, free, doesn't look like it was made with Bootstrap v2.3.2 from 2013 (I love the style though), and doesn't litter you with ads.

## 📚 The Stack

Tooloxide is currently built with
- [TypeScript](https://www.typescriptlang.org/) 
- [Svelte 5 + SvelteKit](https://svelte.dev/) + [Vite](https://vite.dev/)
- [Rust](https://www.rust-lang.org/) compiled to [WebAssembly](https://webassembly.org/) with [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) and [Vercel's serverless functions](https://vercel.com/docs/functions) running the [Rust runtime](https://github.com/vercel-community/rust) to get that juicy 🦀⚡️ blazingly fast performance
- [TailwindCSS](https://tailwindcss.com/)
- [DaisyUI](https://daisyui.com/) and [shadcn-svelte](https://www.shadcn-svelte.com/)

## 📋 TODO

- [ ] add per-keyboard keybind display in search.svelte
- [ ] revamp search.svelte to no longer fully rely on shadcn-svelte's Command component but instead a full screen search and browse window

## 🚀 Hosting Your Own Version

Tooloxide aims to be a decentralized project allowing anyone to host their own version with a centralized (yet private and secure) deployment at <https://tooloxide.vercel.app>.

Here's how you can host your own version if you wish to:

### 1. Install the dependencies

The dependencies are as follows

- [Bun](https://bun.sh/) as the package manager (feel free to switch it out for an alternative)
- [Rust (nightly) and Cargo](https://www.rust-lang.org/) for you know why
- [wasm-pack](https://github.com/rustwasm/wasm-pack) for compiling Rust into WebAssembly

### 2. Clone/fork the repository

After you've installed the dependencies and have confirmed they work as expected.

Feel free to fork the repository and clone your own repo so that you can get updates while making your own.

If you want to clone this repo

```sh
$ git clone https://github.com/hozhai/tooloxide.git
$ cd tooloxide

# feel free to remove .git so you can init it as your own repo
$ rm -rf .git/
$ git init # and so on...
```

### 3. Install the NPM dependencies

```sh
# for bun
$ bun i

# for npm and pnpm
$ npm i
# or
$ pnpm i

# for yarn
$ yarn install
```

### 4. Run the dev server

> [!NOTE]
> Every time the `dev` script is run, all the Rust code will be compiled, both for WebAssembly and Vercel's serverless. The first time will take longer as the Cargo dependencies wll have to be downloaded and compiled.

> [!NOTE]
> You will need a [Vercel](https://vercel.com/) account when you run the `dev` script, as it will ask for you to sign in and link this project with an existing/new one for the serverless functions to work properly.

```sh
# replace the command depending on your pkg manager
$ bun dev
```

### 5. Deploy

After having cloned and initialized the repository, if you've followed the instructions step by step you should be able to head to the [Vercel dashboard](https://vercel.com/) and click on the link under your project's name.

If you are working with [Netlify](https://netlify.com/), then good luck, I don't know how to use it.

## 🌱 Contributing

Contributions are always welcome. If you don't know how to fix a problem, [create an issue](https://github.com/hozhai/tooloxide/issues/new) and wait. If you do know how to fix a problem, [create an issue](https://github.com/hozhai/tooloxide/issues/new) and mention that you'll be working on a fix, fork the repo, then [create a PR](https://github.com/hozhai/tooloxide/compare).

Your commits should have a verb in simple present tense (add, modify, remove) and the subject you have done a change on. Or just use AI, I can't stop you anyway.

You can add new tools, create a wiki, do anything, as long as it LGTM you're chilling.