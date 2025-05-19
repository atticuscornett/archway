<script>
    import JobBehavior from "../components/JobBehavior.svelte";
    import {Button} from "$lib/components/ui/button/index.js";
    import { ChevronLeft, ChevronRight } from '@lucide/svelte';

    let step = $state(0);
    let job = $state({
        "job-name": "New Job",
        "uuid": crypto.randomUUID(),
        "file-behavior": "copy",
        "input-dirs": [],
        "output-dir": "",
        "copies": 1,
        "file-filters": [],
        "triggers": []
    });
</script>

{#if step === 0}
    <JobBehavior bind:job={job}/>
{/if}

{#if step > 1}
    <Button variant="secondary" class="fixed bottom-4 left-4"><ChevronLeft/> Back</Button>
{/if}
<Button class="fixed bottom-4 right-4">Next <ChevronRight/></Button>