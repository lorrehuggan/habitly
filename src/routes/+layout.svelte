<script>
  import "$styles/app.css";
  import TitleBar from "$components/global/TitleBar.svelte";
  import SideBar from "$components/global/SideBar.svelte";
</script>

<div class="h-screen w-screen overflow-hidden rounded-xl">
  <TitleBar />
  <div class="grid h-full w-full grid-cols-12">
    <SideBar />
    <main class="col-span-11 w-full overflow-y-scroll">
      <slot />
    </main>
  </div>
</div>
