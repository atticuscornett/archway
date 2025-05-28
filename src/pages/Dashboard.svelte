<script>
    import {onMount} from "svelte";
    import JobListTile from "../components/Dashboard/JobListTile.svelte";
    import ProgressTile from "../components/Dashboard/ProgressTile.svelte";

    let {page = $bindable()} = $props();

    onMount(
        () => {
            // This is a placeholder for any setup logic you might need
            console.log("Dashboard mounted");
        }
    );
</script>

<img src="src/img/ArchwayFull.svg" class="w-52 mb-4" alt="Archway Logo" />
<JobListTile bind:page/>
<ProgressTile bind:page/>
<JobListTile bind:page/>


<style>
    .flex-2 {
        flex: 2;
    }
</style>
