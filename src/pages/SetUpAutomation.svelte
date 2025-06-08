<script>
    import JobBehavior from "../components/JobSetup/JobBehavior.svelte";
    import {Button} from "$lib/components/ui/button/index.js";
    import { ChevronLeft, ChevronRight, Home } from '@lucide/svelte';
    import OutputSelect from "../components/JobSetup/OutputSelect.svelte";
    import InputSelect from "../components/JobSetup/InputSelect.svelte";
    import FileFilters from "../components/JobSetup/FileFilters.svelte";
    import JobTriggers from "../components/JobSetup/JobTriggers.svelte";
    import {toast} from "svelte-sonner";
    import {onMount} from "svelte";

    let { page = $bindable(), pageData=$bindable() } = $props();

    let step = $state(0);
    let canContinue = $state(true);
    let job = $state({
        "job_name": "New Job",
        "uuid": crypto.randomUUID(),
        "file_behavior": "copy",
        "input_dirs": [],
        "output_dir": "",
        "copies": 1,
        "portable": false,
        "file_filters": [],
        "triggers": [],
        "new_folder": false,
        "output_device": "special:thisdrive",
        "version": 1
    });

    let nextStep = async () => {
        step += 1;

        if (step === 5) {
            let result = await invoke("setup_job", {jobInfo: JSON.stringify(job)});
            page = "Dashboard";

            console.log(JSON.stringify(job));

            if (result) {
                toast.success("Job created successfully!");
            }
            else {
                toast.error("Failed to create job. Please try again.");
            }
        }
    }

    let prevStep = () => {
        step -= 1;
    }

    onMount(async ()=> {
        if (pageData !== ""){
            job = JSON.parse(await invoke("get_job_by_uuid", {uuid: pageData}));
            pageData = "";
        }
    });
</script>

{#if step === 0}
    <JobBehavior bind:job bind:canContinue/>
{/if}
{#if step === 1}
    <OutputSelect bind:job bind:canContinue/>
{/if}
{#if step === 2}
    <InputSelect bind:job bind:canContinue/>
{/if}
{#if step === 3}
    <FileFilters bind:job bind:canContinue/>
{/if}
{#if step === 4}
    <JobTriggers bind:job bind:canContinue/>
{/if}

{#if step > 0}
    <Button variant="secondary" class="fixed bottom-4 left-4" onclick={prevStep} disabled={step === 5}><ChevronLeft/> Back</Button>
{:else}
    <Button variant="secondary" class="fixed bottom-4 left-4" onclick={()=>{page="Dashboard";}}><Home/> Go to Dashboard</Button>
{/if}
<Button class="fixed bottom-4 right-4" onclick={nextStep} disabled={(!canContinue || step === 5)}>
    {#if step <= 3}
        Next
    {:else if step === 4}
        Finish
    {:else}
        Creating Job...
    {/if}
    <ChevronRight/>
</Button>