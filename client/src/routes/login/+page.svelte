<script lang="ts">
    import type { ActionData } from './$types';

    // components
    import * as Card from '$lib/components/ui/card';

    // images
    import logo from '$lib/images/logo.svg';

    // icons
    import Icon from '@iconify/svelte';

    export let form: ActionData;

    let showPass = false;
</script>

<main class="relative">
    <section class="flex justify-center items-center w-full h-screen">
        <Card.Root class="space-y-2 {form?.err && "space-y-0"} px-2 pb-3 bg-[rgb(32,39,55)] bg-opacity-50 border-none">
            <Card.Header class="items-center">
                <img 
                    src={logo}
                    alt="Logo Semerak"
                    width="70"
                    height="70"
                >
            </Card.Header>
            <Card.Content class="space-y-4 px-5">
                <form
                    action="?/login"
                    method="post"
                    class="flex flex-col gap-3"
                >
                    {#if form?.err}
                         <p class="text-center text-red-500">{form.err}</p>
                    {/if}
                    <fieldset class="flex flex-col gap-1">
                        <label for="username" class="text-sm text-slate-300">
                            Username
                        </label>
                        <input 
                            type="text"
                            id="username"
                            name="username"
                            placeholder="Admin username"
                            required
                            class="admin-input"
                        >
                    </fieldset>
                    <fieldset class="flex flex-col gap-1">
                        <label for="username" class="text-sm text-slate-300">
                            Password
                        </label>
                        <div class="relative">
                            <input 
                                type={showPass ? "text" : "password"}
                                id="password"
                                name="pw"
                                placeholder="Admin password"
                                required
                                class="admin-input"
                            >
                            <!-- svelte-ignore a11y-click-events-have-key-events -->
                            <!-- svelte-ignore a11y-no-static-element-interactions -->
                            <span
                                on:click={() => showPass = !showPass}
                                class="cursor-pointer absolute top-1/2 right-3 -translate-y-1/2 text-sky-500"
                            >
                                <Icon 
                                    icon={showPass ? "mdi:eye" : "mdi:eye-off"}
                                />
                            </span>
                        </div>
                    </fieldset>
                    <md-filled-button type class="text-[.95rem] mt-1.5 py-3">
                        Login
                    </md-filled-button>
                </form>
            </Card.Content>
        </Card.Root>
    </section>
</main>