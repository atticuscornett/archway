<script lang="ts">
    import * as Card from "$lib/components/ui/card/index.js";
    import {Button} from "$lib/components/ui/button/index.js";
    import {ArrowRight} from "@lucide/svelte";
    import {onMount} from "svelte";
    import {toast} from "svelte-sonner";

    let {page = $bindable()} = $props();

    let jobList: Object[] = $state([]);

    let loadJobs = async () => {
        try {
            jobList = JSON.parse(await invoke("get_all_jobs")).slice(-3);
        } catch (error) {
            toast.error("Something went wrong while loading jobs.");
        }
    };

    onMount(()=> {
        loadJobs();
    });

</script>

<Card.Root class="w-full h-full mb-4 relative pb-8">
    <Card.Header>
        <Card.Title>Jobs</Card.Title>
        <Card.Description>Create, run, and edit jobs</Card.Description>
    </Card.Header>
    <Card.Content>
        {#each jobList as job}
            <h4>Latest Jobs</h4>
            <h6>{job["job_name"]}</h6>
        {/each}
    </Card.Content>
    <Button onclick={()=>{page="JobManager";}} class="align-middle absolute bottom-4 right-4"><ArrowRight/></Button>
</Card.Root>