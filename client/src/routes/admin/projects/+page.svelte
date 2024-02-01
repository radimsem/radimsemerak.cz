<script lang="ts">
    import type { ActionData } from './$types';
    
    export let data: ActionData;
</script>


<section>
    <article>
        <h1>Unable to handle action: {data?.err}</h1>
    </article>
</section>