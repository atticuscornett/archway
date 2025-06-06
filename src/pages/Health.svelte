<script lang="ts">
    import {toast} from "svelte-sonner";

    let {page = $bindable()} = $props();
    import {onMount} from "svelte";
    import {ArrowRight, CircleCheck, CircleHelp, CircleX, Home} from "@lucide/svelte";
    import {Button} from "$lib/components/ui/button";
    import * as Card from "$lib/components/ui/card/index.js";

    let health: Object = $state({});
    let healthDetails: string = $state("Loading job health...");
    let healthOverview: string = $state("unknown");
    let allJobs: Object[] = $state([]);

    let loadHealth = async () => {
        if (page !== "Health") {
            return;
        }

        try {
            health = await invoke("get_all_job_health");
        } catch (error) {
            toast.error("Something went wrong while loading job health.");
        }

        allJobs = JSON.parse(await invoke("get_all_jobs"));
        let unknown_jobs = allJobs.length;
        console.log("Unknown jobs:", unknown_jobs);
        let healthy_jobs = 0;
        let unhealthy_jobs = 0;
        for (let job in health) {
            if (health[job].includes("good")) {
                healthy_jobs++;
            }
            if (health[job].includes("bad")) {
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
<h2 class="mb-5">Job Health</h2>
<Button onclick={() => page = "Dashboard"} class="fixed top-10 right-10 z-50"><Home/> Back to Dashboard</Button>
<Card.Root class="w-full h-full mb-4 relative">
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
            <h3 class="mb-2">{healthDetails}</h3>
        </div>
    </Card.Content>
</Card.Root>

{#each allJobs as job}
    <Card.Root class="relative mb-4">
        <Card.Header>
            <Card.Title>{job["job_name"]}</Card.Title>
            <Card.Description>Job UUID: {job["uuid"]}</Card.Description>
        </Card.Header>
        <Card.Content>
            {#if health[job["uuid"]] ? health[job["uuid"]].split("/")[0] : "unknown" == "good"}
                <CircleCheck class="align-bottom inline text-green-400"/>
            {:else if health[job["uuid"]] ? health[job["uuid"]].split("/")[0] : "unknown" == "bad"}
                <CircleX class="align-bottom inline text-red-600"/>
            {:else}
                <CircleHelp class="align-bottom inline text-yellow-400"/>
            {/if}

            <p class="inline">Last Health Update: {health[job["uuid"]] ? health[job["uuid"]].split("/")[1] : "Job has never run."}</p>
        </Card.Content>
    </Card.Root>
{/each}