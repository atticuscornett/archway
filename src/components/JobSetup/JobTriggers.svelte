<script>
    import {Switch} from "$lib/components/ui/switch/index.js";
    import {Label} from "$lib/components/ui/label/index.js";
    import * as Select from "$lib/components/ui/select/index.js";

    let { job = $bindable(), canContinue = $bindable() } = $props();

    let times = ["12 AM", "1 AM", "2 AM", "3 AM", "4 AM", "5 AM", "6 AM", "7 AM", "8 AM", "9 AM", "10 AM", "11 AM",
        "12 PM", "1 PM", "2 PM", "3 PM", "4 PM", "5 PM", "6 PM", "7 PM", "8 PM", "9 PM", "10 PM",
        "11 PM"];

    let onSchedule = $state(false);
    let scheduleEnabled = $state({
        "hourly": false,
        "daily": false,
        "weekly": false,
        "monthly": false,
    });
    let scheduleTiming = $state({
        "daily": "9 AM",
        "weekly": ["Monday", "9 AM"],
        "monthly": ["1", "9 AM"]
    });

</script>

<h2>Job Triggers</h2>
<h4>The backup/archive will run whenever any of these occur:</h4>
<h5 class="mb-4">Job triggers may run any time within a minute of the event.</h5>

<Switch id="devicePlugIn"></Switch>
<Label for="devicePlugIn" class="align-text-bottom text-lg">Output Device Connected</Label>
<br>
<Switch bind:checked={onSchedule} id="schedule"></Switch>
<Label for="schedule" class="align-text-bottom text-lg">On A Schedule</Label>
<br>

{#if onSchedule}
    <div class="ml-4">
        <Switch bind:checked={scheduleEnabled.hourly} id="hourly"></Switch>
        <Label for="hourly" class="align-text-bottom text-lg">Hourly</Label>
        <br>
        <Switch bind:checked={scheduleEnabled.daily} id="daily"></Switch>
        <Label for="daily" class="align-text-bottom text-lg">Daily</Label>
        {#if scheduleEnabled.daily}
            <br>
            <Select.Root type="single" bind:value={scheduleTiming.daily}>
                <Select.Trigger class="w-[180px]">
                    at {scheduleTiming.daily}
                </Select.Trigger>
                <Select.Content>
                    {#each times as time}
                        <Select.Item value={time} label={time}>
                            {time}
                        </Select.Item>
                    {/each}
                </Select.Content>
            </Select.Root>
        {/if}
    </div>
{/if}