<script>
    import {Button} from "$lib/components/ui/button/index.js";
    import {Home} from "@lucide/svelte";
    import {onMount} from "svelte";
    import {getVersion} from "@tauri-apps/api/app";
    import {Switch} from "$lib/components/ui/switch/index.js";
    import {Label} from "$lib/components/ui/label/index.js";
    import {Separator} from "$lib/components/ui/separator/index.js";

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

<div class="flex items-center space-x-2">
    <Switch id="run-on-startup" bind:checked={appSettings.run_on_startup} onCheckedChange={updateSettings}/>
    <Label for="run-on-startup">Run Archway in background on startup</Label>
</div>