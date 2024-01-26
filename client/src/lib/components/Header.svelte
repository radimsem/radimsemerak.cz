<script lang="ts">
    import { afterUpdate } from "svelte";

    // components
    import * as Sheet from "$lib/components/ui/sheet";
    import Navigation from "./layouts/Navigation.svelte";

    // images
    import logo from '$lib/images/logo.svg';

    // icons
    import Icon from '@iconify/svelte';
</script>

<section class="flex flex-row items-center justify-between">
    <figure>
        <img 
            src={logo} 
            alt="Logo Semerak"
            width="50"
            height="50"
        >
    </figure>
    <aside class="hidden lg:block">
        <Navigation direction="row" />
    </aside>
    <aside class="lg:hidden">
        <Sheet.Root closeFocus="">
            <Sheet.Trigger>
                <Icon icon="material-symbols:menu-rounded" width="30" height="30" class="text-slate-200" />
            </Sheet.Trigger>
            <Sheet.Content class="w-auto px-10 py-14 text-slate-300 bg-[#12181b] border-l-0">
                <Navigation direction="col" />
            </Sheet.Content>
        </Sheet.Root>
    </aside>
</section>