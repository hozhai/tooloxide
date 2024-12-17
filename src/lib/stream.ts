import { derived } from "svelte/store";

export default function (
  stores: any,
  callback: (arg0: unknown) => any,
  initial_value: any
) {
  let previous = 0;

  return derived(
    stores,
    ($stores, set) => {
      const start = Date.now();
      Promise.resolve(callback($stores)).then((value) => {
        if (start > previous) {
          previous = start;
          set(value);
        }
      });
    },
    initial_value
  );
}
