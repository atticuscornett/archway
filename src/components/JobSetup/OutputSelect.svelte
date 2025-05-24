<script>
    import {Button} from "$lib/components/ui/button/index.js";
    import {Folder} from "@lucide/svelte";
    import {onMount} from "svelte";
    import {invoke} from "@tauri-apps/api/core";
    import {Label} from "$lib/components/ui/label/index.js";
    import * as RadioGroup from "$lib/components/ui/radio-group/index.js";
    import { open } from '@tauri-apps/plugin-dialog';
    import {Checkbox} from "$lib/components/ui/checkbox/index.js";

    let {job = $bindable(), canContinue = $bindable()} = $props();

    let drives = $state([]);

    onMount(async () => {
        canContinue = job["output-dir"] !== "";
    })

    let selectFolder = async () => {
        let selected = await open({
            directory: true,
            multiple: false,
            title: "Select Output Folder",
            filters: [
                {
                    name: "Folders",
                    extensions: ["*"]
                }
            ]
        });

        console.log(selected);

        if (selected) {
            job["output-dir"] = selected;
            canContinue = true;
        }
    }
</script>

<h2>Output Folder</h2>
<br>
<Button onclick={selectFolder}><Folder/> Select Output Folder</Button>
{#if job["output-dir"]}
    <h5>Output Folder: {job["output-dir"]}</h5>

    {#if job["output-dir"].startsWith("C:")}
            <h4 class="text-red-500">Are you sure you want to save to your main drive? This may cause unexpected behavior.</h4>
    {/if}
{/if}
<Checkbox class="my-4" id="device-specific" bind:checked={job["output-device"]}></Checkbox>
<Label for="device-specific">Use any device with this drive letter</Label>
<br>
<Checkbox id="new-folder" bind:checked={job["new-folder"]}></Checkbox>
<Label for="new-folder">Create folder if it does not exist</Label>
