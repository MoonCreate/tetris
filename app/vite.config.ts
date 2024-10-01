import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';
import wasmPack from "vite-plugin-wasm-pack";

export default defineConfig({
	plugins: [sveltekit(), wasmPack("../core_heart")],
	test: {
		include: ['src/**/*.{test,spec}.{js,ts}']
	}
});
