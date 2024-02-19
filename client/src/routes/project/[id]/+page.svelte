<script lang="ts">
    import xss from "xss";
    import type { PageServerData } from "./$types";

    export let data: PageServerData;
</script>

<section class="relative main-sec p-[5%] text-slate-300 lg:px-[10%] xl:flex xl:justify-center">
    <article class="project-content space-y-5 xl:space-y-6">
        {#if data.project}
            {@html xss(data.project.html)}
        {:else}
            {data.err}
        {/if}
    </article>
</section>