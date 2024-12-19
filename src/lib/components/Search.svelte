<script lang="ts">
  import {
    CommandDialog,
    CommandEmpty,
    CommandGroup,
    CommandInput,
    CommandItem,
    CommandList,
    CommandSeparator,
  } from "$lib/components/ui/command/index";

  let metaKeyPressed: boolean = false;
  let open = $state<boolean>(false);
  let value = $state<string>("");

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

  // TODO make api fetch calls :)
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
