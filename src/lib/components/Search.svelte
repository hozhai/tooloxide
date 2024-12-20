<script lang="ts">
  import { resolveRoute } from "$app/paths";
  import type { SearchAPI } from "$lib/api";
  import {
    CommandDialog,
    CommandEmpty,
    CommandGroup,
    CommandInput,
    CommandItem,
    CommandList,
  } from "$lib/components/ui/command/index";
  import { derived, writable, type Readable } from "svelte/store";
  import CommandSeparator from "./ui/command/command-separator.svelte";

  // keep track of command open state and condition
  let metaKeyPressed: boolean = false;
  let open = $state<boolean>(false);

  // keep track of search input value as writable store
  let value = writable("");

  // event handler for keybind shortcuts
  function onKeyDown(event: KeyboardEvent) {
    if (event.repeat) return;

    if (event.key == "Meta" || event.key == "Control") {
      metaKeyPressed = true;
    }

    if (event.key == "x" && metaKeyPressed) {
      open = true;
    }
  }

  function onKeyUp(event: KeyboardEvent) {
    if (event.key == "Meta" || event.key == "Control") {
      metaKeyPressed = false;
    }
  }

  // fetch api endpoint, parse result and set it to results
  const results: Readable<SearchAPI> = derived(
    value,
    ($value, set) => {
      fetch(`api/search?q=${$value}`)
        .then((res) => res.json())
        .then((data: SearchAPI) => set(data));
    }
  );

  // TODO change ⌘ for ^ depending on platform
</script>

<svelte:document onkeyup={onKeyUp} onkeydown={onKeyDown} />

<span>
  Search &nbsp;
  <button
    type="button"
    class="kbd cursor-pointer"
    onclick={() => (open = true)}
    onkeydown={(event: KeyboardEvent) => {
      // accessibility
      if (event.key === "Enter" || event.key === " ") {
        open = true;
      }
    }}
  >
    ⌘ X
  </button>
</span>

<CommandDialog bind:open shouldFilter={false}>
  <CommandInput
    placeholder="Type a command or search..."
    bind:value={$value}
  />
  <CommandList>
    <CommandGroup heading="Results">
      <CommandEmpty>No results found.</CommandEmpty>
      {#if $results}
        {#each $results.query.results as result}
          <CommandItem>{result.name}</CommandItem>
        {/each}
      {/if}
    </CommandGroup>
  </CommandList>
</CommandDialog>
