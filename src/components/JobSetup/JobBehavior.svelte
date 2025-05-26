<script>
    import {Files, FileSliders} from "@lucide/svelte";
    import * as Select from "$lib/components/ui/select/index.js";
    import {Input} from "$lib/components/ui/input/index.js";
    import {Checkbox} from "$lib/components/ui/checkbox/index.js";
    import {Label} from "$lib/components/ui/label/index.js";
    import {onMount} from "svelte";


    let { job = $bindable(), canContinue = $bindable() } = $props();

    $effect(() => {
        if (job["copies"] < 1) {
            job["copies"] = 1;
        }
    })

    onMount(() => {
        canContinue = true;
    });
</script>

<h2>Job Setup</h2>


<h5>Job Name</h5>
<Input bind:value={job["job_name"]} placeholder="Job Name" />
<h6>UUID: {job.uuid}</h6>
<br>
<h5>File Behavior</h5>
<Select.Root type="single" bind:value={job["file_behavior"]} >
    <Select.Trigger class="w-[180px]">
        {#if job["file_behavior"] === "copy"}
            <Files></Files>
        {:else}
            <FileSliders></FileSliders>
        {/if}
        &emsp;
        {job["file_behavior"].replace("copy", "Copy Files").replace("move", "Move Files")}
    </Select.Trigger>
    <Select.Content>
        <Select.Item value="copy" label="Copy">
            <Files></Files>
            &emsp;Copy Files
        </Select.Item>
        <Select.Item value="move" label="Move">
            <FileSliders></FileSliders>
            &emsp;Move Files
        </Select.Item>
    </Select.Content>
</Select.Root>

{#if job["file_behavior"] === "copy"}
    <br>
    <h5>Copies (Keep Last {job["copies"]} Backup{job["copies"] > 1 ? "s":""})</h5>
    <Input bind:value={job["copies"]} type="number" placeholder="Number of Copies" />
{/if}
<br>

<Checkbox id="portable" bind:checked={job["portable"]}></Checkbox>
<Label for="portable">Copy this job to drive for multi-machine use</Label>

<style>
    h2 {
        margin-top: 0;
    }
</style>