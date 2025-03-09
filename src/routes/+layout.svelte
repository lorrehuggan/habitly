<script>
  import "$styles/app.css";
  import TitleBar from "$components/global/TitleBar.svelte";
  import SideBar from "$components/global/SideBar.svelte";
  import { Toaster } from "svelte-sonner";
  import InfoBar from "$components/global/InfoBar.svelte";
</script>

<div class="relative h-screen w-screen overflow-hidden rounded-xl">
  <TitleBar />
  <div class="relative grid h-full w-full grid-cols-12">
    <SideBar />
    <main class="relative col-span-11 w-full overflow-y-scroll p-6">
      <Toaster richColors />
      <slot />
    </main>
  </div>
</div>
