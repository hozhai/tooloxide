import eslintPluginSvelte from "eslint-plugin-svelte";
export default [
  ...eslintPluginSvelte.configs.recommended,
  ...eslintPluginSvelte.configs.prettier,
  {
    rules: {},
  },
];
