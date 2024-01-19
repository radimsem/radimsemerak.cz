<script lang="ts">
    import DataTable from './data-table.svelte';
</script>

<div class="relative">
    <aside></aside>

    <div class="space-y-10 px-[5%] py-4 text-slate-300">
        <header class="flex items-center justify-between">
            <div class="space-y-1">
                <h2 class="text-2xl font-semibold text-slate-100">
                    Admin
                </h2>
                <h3>Overview</h3>
            </div>
            <md-filled-button class="text-[.95rem]">
                <a href="/">See web</a>
            </md-filled-button>
        </header>
    
        <main>
            <section>
                <article>
                    <h1 class="text-[1.7rem] heading">
                        Projects overview
                    </h1>
                </article>
                <article>
                    <DataTable />
                </article>
            </section>
        </main>
    </div>
</div>

<style>

</style>