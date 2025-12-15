<script lang="ts">
    import {toast} from "svelte-sonner";

    let { page = $bindable(), restoreId = $bindable()} = $props();
    import { onMount } from "svelte";
    import * as Card from "$lib/components/ui/card/index.js";
    import {Button} from "$lib/components/ui/button";
    import {Plus, Home, Pencil, Play, Trash2, Download, ArchiveRestore} from "@lucide/svelte";
    import {save} from "@tauri-apps/plugin-dialog";

    let jobList: Object[] = $state([]);

    let loadJobs = async () => {
        try {
            jobList = JSON.parse(await invoke("get_all_jobs")).slice();
        } catch (error) {
            toast.error("Something went wrong while loading jobs.");
        }
    };

    let deleteJob = async (jobUuid: string) => {
        try {
            let deleteSuccess = await invoke("remove_job_by_uuid", {uuid: jobUuid});
            if (!deleteSuccess) {
                throw new Error("Failed to delete job");
            }
            toast.success("Job deleted successfully.");
            loadJobs();
        } catch (error) {
            toast.error("Failed to delete job. Please try again.");
        }
    };

    let startJob = async (jobUuid: string) => {
        try {
            let startSuccess = await invoke("start_job", {uuid: jobUuid});
            if (!startSuccess) {
                throw new Error("Failed to start job");
            }
            toast.success("Job started successfully.");
            page = "Status"; // Redirect to Status page to view job progress
        } catch (error) {
            toast.error("Job already started or failed to start. Please check the job status.");
        }
    };

    let exportJob = async (jobUuid: string) => {
        let path = await save({
            filters: [
                {
                    name: "Job Files",
                    extensions: ["json"]
                }
            ],
            defaultPath: `job_${jobUuid}.json`
        })

        if (!path) {
            toast.error("Export cancelled.");
            return;
        }

        let result = await invoke("export_job", {uuid: jobUuid, path: path});
        if (result) {
            toast.success("Job exported successfully.");
        } else {
            toast.error("Failed to export job. Please try again.");
        }
    }

    onMount(()=>{
        loadJobs();
    })
</script>

<h2 class="mb-5">Job Manager</h2>
<div class="fixed top-10 right-10 z-50">
    <Button onclick={() => page = "SetUpAutomation"}><Plus/> Import/Create New Job</Button>
    <Button onclick={() => page = "Dashboard"}><Home/> Back to Dashboard</Button>
</div>
{#if jobList.length === 0}
    <p>No jobs found. Create a new job to get started.</p>
{:else}
    <div class="job-list">
        {#each jobList as job}
            <Card.Root class="relative mb-4">
                <Card.Header>
                    <Card.Title>{job["job_name"]}</Card.Title>
                    <Card.Description>Job UUID: {job["uuid"]}</Card.Description>
                </Card.Header>
                <Card.Content>
                    <p>Input Directories: {job["input_dirs"].map((dir: Object)=>dir["path"]).join(",")}</p>
                    <p class="mb-24">Output Directory: {job["output_dir"]}</p>
                </Card.Content>
                <div class="absolute top-4 right-4">
                    <Button class="mb-2" onclick={()=>{startJob(job["uuid"])}}><Play/> Start Job</Button>
                    <br>
                    <Button class="mb-2" onclick={()=>{page="SetUpAutomation:"+job["uuid"]}}><Pencil/> Edit Job</Button>
                    <br>
                    <Button class="mb-2" onclick={()=>{exportJob(job["uuid"])}}><Download/> Export Job</Button>
                    <br>
                    <Button class="mb-2" onclick={()=>{restoreId=job["uuid"];page = "Restore";}}><ArchiveRestore/> Restore</Button>
                    <br>
                    <Button class="mb-2" variant="destructive" onclick={()=>{deleteJob(job["uuid"])}}><Trash2/> Delete Job</Button>
                </div>
            </Card.Root>
        {/each}
    </div>
{/if}
