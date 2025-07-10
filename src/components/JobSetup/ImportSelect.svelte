<script>
    import {onMount} from "svelte";
    import {Button} from "$lib/components/ui/button/index.js";
    import {Pencil, Play, Plus, Trash2} from "@lucide/svelte";
    import * as Card from "$lib/components/ui/card/index.js";

    let {job = $bindable(), canContinue = $bindable(), step = $bindable()} = $props();

    let driveInfoList = $state([]);
    let totalJobsOnDrives = $state(0);


    onMount(async () => {
        canContinue = false;
        driveInfoList = JSON.parse(await invoke("get_all_drive_info"));

        totalJobsOnDrives = 0;
        for (let drive of driveInfoList) {
            totalJobsOnDrives += drive["jobs"].length;
        }
    });
</script>

<h2>Import</h2>

{#if totalJobsOnDrives === 0}
    <h3>No jobs found automatically on drives.</h3>
{:else}
    <h4>Automatically found on drives:</h4>
    {#each driveInfoList as drive}
        {#if drive.jobs.length > 0}
            <h5>Drive: {drive.uuid}</h5>
            <ul>
                {#each drive.jobs as jobItem}
                    <li>
                        <Card.Root class="relative mb-4">
                            <Card.Header>
                                <Card.Title>{jobItem["job_name"]}</Card.Title>
                                <Card.Description><span class="mb-5">Job UUID: {jobItem["uuid"]}</span></Card.Description>
                            </Card.Header>
                            <Card.Content>
                                <Button>Import</Button>
                            </Card.Content>
                        </Card.Root>
                    </li>
                {/each}
            </ul>
        {/if}
    {/each}
{/if}

<br>
<Button>Import File Manually <Plus/></Button>