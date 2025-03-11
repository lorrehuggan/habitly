<script lang="ts">
  import type { PageProps } from "./$types";
  import TrashIcon from "$components/icons/TrashIcon.svelte";
  import Calendar from "$components/icons/Calendar.svelte";
  import DeleteHabit from "$components/global/DeleteHabit.svelte";
  import Button from "$components/ui/Button.svelte";
  import PenIcon from "$components/icons/PenIcon.svelte";
  let { data }: PageProps = $props();
</script>

<div>
  {#if data.habit}
    <div class="flex-center justify-between">
      <div>
        <h1 class="text-2xl">{data.habit.title}</h1>
        <h3 class="text-neutral-400">{data.habit.description}</h3>
      </div>
      <div class="flex-center gap-2">
        <Button intent="primary" size="icon">
          <PenIcon />
        </Button>
        <Button intent="primary" size="icon">
          <Calendar />
        </Button>
        <DeleteHabit id={data.habit.id}>
          <Button intent="danger" size="icon">
            <TrashIcon />
          </Button>
        </DeleteHabit>
      </div>
    </div>
  {/if}
</div>
