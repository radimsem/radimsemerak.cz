<script lang="ts">
    import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
    import { Button } from "$lib/components/ui/button";
    import * as Dialog from "$lib/components/ui/dialog";

    // icons
    import Icon from "@iconify/svelte";

    export let id: string;
</script>

<DropdownMenu.Root>
    <DropdownMenu.Trigger asChild let:builder>
        <Button
            variant="ghost"
            builders={[builder]}
            size="icon"
            class="relative w-8 h-8 p-0"
        >
        <Icon 
            icon="mdi:dots-vertical" 
            width="20"
            height="20"
        /> 
        </Button>
    </DropdownMenu.Trigger>
    <DropdownMenu.Content class="p-3 text-slate-300 bg-[rgb(32,39,55)] border-none rounded-xl z-10">
        <a href={`admin/projects/update/${id}`} class="outline-none">
            <DropdownMenu.Item>
                <Icon icon="mdi:edit" width="18" />
                Edit
            </DropdownMenu.Item>
        </a>
        <DropdownMenu.Separator class="bg-slate-700" />
        <Dialog.Root>
            <Dialog.Trigger>
                <Button class="flex gap-2 pl-2">
                    <Icon icon="mdi:trash-can-outline" width="18" />
                    Delete
                </Button>
            </Dialog.Trigger>
        <Dialog.Content class="text-slate-300 bg-[#12181b] border border-slate-700 !rounded-xl z-[100]">
            <Dialog.Header class="space-y-5">
                <Dialog.Title class="text-start">Do you want to delete project {id}?</Dialog.Title>
                <form 
                    method="post" 
                    action="/admin/projects"
                    enctype="multipart/form-data"
                >
                    <Button 
                        type="submit"
                        variant="destructive"
                        class="px-5 bg-[#7f1d1d] rounded-full hover:bg-[#731b1b]"
                    >Delete</Button>
                    <input type="hidden" name="id" value={id}>
                    <input type="hidden" name="action" value="delete">
                </form>
            </Dialog.Header>
        </Dialog.Content>
        </Dialog.Root>
    </DropdownMenu.Content>
</DropdownMenu.Root>