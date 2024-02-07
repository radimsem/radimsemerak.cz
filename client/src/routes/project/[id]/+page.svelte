<script lang="ts">
    import xss from "xss";
    import type { PageServerData } from "./$types";

    export let data: PageServerData;
</script>

<section class="relative main-sec text-slate-200">
    <article>
        {#if data.project}
            {@html xss(data.project.html)}
        {:else}
            {data.err}
        {/if}
    </article>
</section>