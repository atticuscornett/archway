<script type="ts">
    import {Button} from "$lib/components/ui/button/index.js";
    import {Files, FileSliders} from '@lucide/svelte';
    import * as Select from "$lib/components/ui/select/index.js";
    import SetUpAutomation from "./SetUpAutomation.svelte";
    import {onMount} from "svelte";

    let { page = $bindable() } = $props();

    let job = $state({
        type: "copy"
    });

    onMount(async ()=> {
       let jobs = await invoke("get_all_jobs");
       jobs = JSON.parse(jobs);
       if (jobs.length > 0){
           page = "Dashboard";
       }
    });
</script>

<div class="center-screen">
    <img src="src/img/ArchwayFull.svg" alt="Archway Logo" class="logo" />
    <h1>Welcome to Archway!</h1>
    <h2>Let's get your first automation set up.</h2>
    <div class="small-margin">
        <Button onclick={()=>{page = "SetUpAutomation"}}>Quick Setup</Button>
        <Button variant="secondary" onclick={()=>{page = "Dashboard"}}>No, thanks. I'll set up later.</Button>
    </div>
</div>

<style>
    .logo {
        width: 300px;
    }

    .large-button {
        outline: white solid 7px;
        padding: 15px;
        border-radius: 7px;
        width: 350px;
        height: 450px;
        margin-right: 40px;
    }

    .large-button-container {
        display: flex;
    }

    .large-button > img {
        width: 200px;
        margin-left: auto;
        margin-right: auto;
    }

    .large-button:focus-visible {
        outline: dodgerblue solid 7px;
    }

    .small-margin {
        margin-top: 20px;
    }

    .center-screen {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        height: calc(100vh - 100px);
    }

    h1 {
        margin-bottom: 15px;
    }
</style>