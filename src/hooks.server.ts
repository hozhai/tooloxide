import type { Handle } from "@sveltejs/kit";
import { i18n } from "$lib/i18n";

/** @type {import('@sveltejs/kit').Handle} */
export const handle: Handle = i18n.handle();
