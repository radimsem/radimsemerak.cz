<script lang="ts">
    import { onMount } from "svelte";
    import { encode, decode } from "html-entities";

    let encoded: string;

    onMount(() => encoded = encode("semerak@radimsemerak.cz", {
        mode: "extensive",
    }));
</script>

<section id="contact" class="main-sec">
    <article>
        <h1 class="text-[1.7rem] heading">
            Contact
        </h1>
    </article>
    <article class="space-y-5 xl:space-y-6">
        <ul class="contact-options space-y-2">
            <li>Write me an email: 
                <a 
                    href={`mailto:${encoded}`}
                    on:click={async (e) => {
                        e.preventDefault();
                        window.location.href = `mailto:${decode(encoded)}`;
                    }}
                >adress</a>
            </li>
            <li>You can encrypt by my <a href="$lib/docs/publickey.asc">PGP</a> public key</li>
            <li>Send me a message: <a href="https://matrix.to/#/@radimsem:matrix.org">Matrix</a></li>
        </ul>
    </article>
</section>