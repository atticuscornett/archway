<script lang="ts">
    import {toast} from "svelte-sonner";

    let { page = $bindable() } = $props();
    import { onMount } from "svelte";
    import * as Card from "$lib/components/ui/card/index.js";
    import {Button} from "$lib/components/ui/button";
    import {Plus, Home, Pencil, Play, Trash2, SquareX, CircleX, CircleCheck, Pause, Octagon} from "@lucide/svelte";
    import {Progress} from "$lib/components/ui/progress";

    let statusList: Object[] = $state([]);
    let updateList = $state({});

    let loadJobs = async () => {
        if (page === "Status") {
            try {
                statusList = JSON.parse(await invoke("get_all_job_statuses"));
                for (let job of statusList){
                    updateList[job["job"]["uuid"]] = await invoke("get_job_update", {uuid: job["job"]["uuid"]});
                }
            }
            catch (e){
                console.error("Error loading job statuses:", e);
                toast.error("Failed to load job statuses.");
            }
            setTimeout(loadJobs, 2500); // Refresh every 2.5 seconds
        }
    }

    let clearCompletedStatuses = async () => {
        await invoke("clear_completed_jobs");
        try {
            statusList = JSON.parse(await invoke("get_all_job_statuses"));
        }
        catch (e){
            toast.error("Failed to load job statuses.");
        }
    };

    let pauseJob = async (jobUuid: string) => {
        await invoke("pause_job", {uuid: jobUuid});
        updateList[jobUuid] = "pause_requested";
    };

    let resumeJob = async (jobUuid: string) => {
        await invoke("unpause_job", {uuid: jobUuid});
        updateList[jobUuid] = "running";
    };

    let stopJob = async (jobUuid: string) => {
        await invoke("stop_job", {uuid: jobUuid});
        updateList[jobUuid] = "stop_requested";
    };



    onMount(()=>{
        loadJobs();
    })
</script>

<h2 class="mb-5">Status</h2>
<div class="fixed top-10 right-10 z-50" >
    <Button onclick={clearCompletedStatuses}><SquareX/> Clear Completed Statuses</Button>
    <Button onclick={() => page = "Dashboard"}><Home/> Back to Dashboard</Button>
</div>

{#if statusList.length === 0}
    <p>No jobs in progress.</p>
{:else}
    <div class="job-list">
        {#each statusList as status}
            <Card.Root class="relative mb-4">
                <div class={status["success"] ? "" : "text-red-600"}>
                    <Card.Header>
                        <Card.Title>{status["job"]["job_name"]}</Card.Title>
                        <Card.Description>Job UUID: {status["job"]["uuid"]}</Card.Description>
                    </Card.Header>
                    <Card.Content>
                        {#if status["completed"]}
                            {#if status["success"]}
                                <CircleCheck class="absolute top-4 right-4 text-green-400"/>
                                <h3>Job complete.</h3>
                            {:else}
                                <CircleX class="absolute top-4 right-4"/>
                                <h3>Job failed. </h3>
                                <h4>{status["last_action"]}</h4>
                            {/if}
                        {:else}
                            <h3>Step {status["step"]} of {status["total_steps"]}: {status["step_title"]}...</h3>
                            <h4 class="mb-2 break-all">{status["last_action"]}</h4>

                            <Progress value={status["percent"]*100}></Progress>
                        {/if}
                    </Card.Content>
                    {#if !status["completed"]}
                        <div class="absolute top-4 right-4">
                            {#if updateList[status["job"]["uuid"]] === "running"}
                                <Button class="mb-2" onclick={()=>{pauseJob(status["job"]["uuid"])}}><Pause/> Pause Job</Button>
                            {:else if updateList[status["job"]["uuid"]] === "pause_requested"}
                                <Button class="mb-2" disabled><Pause/> Pausing...</Button>
                            {:else}
                                <Button class="mb-2" onclick={()=>{resumeJob(status["job"]["uuid"])}}><Play/> Resume Job</Button>
                            {/if}
                            <br>
                            {#if updateList[status["job"]["uuid"]] === "stop_requested"}
                                <Button variant="destructive" class="mb-2" disabled><Octagon/> Stopping...</Button>
                            {:else}
                                <Button variant="destructive" class="mb-2" onclick={()=>{stopJob(status["job"]["uuid"])}}><Octagon/> Stop Job</Button>
                            {/if}

                        </div>
                    {/if}
                </div>
            </Card.Root>
        {/each}
    </div>
{/if}
