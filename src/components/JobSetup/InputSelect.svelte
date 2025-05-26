<script lang="ts">
    import {Switch} from "$lib/components/ui/switch/index.js";
    import {Label} from "$lib/components/ui/label/index.js";
    import {onMount} from "svelte";
    import {Plus, Minus} from "@lucide/svelte";
    import {Button} from "$lib/components/ui/button/index.js";
    import { open } from '@tauri-apps/plugin-dialog';

    let { job = $bindable(), canContinue = $bindable() } = $props();

    let libraries = $state({
        "documents": false,
        "music": false,
        "desktop": false,
        "downloads": false,
        "pictures": false,
        "videos": false
    });

    let refreshInputDirs = () => {
        job["input_dirs"] = [];
        if (libraries.documents) {
            job["input_dirs"].push({"path_type": "library", "path": "documents"});
        }
        if (libraries.music) {
            job["input_dirs"].push({"path_type": "library", "path": "music"});
        }
        if (libraries.desktop) {
            job["input_dirs"].push({"path_type": "library", "path": "desktop"});
        }
        if (libraries.downloads) {
            job["input_dirs"].push({"path_type": "library", "path": "downloads"});
        }
        if (libraries.pictures) {
            job["input_dirs"].push({"path_type": "library", "path": "pictures"});
        }
        if (libraries.videos) {
            job["input_dirs"].push({"path_type": "library", "path": "videos"});
        }
        if (customFolders.length > 0) {
            customFolders.forEach(folder => {
                job["input_dirs"].push({"path_type": "custom", "path": folder});
            });
        }

        console.log(job["input_dirs"]);
    }

    let customFolders: string[] = $state([]);

    let addFolder = async () => {
        let selected = await open({
            directory: true,
            multiple: false,
            title: "Select Input Folder",
            filters: [
                {
                    name: "Folders",
                    extensions: ["*"]
                }
            ]
        });

        if (selected) {
            customFolders.push(selected);
        }

        refreshInputDirs();
    }

    let removeFolder = (index: number) => {
        customFolders.splice(index, 1);
    }

    onMount(()=> {
        canContinue = true;

        for (let i = 0; i < job["input_dirs"].length; i++) {
            if (job["input_dirs"][i]["path_type"] === "custom") {
                customFolders.push(job["input_dirs"][i]["path"]);
            } else {
                libraries[job["input_dirs"][i]["path"]] = true;
            }
        }
    })
</script>

<h2>Input Folders</h2>
<br>
<div class="flex">
    <div>
        <h3>Libraries</h3>
        <Switch id="docs" bind:checked={libraries.documents} onCheckedChange={refreshInputDirs}></Switch>
        <Label for="docs" class="align-text-bottom text-lg">Documents</Label>
        <br>
        <Switch id="music" bind:checked={libraries.music} onCheckedChange={refreshInputDirs}></Switch>
        <Label for="music" class="align-text-bottom text-lg">Music</Label>
        <br>
        <Switch id="desktop" bind:checked={libraries.desktop} onCheckedChange={refreshInputDirs}></Switch>
        <Label for="desktop" class="align-text-bottom text-lg">Desktop</Label>
        <br>
        <Switch id="downloads" bind:checked={libraries.downloads} onCheckedChange={refreshInputDirs}></Switch>
        <Label for="downloads" class="align-text-bottom text-lg">Downloads</Label>
        <br>
        <Switch id="pictures" bind:checked={libraries.pictures} onCheckedChange={refreshInputDirs}></Switch>
        <Label for="pictures" class="align-text-bottom text-lg">Pictures</Label>
        <br>
        <Switch id="videos" bind:checked={libraries.videos} onCheckedChange={refreshInputDirs}></Switch>
        <Label for="videos" class="align-text-bottom text-lg">Videos</Label>
    </div>
    <div class="mx-10">
        <h3>Custom Folders</h3>
        {#each customFolders as folder, index}
            <div class="flex">
                <Button variant="destructive" class="mb-2" onclick={() => removeFolder(index)}><Minus/>{folder}</Button>
            </div>
        {/each}

        <Button onclick={addFolder}><Plus/> Add Folder</Button>
    </div>
</div>