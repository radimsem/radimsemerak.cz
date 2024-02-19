<script lang="ts">
    // icons
    import Icon from '@iconify/svelte';
</script>

<section id="contact" class="main-sec">
    <article>
        <h1 class="text-[1.7rem] heading">
            Contact
        </h1>
    </article>
    <article class="space-y-5 xl:space-y-6">
        <p>
            As a preferred contact option, I use <a href="https://matrix.org/" target="_blank">Matrix</a>, specifically the <a href="https://element.io/" target="_blank">Element</a> client. 
            Element is a cross-platform client that works on the Matrix protocol, which offers end-to-end encryption ensuring privacy of your conversations.
        </p>
        <div class="flex gap-3 md:flex-row">
            <a title="Matrix" href="https://matrix.to/#/@radimsem:matrix.org">
                <md-filled-tonal-button class="text-[.95rem] py-4">
                    <span class="flex items-center gap-3">
                        <Icon icon="simple-icons:matrix" width="24" height="24" />
                    </span>
                </md-filled-tonal-button>
            </a>
            <a title="Email" href="mailto:semerak@radimsemerak.cz">
                <md-filled-tonal-button class="text-[.95rem] py-4">
                    <span class="flex items-center gap-3">
                        <Icon icon="material-symbols:mail-rounded" width="24" height="24" />
                    </span>
                </md-filled-tonal-button>
            </a>
            <a href="/src/lib/docs/publickey.asc">
                <md-filled-tonal-button class="text-[.95rem] py-4">
                    <span class="flex items-center gap-2">
                        <Icon icon="material-symbols:key" width="24" height="24" />
                        PGP
                    </span>
                </md-filled-tonal-button>
            </a>
        </div>
    </article>
</section>