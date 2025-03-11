<script lang="ts">
  import { archiveHabit, deleteHabit } from "$actions/timeline";
  import ArchiveIcon from "$components/icons/ArchiveIcon.svelte";
  import TrashIcon from "$components/icons/TrashIcon.svelte";
  import Button from "$components/ui/Button.svelte";
  import DialogComponent from "$components/ui/DialogComponent.svelte";
  let { id, children, archive }: { archive?: boolean; id: string; children?: () => any } = $props();
</script>

<DialogComponent
  title="Are you sure you want to delete?"
  description="This action cannot be undone"
>
  {#snippet trigger()}
    {@render children?.()}
  {/snippet}
  {#snippet body()}
    <div class="flex-center mt-8 justify-end gap-2">
      {#if !archive}
        <Button intent="muted" size="button" aria-label="archive" onclick={() => archiveHabit(id)}
          ><ArchiveIcon /> Archive</Button
        >
      {/if}
      <Button intent="danger" size="button" aria-label="delete" onclick={() => deleteHabit(id)}
        ><TrashIcon /> Delete</Button
      >
    </div>
  {/snippet}
</DialogComponent>
