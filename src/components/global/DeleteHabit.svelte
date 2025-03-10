<script>
  import { deleteHabit } from "$actions/timeline";
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
      <button class="cursor-pointer" aria-label="archive">Archive</button>
      <button class="cursor-pointer" aria-label="delete" onclick={() => deleteHabit(id)}
        >Delete</button
      >
    </div>
  {/snippet}
</DialogComponent>
