<script>
    import {onMount} from "svelte";
    import JobListTile from "../components/Dashboard/JobListTile.svelte";
    import StatusTile from "../components/Dashboard/StatusTile.svelte";
    import HealthTile from "../components/Dashboard/HealthTile.svelte";

    let {page = $bindable()} = $props();
</script>

<img src="src/img/ArchwayFull.svg" class="w-52 mb-4" alt="Archway Logo" />
<HealthTile bind:page/>
<JobListTile bind:page/>
<StatusTile bind:page/>