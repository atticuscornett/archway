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

    let name = $state("");
    let greetMsg = $state("");
    window.invoke = invoke;
    let page = $state("WelcomePage");
    let pageData = $state("");

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
        <JobManager bind:page></JobManager>
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

<!--    <div class="row">-->
<!--        <a href="https://vite.dev" target="_blank">-->
<!--            <img src="/vite.svg" class="logo vite" alt="Vite Logo" />-->
<!--        </a>-->
<!--        <a href="https://tauri.app" target="_blank">-->
<!--            <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />-->
<!--        </a>-->
<!--        <a href="https://svelte.dev" target="_blank">-->
<!--            <img src="/svelte.svg" class="logo svelte-kit" alt="SvelteKit Logo" />-->
<!--        </a>-->
<!--    </div>-->
<!--    <p>Click on the Tauri, Vite, and SvelteKit logos to learn more.</p>-->

<!--    <form class="row" onsubmit={greet}>-->
<!--        <input id="greet-input" placeholder="Enter a name..." bind:value={name} />-->
<!--        <button type="submit">Greet</button>-->
<!--    </form>-->
<!--    <p>{greetMsg}</p>-->
<!--    <h2>Hellow</h2>-->
<!--    <Calendar type="single"/>-->
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
