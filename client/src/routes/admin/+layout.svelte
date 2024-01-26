<script lang="ts">
    import { page } from "$app/stores";

    type AdminPageIdentifier = {
        title: string,
        sub: string
    };

    const root = "/admin";
    const path = $page.url.pathname;
    let pageIdentifier: AdminPageIdentifier;

    const capitalize = (str: string): string => str.charAt(0).toUpperCase() + str.substring(1);

    function getSlashIndexes(path: string): number[] {
        const arr: number[] = [];
        for (let i = 0; i < path.length; i++) {
            if (path[i] == '/') {
                arr.push(i);
            }
        }
        return arr;
    }

    if ($page.url.pathname == root) {
        pageIdentifier = { title: "Admin", sub: "Overview" };
    } else {
        const slashIndexes = getSlashIndexes(path);
        const title = capitalize(path.substring(root.length + 1, slashIndexes[2]));
        const sub = capitalize(path.substring(root.length + title.length + 2, slashIndexes[3] ? slashIndexes[3] : path.length));

        pageIdentifier = { title, sub };
    }
</script>

<div class="relative">
    <aside></aside>

    <div class="space-y-10 px-[5%] py-4 text-slate-300">
        <header class="flex items-center justify-between">
            <div class="space-y-1">
                <h2 class="text-2xl font-semibold text-slate-100">
                    {pageIdentifier.title}
                </h2>
                <h3>{pageIdentifier.sub}</h3>
            </div>
            <a href="/" target="_blank">
                <md-filled-button class="text-[.95rem]">
                    See web
                </md-filled-button>
            </a>
        </header>
    
        <main>
            <slot/>
        </main>
    </div>
</div>