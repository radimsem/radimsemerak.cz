<script lang="ts">
    	// icons
	import Icon from '@iconify/svelte';
</script>

<section class="flex flex-col space-y-20 xl:justify-center xl:min-h-[80vh] xl:space-y-40">
    <section class="space-y-4 xl:space-y-6">
        <h1 class="text-3xl font-bold text-white lg:text-[2rem] xl:text-4xl">
            Open-source <span class="highlight">Full Stack</span> Developer
        </h1>
        <p class="lg:w-1/2 xl:w-1/4">
            High school student with passion for learning new things and improving own skills.
        </p>
        <div>
            <a href="#experience">
                <md-filled-button class="text-[.95rem] py-2">
                    Experience
                </md-filled-button>
            </a>
        </div>
    </section>

    <section>
        <article class="overflow-x-auto">
            <h2 class="text-xl heading">
                Tech stacks
            </h2>
            <table>
                <thead>
                    <tr>
                        <th></th>
                        <th>
                            <span class="icon-text">
                                <Icon icon="material-symbols:desktop-mac-outline" width="28" height="28" />
                                Desktop
                            </span>
                        </th>
                        <th>
                            <span class="icon-text">
                                <Icon icon="material-symbols:android" width="28" height="28" />
                                Mobile
                            </span>
                        </th>
                        <th>
                            <span class="icon-text">
                                <Icon icon="mdi:web" width="28" height="28" />
                                Web
                            </span>
                        </th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>Frontend</td>
                        <td>
                            <span class="icon-text">
                                <Icon icon="logos:svelte-icon" width="28" height="28" />
                                SvelteKit
                            </span>
                        </td>
                        <td>
                            <span class="icon-text">
                                <Icon icon="devicon:jetpackcompose" width="28" height="28" />
                                Jetpack Compose
                            </span>
                        </td>
                        <td>
                            <span class="icon-text">
                                <Icon icon="logos:svelte-icon" width="28" height="28" />
                                SvelteKit
                            </span>
                        </td>
                    </tr>
                    <tr>
                        <td>Backend</td>
                        <td>
                            <span class="icon-text">
                                <Icon icon="logos:tauri" width="28" height="28" />
                                Tauri
                            </span>
                        </td>
                        <td>
                            <span class="icon-text">
                                <Icon icon="logos:kotlin-icon" width="24" height="24" />
                                Kotlin
                            </span>
                        </td>
                        <td>
                            <span class="icon-text">
                                <Icon icon="vscode-icons:file-type-rust" width="32" height="32" />
                                Rust
                            </span>
                        </td>
                    </tr>
                </tbody>
            </table>
        </article>
    </section>
</section>

<style>
    tr {
		border-bottom: 1px solid #0284c7;
	}
	th {
		font-weight: normal;
	}
	th, td {
		padding: 1rem;
	}
</style>