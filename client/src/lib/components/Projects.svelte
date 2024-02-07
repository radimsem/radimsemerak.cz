<script lang="ts">
    // components
    import * as Card from '$lib/components/ui/card';
    import xss from 'xss';

    export let data: ProjectResponse[];
</script>

<section id="projects" class="main-sec">
    <article class="space-y-4">
        <h1 class="text-[1.7rem] heading">
            Projects
        </h1>
        <p>
            My open-source projects are primarily hosted on <a href="https://gitlab.com/radimsem">GitLab</a>, although I also maintain some on <a href="https://github.com/radimsem">GitHub</a>. 
            Both provide robust platforms for hosting and collaborating on open-source projects, 
            but they each have their unique strengths.
        </p>
    </article>
    <section class="grid grid-cols-1 gap-8 md:grid-cols-2 xl:grid-cols-3">
       {#each data as project}
            <a href={`project/${project.id}`}>
                <Card.Root class="h-full space-y-6 p-0 pb-3 bg-[rgb(32,39,55)] border-none transition-transform duration-300 hover:-translate-y-2">
                    <Card.Header class="p-0">
                        <img 
                            src="/src/lib/images/hust_post.png"
                            alt="Markdown Rust Convert to HTML"
                            height="250"
                            class="h-[250px] object-cover object-center rounded-t-xl"
                        >
                    </Card.Header>
                    <Card.Content class="space-y-4 px-5">
                        <Card.Title class="text-xl text-slate-200">
                            {project.title}
                        </Card.Title>
                        <Card.Description class="text-base">
                            {project.annotation}
                        </Card.Description>
                    </Card.Content>
                </Card.Root>
            </a>
       {/each}
    </section>
</section>