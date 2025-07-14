<script>
    import {onMount} from "svelte";
    import JobListTile from "../components/Dashboard/JobListTile.svelte";
    import StatusTile from "../components/Dashboard/StatusTile.svelte";
    import HealthTile from "../components/Dashboard/HealthTile.svelte";
    import SettingsTile from "../components/Dashboard/SettingsTile.svelte";
    import * as AlertDialog from "$lib/components/ui/alert-dialog/index.js";
    import {getVersion} from "@tauri-apps/api/app";

    let {page = $bindable()} = $props();

    let updateNoticeOpen = $state(false);

    onMount(()=>{
        updateNoticeOpen = false;
        // Check for updates
        fetch("https://api.github.com/repos/atticuscornett/archway/releases/latest")
            .then(response => {
                if (response.status === 200) {
                    response.json()
                        .then(async (data) => {
                            let currentVersion = await getVersion();
                            let latestVersion = data.tag_name;
                            if (latestVersion.startsWith("v")) {
                                latestVersion = latestVersion.slice(1); // Remove 'v' prefix if present
                            }

                            if (currentVersion === latestVersion) {
                                console.log("No updates found. (Current version is up to date)");
                                return;
                            }

                            updateNoticeOpen = true;
                        })
                }
                if (response.status === 404) {
                    console.log("No updates found. (No releases on GitHub)");
                }
            })
            .catch(error => {
                console.error("Error checking for updates:", error);
            });
    })

</script>

<img src="./src/img/ArchwayFull.svg" class="w-52 mb-4" alt="Archway Logo" />
<HealthTile bind:page/>
<JobListTile bind:page/>
<StatusTile bind:page/>
<SettingsTile bind:page/>

<AlertDialog.Root bind:open={updateNoticeOpen}>
    <AlertDialog.Content>
        <AlertDialog.Header>
            <AlertDialog.Title>Update is available.</AlertDialog.Title>
            <AlertDialog.Description>
                An update is available for Archway. Please download the latest version from our website to ensure you have the best experience.
            </AlertDialog.Description>
        </AlertDialog.Header>
        <AlertDialog.Footer>
            <AlertDialog.Cancel>Not Now</AlertDialog.Cancel>
            <AlertDialog.Action onclick={()=>{open("https://github.com/atticuscornett/archway/releases/latest")}}>Go to Releases</AlertDialog.Action>
        </AlertDialog.Footer>
    </AlertDialog.Content>
</AlertDialog.Root>