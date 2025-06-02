<script lang="ts">
    import {Switch} from "$lib/components/ui/switch/index.js";
    import {Label} from "$lib/components/ui/label/index.js";
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import * as Select from "$lib/components/ui/select/index.js";
    import {Button} from "$lib/components/ui/button";
    import {Plus, Minus} from "@lucide/svelte";
    import {Input} from "$lib/components/ui/input";
    import {onMount} from "svelte";

    let { job = $bindable(), canContinue = $bindable() } = $props();

    let typeFilter = $state(false);
    let lastUsedFilter = $state(false);
    let sizeFilter = $state(false);

    let lastUsedFilterValue = $state("Month");
    let fileSizeFilterValue = $state(1000);

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
        for (let filter of job["file_filters"]) {
            if (filter["filter_type"] === "extension") {
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

            if (filter["filter_type"] === "last-used") {
                lastUsedFilter = true;
                lastUsedFilterValue = filter["traits"]["lastused"];
                // Restore to human-readable format
                if (/^[0-9]$/.test(lastUsedFilterValue[0])){
                    let fixedValue = lastUsedFilterValue[0];
                    fixedValue += " ";
                    fixedValue += lastUsedFilterValue[1].toUpperCase();
                    fixedValue += lastUsedFilterValue.slice(2);
                    lastUsedFilterValue = fixedValue;
                }
                else {
                    lastUsedFilterValue = lastUsedFilterValue[0].toUpperCase() + lastUsedFilterValue.slice(1);
                }
            }

            if (filter["filter_type"] === "size") {
                sizeFilter = true;
                fileSizeFilterValue = filter["traits"]["size"];
            }
        }
    }

    let updateJob = () => {
        job["file_filters"] = [];

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

            fullFilterList = fullFilterList.concat(typeFilterList);

            job["file_filters"].push({
                "filter_type": "extension",
                "traits": {
                    "extensions": fullFilterList
                }
            });
        }

        if (lastUsedFilter) {
            job["file_filters"].push({
                "filter_type": "last-used",
                "traits": {
                    "lastused": lastUsedFilterValue.replace(" ", "").toLowerCase()
                }
            });
        }

        if (sizeFilter) {
            if (fileSizeFilterValue < 1) {
                fileSizeFilterValue = 1;
            }

            job["file_filters"].push({
                "filter_type": "size",
                "traits": {
                    "size": fileSizeFilterValue
                }
            });
        }


        console.log(job["file_filters"]);
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

<Switch id="typeFilter" bind:checked={typeFilter} onCheckedChange={updateJob}></Switch>
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

<Switch id="lastUsedFilter" bind:checked={lastUsedFilter} onCheckedChange={updateJob}></Switch>
<Label for="lastUsedFilter" class="align-text-bottom mt-4 text-lg">Filter by Last Use</Label>
<br>

{#if lastUsedFilter}
    <h5>Only backup/archive files that have not been used in the last:</h5>
    <Select.Root type="single" bind:value={lastUsedFilterValue} onValueChange={updateJob}>
        <Select.Trigger class="w-[180px] mb-5">{lastUsedFilterValue}</Select.Trigger>
        <Select.Content>
            <Select.Item value="Week">Week</Select.Item>
            <Select.Item value="2 Weeks">2 Weeks</Select.Item>
            <Select.Item value="Month">Month</Select.Item>
            <Select.Item value="2 Months">2 Months</Select.Item>
            <Select.Item value="3 Months">3 Months</Select.Item>
            <Select.Item value="6 Months">6 Months</Select.Item>
            <Select.Item value="Year">Year</Select.Item>
        </Select.Content>
    </Select.Root>
{/if}

<Switch id="sizeFilter" bind:checked={sizeFilter} onCheckedChange={updateJob}></Switch>
<Label for="sizeFilter" class="align-text-bottom mt-4 text-lg">Filter by File Size</Label>
<br>

{#if sizeFilter}
    <h5>Only backup/archive files that are larger than:</h5>
    <Input onchange={updateJob} bind:value={fileSizeFilterValue} min="1" type="number" placeholder="Size in MB" class="w-[180px] inline"/>
    <Label for="sizeFilter" class="align-baseline text-lg inline">MB</Label>
    <br>
{/if}

<br class="mb-10">