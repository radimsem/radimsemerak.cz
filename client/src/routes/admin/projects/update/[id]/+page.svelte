<script lang="ts">
    import * as Card from "$lib/components/ui/card/";
    import type { PageData } from "./$types";

    export let data: PageData;
</script>

<section class="relative">
    <article>
        <Card.Root class="space-y-2 px-2 pt-5 pb-2 bg-[rgb(32,39,55)] bg-opacity-50 border-none">
            <Card.Content class="space-y-4 px-5">
                <form
                    action="/admin/projects"
                    method="post"
                    enctype="multipart/form-data"
                    class="flex flex-col gap-3"
                >
                    <fieldset class="flex flex-col gap-1">
                        <label for="file" class="text-sm text-slate-300">
                            Markdown file
                        </label>
                        <input
                            type="file"
                            id="file"
                            name="file"
                            required
                            class="admin-input"
                        >
                    </fieldset>
                    <md-filled-button type class="text-[.95rem] mt-1.5 py-3">
                        Update
                    </md-filled-button>
                    <input type="hidden" name="id" value={data.id}>
                    <input type="hidden" name="action" value="update">
                </form>
            </Card.Content>
        </Card.Root>
    </article>
</section>