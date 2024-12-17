<script lang="ts">
  import ThemeToggle from "$lib/components/ThemeToggle.svelte";
  import {
    CommandDialog,
    CommandEmpty,
    CommandGroup,
    CommandInput,
    CommandItem,
    CommandList,
    CommandSeparator,
  } from "$lib/components/ui/command/index";
  import type { SearchAPI } from "$lib/api";

  let open: boolean = $state(false);
  let metaKeyPressed: boolean = false;
  let value = $state("");

  async function fetchResults($value: string) {
    const req = await fetch(`api/${$value}`);
    const res: SearchAPI = await req.json();

    return res.query.results;
  }

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
</script>

<svelte:head>
  <title>Toorust</title>
</svelte:head>

<svelte:document on:keydown={onKeyDown} on:keyup={onKeyUp} />

<div class="hero min-h-screen">
  <div class="hero-content">
    <div class="max-w-md text-center">
      <span>
        Search
        <button
          type="button"
          class="kbd cursor-pointer"
          onclick={() => (open = true)}
          onkeydown={(event) => {
            if (event.key === "Enter" || event.key === " ") {
              open = true;
            }
          }}
        >
          ⌘ X
        </button>
      </span>
      <h1 class="font-sans text-8xl font-black text-glow-foreground">
        Toolshed
      </h1>
      <h2 class="font-mono italic">tools that go → the point</h2>
      <div class="mt-6">
        <ThemeToggle />
      </div>
    </div>
  </div>
</div>

<CommandDialog bind:open shouldFilter={false}>
  <CommandInput
    placeholder="Type a command or search..."
    bind:value
  />
  <CommandList>
    <CommandEmpty>No results found.</CommandEmpty>
    <CommandGroup heading="Suggestions">
      <CommandItem>Calendar</CommandItem>
      <CommandItem>Search Emoji</CommandItem>
      <CommandItem>Calculator</CommandItem>
    </CommandGroup>
    <CommandSeparator />
    <CommandGroup heading="Settings">
      <CommandItem>Profile</CommandItem>
      <CommandItem>Billing</CommandItem>
      <CommandItem>Settings</CommandItem>
    </CommandGroup>
  </CommandList>
</CommandDialog>
