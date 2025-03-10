<script>
  import { deleteHabit } from "$actions/timeline";
  import Button from "$components/ui/Button.svelte";
  import DialogComponent from "$components/ui/DialogComponent.svelte";
  let { id, children } = $props();
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
      <Button intent="muted" size="button" aria-label="archive">Archive</Button>
      <Button intent="error" size="button" aria-label="delete" onclick={() => deleteHabit(id)}
        >Delete</Button
      >
    </div>
  {/snippet}
</DialogComponent>
