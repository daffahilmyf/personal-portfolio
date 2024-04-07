import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';

export default defineConfig(({ mode }) => ({
	plugins: [sveltekit()],
	resolve: {
		conditions: mode === 'test' ? ['browser'] : []
	},
	test: {
		coverage: {
			enabled: true,
			provider: 'v8',
			reporter: ['text', 'html'],
			reportsDirectory: 'tests/vitest/test-results/coverage/'
		},
		css: true,
		environment: 'jsdom',
		include: ['tests/vitest/**/*.{test,spec}.js'],
		outputFile: 'tests/vitest/test-results/index.html',
		reporters: ['default', 'html'],
		setupFiles: ['./vitest-setup.js']
	}
}));
