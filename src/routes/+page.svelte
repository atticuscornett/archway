<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import {Calendar} from "$lib/components/ui/calendar";
  import { toast } from "svelte-sonner";

  let name = $state("");
  let greetMsg = $state("");

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
  <img src="src/img/ArchwayFull.svg" alt="Archway Logo" class="logo" />
  <h1>Welcome to Archway</h1>
  <h2>Let's get your first automation set up.</h2>

  <div class="row">
    <a href="https://vite.dev" target="_blank">
      <img src="/vite.svg" class="logo vite" alt="Vite Logo" />
    </a>
    <a href="https://tauri.app" target="_blank">
      <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
    </a>
    <a href="https://svelte.dev" target="_blank">
      <img src="/svelte.svg" class="logo svelte-kit" alt="SvelteKit Logo" />
    </a>
  </div>
  <p>Click on the Tauri, Vite, and SvelteKit logos to learn more.</p>

  <form class="row" onsubmit={greet}>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button type="submit">Greet</button>
  </form>
  <p>{greetMsg}</p>
  <h2>Hellow</h2>
  <Calendar type="single"/>
</main>

<style>
  .logo {
    width: 300px;
  }

  main {
    margin: 40px;
  }

  :root {
    h1 {

    }
  }
</style>
