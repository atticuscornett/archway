<script lang="ts">
    import * as Card from "$lib/components/ui/card/index.js";
    import {Button} from "$lib/components/ui/button/index.js";
    import {ArrowRight, CircleCheck, CircleHelp, CircleX} from "@lucide/svelte";
    import {onMount} from "svelte";
    import {toast} from "svelte-sonner";

    let {page = $bindable()} = $props();

    let health: Object = $state({});
    let healthDetails: string = $state("Loading job health...");
    let healthOverview: string = $state("unknown");

    let loadHealth = async () => {
        if (page !== "Dashboard") {
            return;
        }

        try {
            health = await invoke("get_all_job_health");
        } catch (error) {
            toast.error("Something went wrong while loading job health.");
        }

        let allJobs = JSON.parse(await invoke("get_all_jobs"));
        let unknown_jobs = allJobs.length;
        console.log("Unknown jobs:", unknown_jobs);
        let healthy_jobs = 0;
        let unhealthy_jobs = 0;
        for (let job of allJobs) {
            if (!health[job.uuid]) {
                continue; // Skip if health data is not available for this job
            }
            if (health[job.uuid].includes("good")) {
                healthy_jobs++;
            }
            if (health[job.uuid].includes("bad")) {
                unhealthy_jobs++;
            }
        }

        unknown_jobs -= (healthy_jobs + unhealthy_jobs);

        if (unhealthy_jobs > 0) {
            healthOverview = "unhealthy";
            healthDetails = `${unhealthy_jobs} job${unhealthy_jobs == 1 ? '' : 's'} unhealthy, ${healthy_jobs} job${healthy_jobs == 1 ? '' : 's'} healthy, ${unknown_jobs} job${unknown_jobs == 1 ? '' : 's'} unknown`;
        } else {
            healthOverview = "healthy";
            healthDetails = `${healthy_jobs} job${healthy_jobs == 1 ? '' : 's'} healthy, ${unknown_jobs} job${unknown_jobs == 1 ? '' : 's'} unknown`;
        }

        setTimeout(loadHealth, 15000); // Refresh every 15 seconds
    };

    onMount(()=> {
        loadHealth();
    });

</script>

<Card.Root class="w-full h-full mb-4 relative pb-8">
    <Card.Content>
        <div class="inline-block align-top">
            {#if healthOverview === "unhealthy"}
                <CircleX size={80} class="text-red-600"/>
            {:else if healthOverview === "unknown"}
                <CircleHelp size={80} class="text-yellow-400"/>
            {:else}
                <CircleCheck size={80} class="text-green-400"/>
            {/if}
        </div>
        <div class="inline-block ml-4">
            {#if healthOverview === "unhealthy"}
                <h2 class="text-red-600">Some jobs are unhealthy!</h2>
            {:else if healthOverview === "unknown"}
                <h2 class="text-yellow-400">Job health is unknown.</h2>
            {:else}
                <h2 class="text-green-400">All jobs are healthy!</h2>
            {/if}
            <h3>{healthDetails}</h3>
        </div>
    </Card.Content>
    <Button onclick={()=>{page="Health";}} class="align-middle absolute bottom-4 right-4"><ArrowRight/></Button>
</Card.Root>