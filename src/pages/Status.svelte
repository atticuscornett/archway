<script lang="ts">
    import {toast} from "svelte-sonner";

    let { page = $bindable() } = $props();
    import { onMount } from "svelte";
    import * as Card from "$lib/components/ui/card/index.js";
    import {Button} from "$lib/components/ui/button";
    import {Plus, Home, Pencil, Play, Trash2, SquareX} from "@lucide/svelte";
    import {Progress} from "$lib/components/ui/progress";

    let statusList: Object[] = $state([]);

    let loadJobs = async () => {
        if (page === "Status") {
            try {
                statusList = JSON.parse(await invoke("get_all_job_statuses"));
            }
            catch (e){
                toast.error("Failed to load job statuses.");
            }
            setTimeout(loadJobs, 5000); // Refresh every 5 seconds
        }
    }



    onMount(()=>{
        loadJobs();
    })
</script>

<h2 class="mb-5">Status</h2>
<div class="fixed top-10 right-10" >
    <Button><SquareX/> Clear Completed Statuses</Button>
    <Button onclick={() => page = "Dashboard"}><Home/> Back to Dashboard</Button>
</div>

{#if statusList.length === 0}
    <p>No jobs in progress.</p>
{:else}
    <div class="job-list">
        {#each statusList as status}
            <Card.Root class="relative">
                <div class={status ? "" : "text-red-600"}>
                    <Card.Header>
                        <Card.Title>{status["job"]["job_name"]}</Card.Title>
                        <Card.Description>Job UUID: {status["job"]["uuid"]}</Card.Description>
                    </Card.Header>
                    <Card.Content>
                        <h3>Step {status["step"]} of {status["total_steps"]}: {status["step_title"]}...</h3>
                        <h4 class="mb-2">{status["last_action"]}</h4>

                        <Progress value={status["percent"]}></Progress>
                    </Card.Content>
                    <div class="absolute top-4 right-4">
    <!--                    <Button class="mb-2"><Play/> Start Job</Button>-->
    <!--                    <br>-->
    <!--                    <Button class="mb-2" onclick={()=>{page="SetUpAutomation:"+status["job"]["uuid"]}}><Pencil/> Edit Job</Button>-->
                    </div>
                </div>
            </Card.Root>
        {/each}
    </div>
{/if}
