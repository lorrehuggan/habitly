<script lang="ts">
  import Timeline from "$components/timeline/Component.svelte";
  import { onMount } from "svelte";
  import type { PageProps } from "./$types";

  const { data }: PageProps = $props();
  const timeline: Array<Array<string>> = $state([]);

  onMount(() => {
    timeline.push(data.timeline.days.Sun);
    timeline.push(data.timeline.days.Mon);
    timeline.push(data.timeline.days.Tue);
    timeline.push(data.timeline.days.Wed);
    timeline.push(data.timeline.days.Thu);
    timeline.push(data.timeline.days.Fri);
    timeline.push(data.timeline.days.Sat);
  });
</script>

{#each data.habits as habit}
  <Timeline {timeline} {habit} />
{/each}
