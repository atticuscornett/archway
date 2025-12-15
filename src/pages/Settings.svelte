<script>
    import {Button} from "$lib/components/ui/button/index.js";
    import {Home} from "@lucide/svelte";
    import {onMount} from "svelte";
    import {getVersion} from "@tauri-apps/api/app";
    import {Switch} from "$lib/components/ui/switch/index.js";
    import {Label} from "$lib/components/ui/label/index.js";
    import {Separator} from "$lib/components/ui/separator/index.js";
    import * as Select from "$lib/components/ui/select/index.js";

    let {page = $bindable()} = $props();
    let appVersion = $state("Loading...");
    let appSettings = $state({
        run_on_startup: true,
        log_level: "low"
    });

    let updateSettings = () => {
        console.log("Updating settings:", $state.snapshot(appSettings));
        invoke("set_settings", {
            settings: appSettings
        })
    }

    onMount(async () => {
        appVersion = await getVersion();
        appSettings = await invoke("get_settings");
    })
</script>

<h2 class="mb-5">Settings</h2>
<Button onclick={() => page = "Dashboard"} class="fixed top-10 right-10 z-50"><Home/> Back to Dashboard</Button>
<p class="mb-4">
    App Version: {appVersion} &emsp;
    <a href="#" class="underline" onclick={()=>{open("https://github.com/atticuscornett/archway")}}>GitHub Repo</a> &emsp;
    <a href="#" class="underline" onclick={()=>{open("https://github.com/atticuscornett/archway/issues/new/choose")}}>Report an Issue/Request a Feature</a>
</p>
<Separator class="mb-4"/>

<div>
    <Switch id="run-on-startup" bind:checked={appSettings.run_on_startup} onCheckedChange={updateSettings}/>
    <Label for="run-on-startup">Run Archway in background on startup</Label>
    <br><br>
    <Label for="log-level">
        <h5>Log Level</h5>
        Note: High log level will slow down job processing and increase log file size significantly.<br>
        High log level is only recommended for debugging purposes.
    </Label>
    <Select.Root type="single" bind:value={appSettings.log_level} onValueChange={updateSettings}>
        <Select.Trigger class="w-[180px]">
            {appSettings.log_level.charAt(0).toUpperCase() + appSettings.log_level.slice(1)}
        </Select.Trigger>
        <Select.Content>
            <Select.Item value="low">Low</Select.Item>
            <Select.Item value="medium">Medium</Select.Item>
            <Select.Item value="high">High</Select.Item>
        </Select.Content>
    </Select.Root>
    <br>
    <br>
    <Button onclick={()=>{page = "Restore";}}>Manual Restore</Button>
</div>