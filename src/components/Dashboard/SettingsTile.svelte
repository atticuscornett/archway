<script lang="ts">
    import * as Card from "$lib/components/ui/card/index.js";
    import {Button} from "$lib/components/ui/button/index.js";
    import {ArrowRight} from "@lucide/svelte";
    import {onMount} from "svelte";
    import {toast} from "svelte-sonner";

    let {page = $bindable()} = $props();

</script>

<Card.Root class="w-full h-full mb-4 relative pb-8">
    <Card.Header>
        <Card.Title>Settings</Card.Title>
        <Card.Description>Modify app settings and get version info</Card.Description>
    </Card.Header>
    <Card.Content>
    </Card.Content>
    <Button class="align-middle absolute bottom-4 right-4" onclick={()=>{page = "Settings";}}><ArrowRight/></Button>
</Card.Root>