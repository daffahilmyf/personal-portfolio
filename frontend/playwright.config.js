import 'dotenv/config';

/** @type {import('@playwright/test').PlaywrightTestConfig} */
const config = {
	outputDir: 'tests/playwright/test-results',
	reporter: process.env.CI ? 'dot' : [['list'], ['html']],
	retries: process.env.CI ? 2 : 0,
	testDir: 'tests/playwright',
	testMatch: /(.+\.)?(test|spec)\.js/,
	use: {
		baseURL: process.env.PW_BASE_URL,
		trace: 'on-first-retry'
	},
	webServer: {
		command: process.env.PW_WEB_SERVER_CMD,
		port: 4173
	}
};

export default config;
