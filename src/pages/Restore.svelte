<script>
    import { Home } from "@lucide/svelte";
    import {Button} from "$lib/components/ui/button/index.js";
    import {Label} from "$lib/components/ui/label/index.js";
    import {Input} from "$lib/components/ui/input/index.js";
    import * as Select from "$lib/components/ui/select/index.js";
    import {open} from "@tauri-apps/plugin-dialog";
    import {onMount} from "svelte";
    let { restoreId = $bindable() , page = $bindable() } = $props();
    let restoreFile = $state("");
    let behavior = $state("Keep Most Recently Updated Files");
    let stage = $state(0);

    let fileSelect = async (e) => {
        e.preventDefault();

        let selected = await open({
            directory: false,
            multiple: false,
            title: "Select Recovery File Import",
            filters: [
                {
                    name: "Recovery Files",
                    extensions: ["json"]
                }
            ]
        });

        console.log(selected);
        if (selected) {
            restoreFile = selected;
        }
    }

    onMount(async () => {
        await getRestoreFileFromId();
    });

    let getRestoreFileFromId = async () => {
        if (restoreId && restoreId !== "") {
            let job = JSON.parse(await invoke("get_job_by_uuid", {uuid: restoreId}));

            let output_dir = job.output_dir;
            if (output_dir.includes("\\")){
                restoreFile = output_dir + "\\archway-" + restoreId + "\\recovery_paths.json";
            }
            else {
                restoreFile = output_dir + "/archway-" + restoreId + "/recovery_paths.json";
            }

            restoreId = "";
        }
    }
</script>

<h2 class="mb-5">Restore</h2>
{#if stage !== 2}
<div class="fixed top-10 right-10 z-50">
    <Button onclick={() => page = "Dashboard"}><Home/> Back to Dashboard</Button>
</div>
{/if}

<!-- Configure Stage -->
{#if stage === 0}
    <h3>This wizard will guide you through restoring files from a backup.</h3>
    <h3><strong class="text-red-500">This process may replace files on your system, please check the job details carefully before proceeding.</strong></h3>
    <br>
    <Label>Select a restore file (typically named <code>recovery_paths.json</code> in the job folder):</Label>
    <br>
    <Button class="mt-2" onclick={fileSelect}>Select Restore File</Button>
    {#if restoreFile !== ""}
        <p class="mt-2">Selected File: {restoreFile}</p>
    {/if}
    <br> <br>
    <Label>Select a restore behavior:</Label>
    <Select.Root type="single" bind:value={behavior} >
        <Select.Trigger class="w-[500px]">
            {behavior}
        </Select.Trigger>
        <Select.Content>
            <Select.Item value="Keep Most Recently Updated Files">
                Keep Most Recently Updated Files
            </Select.Item>
            <Select.Item value="Overwrite Existing Files">
                Overwrite Existing Files
            </Select.Item>
            <Select.Item value="Skip Existing Files">
                Skip Existing Files
            </Select.Item>
        </Select.Content>
    </Select.Root>

    {#if behavior !== "Skip Existing Files"}
        <h4 class="mt-4 text-red-500">Warning: This option may overwrite existing files on your system.</h4>
    {/if}
    <br>

    {#if restoreFile !== ""}
        <Button class="mt-4" onclick={() => stage = 1}>Continue to Review</Button>
    {/if}
{/if}

<!-- Review Stage -->
{#if stage === 1}
    <h3>Review Restore Details</h3>
    <p><strong>Restore File:</strong> {restoreFile}</p>
    <p><strong>Restore Behavior:</strong> {behavior}</p>
    <h4 class="mt-4 text-red-500">Please review the details above carefully before proceeding. This process may replace files on your system.</h4>
{/if}