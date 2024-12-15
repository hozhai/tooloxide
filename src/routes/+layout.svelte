<script lang="ts">
  import { i18n } from "$lib/i18n";
  import { ParaglideJS } from "@inlang/paraglide-sveltekit";
  import "@fontsource-variable/space-grotesk";
  import "@fontsource/space-mono";
  import "../app.css";
  let { children } = $props();
</script>

<ParaglideJS {i18n}>
  {@render children()}
</ParaglideJS>
