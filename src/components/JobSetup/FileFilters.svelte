<script lang="ts">
    import {Switch} from "$lib/components/ui/switch/index.js";
    import {Label} from "$lib/components/ui/label/index.js";
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import {Button} from "$lib/components/ui/button";
    import {Plus, Minus} from "@lucide/svelte";
    import {Input} from "$lib/components/ui/input";
    import {onMount} from "svelte";

    let { job = $bindable(), canContinue = $bindable() } = $props();

    let typeFilter = $state(false);
    let lastUsedFilter = $state(false);
    let sizeFilter = $state(false);

    let typeFilterList: string[] = $state([]);
    let typeFilterOptions = $state({
        "documents": false,
        "videos": false,
        "pictures": false,
        "music": false,
        "archives": false
    });

    let extensionInput = $state("");

    let loadStateFromJob = () => {
        for (let filter of job["file-filters"]) {
            if (filter["type"] === "extension") {
                typeFilter = true;
                typeFilterList = filter["traits"]["extensions"];

                for (let i = 0; i < typeFilterList.length; i++) {
                    if (typeFilterList[i].endsWith(":special")) {
                        typeFilterOptions[typeFilterList[i].replace(":special", "")] = true;
                        typeFilterList.splice(Number(i), 1);
                        i--;
                    }
                }

            }
        }
    }

    let updateJob = () => {
        job["file-filters"] = [];

        if (typeFilter) {
            let fullFilterList = [];
            if (typeFilterOptions.documents) {
                fullFilterList.push("documents:special");
            }
            if (typeFilterOptions.videos) {
                fullFilterList.push("videos:special");
            }
            if (typeFilterOptions.pictures) {
                fullFilterList.push("pictures:special");
            }
            if (typeFilterOptions.music) {
                fullFilterList.push("music:special");
            }
            if (typeFilterOptions.archives) {
                fullFilterList.push("archives:special");
            }

            fullFilterList.concat(typeFilterList);

            job["file-filters"].push({
                "type": "extension",
                "traits": {
                    "extensions": fullFilterList
                }
            });
        }



        console.log(job["file-filters"]);
    }


    let addExtensions = () => {
        if (extensionInput === "") {
            return;
        }
        let exts = extensionInput.split(",").map(ext => ext.trim()).map(ext => ext.replace(/^\./, ""));
        typeFilterList = [...typeFilterList, ...exts];
        extensionInput = "";

        updateJob();
    }


    let removeExtension = (index: number) => {
        typeFilterList.splice(index, 1);
    }

    onMount(() => {
        loadStateFromJob();
        canContinue = true;
    });
</script>

<h2>File Filters</h2>

<Switch id="typeFilter" bind:checked={typeFilter}></Switch>
<Label for="typeFilter" class="align-text-bottom mt-4 text-lg">Filter by File Type</Label>
<br>

{#if typeFilter}
    <h5>Only backup/archive files of these types:</h5>

    <div class="flex mb-5">
        <div>
            <h3>Categories</h3>

            <Switch id="documents" onCheckedChange={updateJob} bind:checked={typeFilterOptions.documents}></Switch>
            <Label for="documents" class="align-text-bottom text-lg">Documents (.docx, .pptx, .pdf, etc.)</Label>
            <br>
            <Switch id="videos" onCheckedChange={updateJob} bind:checked={typeFilterOptions.videos}></Switch>
            <Label for="videos" class="align-text-bottom text-lg">Videos (.mp4, .mov, etc.)</Label>
            <br>
            <Switch id="pictures" onCheckedChange={updateJob} bind:checked={typeFilterOptions.pictures}></Switch>
            <Label for="pictures" class="align-text-bottom text-lg">Pictures (.jpg, .png, etc.)</Label>
            <br>
            <Switch id="music" onCheckedChange={updateJob} bind:checked={typeFilterOptions.music}></Switch>
            <Label for="music" class="align-text-bottom text-lg">Music (.mp3, .wav, etc.)</Label>
            <br>
            <Switch id="archives" onCheckedChange={updateJob} bind:checked={typeFilterOptions.archives}></Switch>
            <Label for="archives" class="align-text-bottom text-lg">Archives (.zip, .tar, etc.)</Label>
        </div>
        <div class="ml-5">
            <h3>Extensions</h3>
            {#each typeFilterList as ext, index}
                <Button variant="destructive" class="mb-2" onclick={()=>{removeExtension(index)}}><Minus/> .{ext}</Button>
                <br>
            {/each}

            <Dialog.Root onOpenChange={addExtensions}>
                <Dialog.Trigger><Button><Plus/> Add</Button></Dialog.Trigger>
                <Dialog.Content>
                    <Dialog.Header>
                        <Dialog.Title>Extension</Dialog.Title>
                    </Dialog.Header>
                    <Input bind:value={extensionInput} placeholder="Extension (.docx, .webm, etc.) or extensions seperated by commas"/>
                    <Dialog.Close type="submit"><Button><Plus/> Add Extension</Button></Dialog.Close>
                </Dialog.Content>
            </Dialog.Root>
        </div>
    </div>
{/if}

<Switch id="lastUsedFilter" bind:checked={lastUsedFilter}></Switch>
<Label for="lastUsedFilter" class="align-text-bottom mt-4 text-lg">Filter by Last Use</Label>
<br>

{#if lastUsedFilter}
    <h5>Only backup/archive files that have not been used in the last:</h5>
{/if}

<Switch id="sizeFilter" bind:checked={sizeFilter}></Switch>
<Label for="sizeFilter" class="align-text-bottom mt-4 text-lg">Filter by File Size</Label>
<br>

{#if sizeFilter}
    <h5>Only backup/archive files that are larger than:</h5>
{/if}

<br class="mb-10">