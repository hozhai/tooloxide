<script lang="ts">
  import ThemeToggle from "$lib/components/ThemeToggle.svelte";
  import Search from "$lib/components/Search.svelte";
</script>

<svelte:head>
  <title>Toorust</title>
</svelte:head>

<div class="hero min-h-screen">
  <div class="hero-content">
    <div class="max-w-md text-center">
      <Search />
      <h1 class="font-sans text-8xl font-black text-glow-foreground">
        Tooloxide
      </h1>
      <h2 class="font-mono italic">tools that go → the point</h2>
      <div class="mt-6">
        <ThemeToggle />
      </div>
    </div>
  </div>
</div>
