<script>
    import {Button} from "$lib/components/ui/button/index.js";
    import {Home} from "@lucide/svelte";

    let {page = $bindable()} = $props();
</script>

<h2 class="mb-5">Settings</h2>
<Button onclick={() => page = "Dashboard"} class="fixed top-10 right-10 z-50"><Home/> Back to Dashboard</Button>