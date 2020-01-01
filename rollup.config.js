import resolve from "rollup-plugin-node-resolve";
import replace from "rollup-plugin-replace";
import commonjs from "rollup-plugin-commonjs";
import svelte from "rollup-plugin-svelte";
import { terser } from "rollup-plugin-terser";
import config from "sapper/config/rollup.js";
import pkg from "./package.json";
import preprocess from "svelte-preprocess";
import includepaths from "rollup-plugin-includepaths";

const mode = process.env.NODE_ENV;
const dev = mode === "development";
const legacy = !!process.env.SAPPER_LEGACY_BUILD;

const onwarn = (warning, onwarn) => (warning.code === "CIRCULAR_DEPENDENCY" && /[/\\]@sapper[/\\]/.test(warning.message)) || onwarn(warning);
const dedupe = importee => importee === "svelte" || importee.startsWith("svelte/");

let includepaths_options = {
	include: {},
	paths: ["./src"],
	external: [],
	extensions: [".svelte", ".js"]
};

export default {
	client: {
		input: config.client.input(),
		output: config.client.output(),
		plugins: [
			replace({
				"process.browser": true,
				"process.env.NODE_ENV": JSON.stringify(mode)
			}),
			svelte({
				preprocess: [
					preprocess({
						scss: true,
						postcss: {
							plugins: [require("autoprefixer")({ overrideBrowserslist: '> 1%' })],
						},
					}),
				],
				dev,
				hydratable: true,
				emitCss: true
			}),
			resolve({
				browser: true,
				dedupe
			}),
			commonjs(),
			includepaths(includepaths_options),
			!dev && terser({
				module: true
			})
		],

		onwarn,
	},

	server: {
		input: config.server.input(),
		output: config.server.output(),
		plugins: [
			replace({
				"process.browser": false,
				"process.env.NODE_ENV": JSON.stringify(mode)
			}),
			svelte({
				preprocess: [
					preprocess({
						scss: true,
						postcss: {
							plugins: [require("autoprefixer")({ overrideBrowserslist: '> 1%' })],
						},
					}),
				],
				generate: "ssr",
				dev
			}),
			resolve({
				dedupe
			}),
			commonjs(),
			includepaths(includepaths_options)
		],
		external: Object.keys(pkg.dependencies).concat(
			require("module").builtinModules || Object.keys(process.binding("natives"))
		),

		onwarn,
	},

	serviceworker: {
		input: config.serviceworker.input(),
		output: config.serviceworker.output(),
		plugins: [
			resolve(),
			replace({
				"process.browser": true,
				"process.env.NODE_ENV": JSON.stringify(mode)
			}),
			commonjs(),
			!dev && terser()
		],

		onwarn,
	}
};
