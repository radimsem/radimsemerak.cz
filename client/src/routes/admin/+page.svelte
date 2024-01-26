<script lang="ts">
    import type { PageServerData } from './$types';

    // components
    import DataTable from './data-table.svelte';

    export let data: PageServerData;
</script>

<section class="space-y-6">
    <article>
        <h1 class="text-[1.7rem] heading">
            Projects overview
        </h1>
    </article>
    <article>
        <DataTable data={data.projects} />
    </article>
</section>