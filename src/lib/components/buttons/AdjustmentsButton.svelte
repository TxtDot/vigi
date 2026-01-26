<script>
  import Adjustments from "$lib/icons/Adjustments.svelte";
  import { gotoTBI } from "$lib/utils";
  import Button from "../Button.svelte";
</script>

<Button className="top-bar-input-button" onclick={() => gotoTBI("browser://settings")}>
  <Adjustments />
</Button>
