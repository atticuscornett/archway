<script lang="ts">
    import {Button} from "$lib/components/ui/button/index.js";
    import {Folder} from "@lucide/svelte";
    import {onMount} from "svelte";
    import {invoke} from "@tauri-apps/api/core";
    import {Label} from "$lib/components/ui/label/index.js";
    import * as RadioGroup from "$lib/components/ui/radio-group/index.js";
    import { open } from '@tauri-apps/plugin-dialog';
    import {Checkbox} from "$lib/components/ui/checkbox/index.js";

    let {job = $bindable(), canContinue = $bindable()} = $props();

    onMount(async () => {
        canContinue = job["output_dir"] !== "";
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
            job["output_dir"] = selected;
            canContinue = true;
        }
    }

    let hasOutputDevice = $state(false);
    let updateOutputDevice = () => {
        if (hasOutputDevice) {
            job["output_device"] = "special:any";
        } else {
            job["output_device"] = "special:thisdrive";
        }
    }
</script>

<h2>Output Folder</h2>
<br>
<Button onclick={selectFolder}><Folder/> Select Output Folder</Button>
{#if job["output_dir"]}
    <h5>Output Folder: {job["output_dir"]}</h5>

    {#if job["output_dir"].startsWith("C:")}
            <h4 class="text-red-500">Are you sure you want to save to your main drive? This may cause unexpected behavior.</h4>
    {/if}
{/if}
<br>
<Checkbox class="my-4" id="device-specific" bind:checked={hasOutputDevice} onCheckedChange={updateOutputDevice}></Checkbox>
<Label for="device-specific">Use any device with this drive letter</Label>
<br>
<Checkbox id="new-folder" bind:checked={job["new_folder"]}></Checkbox>
<Label for="new-folder">Create folder if it does not exist</Label>
