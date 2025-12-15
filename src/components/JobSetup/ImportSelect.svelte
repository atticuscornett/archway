<script>
    import {onMount} from "svelte";
    import {Button} from "$lib/components/ui/button/index.js";
    import {Pencil, Play, Plus, Trash2} from "@lucide/svelte";
    import * as Card from "$lib/components/ui/card/index.js";
    import {open} from "@tauri-apps/plugin-dialog";
    import {toast} from "svelte-sonner";

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

    let selectDetectedJob = (driveIndex, jobIndex) => {
        job = driveInfoList[driveIndex]["jobs"][jobIndex];
        step = 0;
    }

    let importManually = async () => {
        let selected = await open({
            directory: false,
            multiple: false,
            title: "Select Job Import",
            filters: [
                {
                    name: "Job Files",
                    extensions: ["json"]
                }
            ]
        });

        if (!selected) {
            return; // User canceled the dialog
        }

        let file_type = await invoke("get_job_file_type", {file: selected});

        if (file_type === "unknown") {
            toast.error("The selected file is not a valid job file. Please select a drive info file or a job file.");
        }

        if (file_type === "drive_file") {
            let jobs = JSON.parse(await invoke("get_job_list_from_drive_info_file", {file: selected}));
            if (jobs.length === 0) {
                toast.error("No jobs found in the selected drive info file.");
            }
            else if (jobs.length === 1) {
                job = jobs[0];
                step = 0;
                toast.success("Job imported successfully from drive info file.");
            }
            else {
                toast.info("Multiple jobs found in drive info file. Please select a job to import.");
                let importedOptions = {
                    uuid: "Import Options",
                    jobs: jobs
                }
                driveInfoList.push(importedOptions);
            }
        }

        if (file_type === "single_job") {
            job = JSON.parse(await invoke("get_individual_job_file", {file: selected}));
            step = 0;
            toast.success("Job imported successfully from file.");
        }
    }
</script>

<h2>Import</h2>

{#if totalJobsOnDrives === 0}
    <h3>No jobs found automatically on drives.</h3>
{:else}
    <h4>Automatically found on drives:</h4>
    {#each driveInfoList as drive, driveIndex}
        {#if drive.jobs.length > 0}
            <h5>Drive: {drive.uuid}</h5>
            <ul>
                {#each drive.jobs as jobItem, jobIndex}
                    <li>
                        <Card.Root class="relative mb-4">
                            <Card.Header>
                                <Card.Title>{jobItem["job_name"]}</Card.Title>
                                <Card.Description><span class="mb-5">Job UUID: {jobItem["uuid"]}</span></Card.Description>
                            </Card.Header>
                            <Card.Content>
                                <Button onclick={()=>{selectDetectedJob(driveIndex, jobIndex)}}>Import</Button>
                            </Card.Content>
                        </Card.Root>
                    </li>
                {/each}
            </ul>
        {/if}
    {/each}
{/if}

<br>
<Button class="mb-6" onclick={importManually}>Import File Manually <Plus/></Button>