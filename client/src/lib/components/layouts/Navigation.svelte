<script lang="ts">
    // icons
    import Icon from '@iconify/svelte';

    export let direction: "col" | "row";
</script>

<nav>
    <ul class={`flex ${direction == "col" ? `flex-col gap-7` : `flex-row gap-10`} items-center text-[.9rem]`}>
        <li>
            <a href="/" class="nav-link group/nav-item">
                <figure class="nav-icon-figure bg-indigo-600">
                    <Icon icon="material-symbols:home" width="24" height="24" />
                </figure>
                Home
            </a>
        </li>
        <li>
            <a href="#projects" class="nav-link group/nav-item">
                <figure class="nav-icon-figure">
                    <Icon icon="mdi:application-cog" width="22" height="22" class="text-slate-300 group-hover/nav-item:text-white" />
                </figure>
                Projects
            </a>
        </li>
        <li>
            <a href="#contact" class="nav-link group/nav-item">
                <figure class="nav-icon-figure">
                    <Icon icon="material-symbols:business-messages" width="24" height="24" class="text-slate-300 group-hover/nav-item:text-white" />
                </figure>
                Contact
            </a>
        </li>
    </ul>
</nav>

<style>

</style>