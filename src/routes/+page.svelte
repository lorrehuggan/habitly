<script lang="ts">
  import { onMount } from "svelte";
  import type { PageProps } from "./$types";
  let { data }: PageProps = $props();
  let timeline: Array<Array<string>> = $state([]);

  onMount(() => {
    timeline.push(data.days.Sun);
    timeline.push(data.days.Mon);
    timeline.push(data.days.Tue);
    timeline.push(data.days.Wed);
    timeline.push(data.days.Thu);
    timeline.push(data.days.Fri);
    timeline.push(data.days.Sat);
  });
</script>

<main class="container">
  <div>Walk</div>
  {#each timeline as tl}
    <div class="one">
      {#each tl as day}
        <div class="box" data-date={day}></div>
      {/each}
    </div>
  {/each}
</main>

<style>
  .one {
    padding-bottom: 4px;
    display: flex;
    gap: 4px;
  }
  .box {
    height: 12px;
    width: 12px;
    background-color: dodgerblue;
    border-radius: 2px;
  }
</style>
