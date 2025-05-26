<script>
    import JobBehavior from "../components/JobSetup/JobBehavior.svelte";
    import {Button} from "$lib/components/ui/button/index.js";
    import { ChevronLeft, ChevronRight } from '@lucide/svelte';
    import OutputSelect from "../components/JobSetup/OutputSelect.svelte";
    import InputSelect from "../components/JobSetup/InputSelect.svelte";
    import FileFilters from "../components/JobSetup/FileFilters.svelte";
    import JobTriggers from "../components/JobSetup/JobTriggers.svelte";

    let { page = $bindable() } = $props();

    let step = $state(0);
    let canContinue = $state(true);
    let job = $state({
        "job-name": "New Job",
        "uuid": crypto.randomUUID(),
        "file-behavior": "copy",
        "input-dirs": [],
        "output-dir": "",
        "copies": 1,
        "portable": false,
        "file-filters": [],
        "triggers": [],
        "new-folder": false,
        "output-device": false
    });

    let nextStep = () => {
        step += 1;

        if (step === 5) {
            invoke("setup_job", {jobInfo: JSON.stringify(job)});
            page = "Dashboard";
        }
    }

    let prevStep = () => {
        step -= 1;
    }
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
    <Button variant="secondary" class="fixed bottom-4 left-4" onclick={prevStep}><ChevronLeft/> Back</Button>
{/if}
<Button class="fixed bottom-4 right-4" onclick={nextStep} disabled={!canContinue}>
    {(step === 4) ? "Finish" : "Next" } <ChevronRight/>
</Button>