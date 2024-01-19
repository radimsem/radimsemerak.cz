<script lang="ts">
    // components
    import * as Card from '$lib/components/ui/card';
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
        <a href="#projects">
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
                        Hust: a SSG from Markdown files written in Rust
                    </Card.Title>
                    <Card.Description class="text-base">
                        Hust is a static site generator similar to <a href="https://gohugo.io/" target="_blank">Hugo</a>, but it's written in Rust. 
                        It allows you to convert Markdown files into static HTML websites. 
                        With Hust, you can create blogs, documentation, and other types of websites with ease...
                    </Card.Description>
                </Card.Content>
            </Card.Root>
        </a>
    </section>
</section>

<style>
    
</style>