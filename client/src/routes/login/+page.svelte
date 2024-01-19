<script lang="ts">
    // components
    import * as Card from '$lib/components/ui/card';

    // images
    import logo from '$lib/images/logo.svg';

    // icons
    import Icon from '@iconify/svelte';

    let showPass = false;
</script>

<main class="relative">
    <section class="flex justify-center items-center w-full h-screen">
        <Card.Root class="space-y-2 px-2 pb-3 bg-[rgb(32,39,55)] bg-opacity-50 border-none">
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
                            class="px-3 py-2.5 text-slate-300 bg-[rgb(18,24,27)] bg-opacity-75 placeholder:text-slate-500 rounded-[.5rem] outline-none transition-colors focus:bg-opacity-100"
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
                                class="px-3 py-2.5 text-slate-300 bg-[rgb(18,24,27)] bg-opacity-75 placeholder:text-slate-500 rounded-[.5rem] outline-none transition-colors focus:bg-opacity-100"
                            >
                            <button
                                on:click={(e) => {
                                    e.preventDefault();
                                    showPass = !showPass
                                }}
                                class="cursor-pointer absolute top-1/2 right-3 -translate-y-1/2 text-sky-500"
                            >
                                <Icon 
                                    icon={showPass ? "mdi:eye" : "mdi:eye-off"}
                                />
                            </button>
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

<style>

</style>