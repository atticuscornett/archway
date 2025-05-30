<script lang="ts">
    import * as Card from "$lib/components/ui/card/index.js";
    import {Button} from "$lib/components/ui/button/index.js";
    import {ArrowRight} from "@lucide/svelte";
    import {onMount} from "svelte";
    import {toast} from "svelte-sonner";

    let {page = $bindable()} = $props();

    let jobsInProgress: number = $state(0);

</script>

<Card.Root class="w-full h-full mb-4 relative pb-8">
    <Card.Header>
        <Card.Title>Progress</Card.Title>
        <Card.Description>View progress of currently running jobs</Card.Description>
    </Card.Header>
    <Card.Content>
        <h4>
            {#if jobsInProgress > 0}
                {jobsInProgress} job{jobsInProgress > 1 ? 's' : ''} in progress
            {:else}
                No jobs currently running
            {/if}
        </h4>
    </Card.Content>
    <Button class="align-middle absolute bottom-4 right-4" onclick={()=>{page = "Status";}}><ArrowRight/></Button>
</Card.Root>