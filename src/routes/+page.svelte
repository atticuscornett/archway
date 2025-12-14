<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import {Calendar} from "$lib/components/ui/calendar";
    import { toast } from "svelte-sonner";
    import WelcomePage from "../pages/WelcomePage.svelte";
    import SetUpAutomation from "../pages/SetUpAutomation.svelte";
    import Dashboard from "../pages/Dashboard.svelte";
    import JobManager from "../pages/JobManager.svelte";
    import Status from "../pages/Status.svelte";
    import Health from "../pages/Health.svelte";
    import Settings from "../pages/Settings.svelte";
    import Restore from "../pages/Restore.svelte";

    let name = $state("");
    let greetMsg = $state("");
    window.invoke = invoke;
    let page = $state("WelcomePage");
    let pageData = $state("");
    let restoreId = $state("");

    $effect(()=>{
        if (page.startsWith("SetUpAutomation:")) {
            pageData = page.replace("SetUpAutomation:", "");
            page = "SetUpAutomation";
        }
    })
</script>

<main >
    {#if page === "WelcomePage"}
        <WelcomePage bind:page></WelcomePage>
    {/if}
    {#if page === "SetUpAutomation"}
        <SetUpAutomation bind:page bind:pageData></SetUpAutomation>
    {/if}
    {#if page === "Dashboard"}
        <Dashboard bind:page></Dashboard>
    {/if}
    {#if page === "JobManager"}
        <JobManager bind:page bind:restoreId></JobManager>
    {/if}
    {#if page === "Status"}
        <Status bind:page></Status>
    {/if}
    {#if page === "Health"}
        <Health bind:page></Health>
    {/if}
    {#if page === "Settings"}
        <Settings bind:page></Settings>
    {/if}
    {#if page === "Restore"}
        <Restore bind:page bind:restoreId></Restore>
    {/if}
</main>

<style>
    main {
        margin: 40px;
    }

    :global(body) {
        @font-face {
            font-family: 'Kumbh Sans';
            src: url('/src/fonts/KumbhSans.ttf');
        }

        h1 {
            font-family: "Kumbh Sans", sans-serif;
            font-size: 2.5rem;
            font-weight: bolder;
        }
        h2 {
            font-family: "Kumbh Sans", sans-serif;
            font-size: 1.75rem;
            font-weight: bold;
        }
    }
</style>
