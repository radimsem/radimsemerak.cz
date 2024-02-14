<script lang="ts">
    import { createRender, createTable, Render, Subscribe } from 'svelte-headless-table';
    import { readable } from 'svelte/store';
    import * as Table from '$lib/components/ui/table';
    import DataTableActions from './data-table-actions.svelte';

    export let data: ProjectResponse[];

    const table = createTable(readable(data));
    const columns = table.createColumns([
        table.column({
            accessor: "id",
            header: "ID"
        }),
        table.column({
            accessor: "title",
            header: "Title"
        }),
        table.column({
            accessor: (item) => item,
            header: "",
            cell: ({ value }) => {
                return createRender(
                    DataTableActions, 
                    { assets: { id: value.id.toString(), title: value.title } }
                );
            }
        })
    ]);

    const { headerRows, pageRows, tableAttrs, tableBodyAttrs } =
        table.createViewModel(columns);
</script>

<div class="bg-[rgb(32,39,55)] bg-opacity-50 rounded-xl border border-slate-500">
    <Table.Root {...$tableAttrs}>
        <Table.Header>
        {#each $headerRows as headerRow}
            <Subscribe rowAttrs={headerRow.attrs()}>
            <Table.Row>
                {#each headerRow.cells as cell (cell.id)}
                <Subscribe attrs={cell.attrs()} let:attrs props={cell.props()}>
                    <Table.Head {...attrs}>
                    <Render of={cell.render()} />
                    </Table.Head>
                </Subscribe>
                {/each}
            </Table.Row>
            </Subscribe>
        {/each}
        </Table.Header>
        <Table.Body {...$tableBodyAttrs}>
        {#each $pageRows as row (row.id)}
            <Subscribe rowAttrs={row.attrs()} let:rowAttrs>
            <Table.Row {...rowAttrs}>
                {#each row.cells as cell (cell.id)}
                <Subscribe attrs={cell.attrs()} let:attrs>
                    <Table.Cell {...attrs}>
                    <Render of={cell.render()} />
                    </Table.Cell>
                </Subscribe>
                {/each}
            </Table.Row>
            </Subscribe>
        {/each}
        </Table.Body>
    </Table.Root>
</div>

<style>

</style>