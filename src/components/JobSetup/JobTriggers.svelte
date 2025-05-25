<script lang="ts">
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

    let addEnding = (day: number) => {
        if (day === 1 || day === 21 || day === 31) {
            return "st";
        } else if (day === 2 || day === 22) {
            return "nd";
        } else if (day === 3 || day === 23) {
            return "rd";
        } else {
            return "th";
        }
    }

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
    <h5>If Archway is not running when job is scheduled, it will not run.</h5>
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
        <br>
        <Switch bind:checked={scheduleEnabled.weekly} id="weekly"></Switch>
        <Label for="weekly" class="align-text-bottom text-lg">Weekly</Label>
        {#if scheduleEnabled.weekly}
            <br>
            <Select.Root type="single" bind:value={scheduleTiming.weekly[0]}>
                <Select.Trigger class="w-[180px]">
                    on {scheduleTiming.weekly[0]}
                </Select.Trigger>
                <Select.Content>
                    {#each ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"] as day}
                        <Select.Item value={day} label={day}>
                            {day}
                        </Select.Item>
                    {/each}
                </Select.Content>
            </Select.Root>
            <Select.Root type="single" bind:value={scheduleTiming.weekly[1]}>
                <Select.Trigger class="w-[180px]">
                    at {scheduleTiming.weekly[1]}
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
        <br>
        <Switch bind:checked={scheduleEnabled.monthly} id="monthly"></Switch>
        <Label for="monthly" class="align-text-bottom text-lg">Monthly</Label>
        {#if scheduleEnabled.monthly}
            <br>
            <Select.Root type="single" bind:value={scheduleTiming.monthly[0]}>
                <Select.Trigger class="w-[180px]">
                    on the {scheduleTiming.monthly[0]}{addEnding(Number(scheduleTiming.monthly[0]))}
                </Select.Trigger>
                <Select.Content>
                    {#each Array.from({length: 28}, (_, i) => i + 1) as day}
                        <Select.Item value={String(day)} label={String(day)}>
                            {day}{addEnding(day)}
                        </Select.Item>
                    {/each}
                </Select.Content>
            </Select.Root>
            <Select.Root type="single" bind:value={scheduleTiming.monthly[1]}>
                <Select.Trigger class="w-[180px]">
                    at {scheduleTiming.monthly[1]}
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

<br class="mb-10">