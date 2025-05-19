<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import {Calendar} from "$lib/components/ui/calendar";
    import { toast } from "svelte-sonner";
    import QuickSetupPage from "../pages/QuickSetupPage.svelte";

    let name = $state("");
    let greetMsg = $state("");
    window.invoke = invoke;

    async function greet(event: Event) {
        event.preventDefault();
        // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
        greetMsg = await invoke("greet", { name });
        toast.loading("Goofy..", {
            id: "loading",
        });
        setTimeout(() => {
            toast.loading("Goofy ahh");
        }, 10000)
    }
</script>

<main >
    <QuickSetupPage></QuickSetupPage>

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
