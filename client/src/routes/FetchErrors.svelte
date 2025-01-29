<script lang="ts">
  import { goto } from "$app/navigation";
  import type { FailedResp } from "@cocreators-ee/apity/dist/svelte/types";
  import { Alert } from "flowbite-svelte";

  let { admin = true }: { admin?: boolean } = $props();
  let error = $state(false);
  let user_error = $state(false);

  export const check = (resp: FailedResp | undefined) => {
    if (resp) {
      if (resp.status == 401) {
        if (admin) goto("/admin/login");
        else {
          user_error = true;
        }
      } else {
        error = true;
      }
    }
  };
</script>

{#if error}
  <div class="w-full mt-24 flex items-center justify-center gap-4">
    <div
      class="animate-bounce rounded-full h-8 w-8 border-t-2 border-b-2 border-red-600"
    ></div>

    <Alert>Error while fetching</Alert>
  </div>
{/if}
{#if user_error}
  <div class="w-full mt-24 flex items-center justify-center gap-4">
    <div
      class="animate-bounce rounded-full h-8 w-8 border-t-2 border-b-2 border-red-600"
    ></div>

    <Alert>Bitte folgen Sie dem Link der E-Mail.</Alert>
  </div>
{/if}
