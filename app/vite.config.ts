import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';
import wasmPack from "vite-plugin-wasm-pack";
import { websocketServer } from './src/lib/socket/plugin';

export default defineConfig({
	plugins: [sveltekit(), wasmPack("../core_heart"), websocketServer],
	test: {
		include: ['src/**/*.{test,spec}.{js,ts}']
	}
});
