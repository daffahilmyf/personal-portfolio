# create-svelte

Everything you need to build a Svelte project, powered by [`create-svelte`](https://github.com/sveltejs/kit/tree/main/packages/create-svelte).

## Creating a project

If you're seeing this, you've probably already done this step. Congrats!

```bash
# create a new project in the current directory
npm create svelte@latest

# create a new project in my-app
npm create svelte@latest my-app
```

## Developing

Once you've created a project and installed dependencies with `npm install` (or `pnpm install` or `yarn`), start a development server:

```bash
npm run dev

# or start the server and open the app in a new browser tab
npm run dev -- --open
```

## Building

To create a production version of your app:

```bash
npm run build
```

You can preview the production build with `npm run preview`.

> To deploy your app, you may need to install an [adapter](https://kit.svelte.dev/docs/adapters) for your target environment.

## Testing

### Unit / Component Testing

Place the test files in `tests/vitest` folder, following the hierarchy of `src`. Use kebab case for file names. Use `*.test.js` for unit tests and `*.spec.js` for component test.

### Integration / E2E Testing

Place the test files in `tests/playwright` folder according to their category. Use kebab case for file names.

- integration test files: Use `*.test.js`. Name the file using the integrated module / component name as a whole following top-down principle e.g. if you're testing the integration of modules / components in Navigation Bar, then you can name the file `navbar.test.js`.
- e2e test files: Use `*.spec.js`. Name the file using the use case e.g. `redirect-to-playground-page.spec.js`

### Playwright for Unsupported Linux Distribution

If you're using a linux distribution that is unsupported by Playwright and you want to test locally, then you need to use docker as the test server, see: [\[TIP\] Run Playwright Tests on unsupported Linux distributions](https://github.com/microsoft/playwright/issues/26482).

Run this command to run Playwright docker test server:

```bash
docker run --add-host=hostmachine:host-gateway -p 3000:3000 --rm --init -it mcr.microsoft.com/playwright:v1.42.1-jammy /bin/sh -c "cd /home/pwuser && npx -y playwright@1.42.1 run-server --port 3000 --host 0.0.0.0"
```

Use these environment variables to run Playwright:

```dotenv
# .env
PW_BASE_URL='http://172.17.0.1:4173/' # Change this according to you docker bridge ip address (this is usually the default)
PW_WEB_SERVER_CMD='pnpm run build && pnpm run preview --host=0.0.0.0'
```

Then you can run one of these command:

```bash
# run all test including vitest
PW_TEST_CONNECT_WS_ENDPOINT=ws://0.0.0.0:3000/ pnpm run test

# or just run playwright tests
PW_TEST_CONNECT_WS_ENDPOINT=ws://0.0.0.0:3000/ pnpm run test:integration
```

If you want to use Playwright web UI, you can run this command:

```bash
PW_TEST_CONNECT_WS_ENDPOINT=ws://0.0.0.0:3000/ pnpm run test:integration --ui-host=0.0.0.0 --ui-port=8080
```
