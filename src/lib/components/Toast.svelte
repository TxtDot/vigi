<script lang="ts">
  import { temporal, dismissToast } from "$lib/state.svelte";
  import X from "$lib/icons/X.svelte";
  import Button from "./Button.svelte";
</script>

{#if temporal.toasts.length > 0}
  <div class="toast-container">
    {#each temporal.toasts as toast (toast.id)}
      <div class="toast toast-{toast.type}">
        <span class="toast-message">{toast.message}</span>
        <Button
          className="toast-dismiss"
          onclick={() => dismissToast(toast.id)}
        >
          <X width="16" height="16" />
        </Button>
      </div>
    {/each}
  </div>
{/if}
