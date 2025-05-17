<script type="ts">
import {Button} from "$lib/components/ui/button/index.js";

    let page = $state("welcome")
</script>

<div class="center-screen">
    {#if page === "welcome"}
        <img src="src/img/ArchwayFull.svg" alt="Archway Logo" class="logo" />
        <h1>Welcome to Archway!</h1>
        <h2>Let's get your first automation set up.</h2>
        <div class="small-margin">
            <Button onclick={()=>{page = "copy-or-move"}}>Quick Setup</Button>
            <Button variant="secondary">No, thanks. I'll set up manually.</Button>
        </div>
    {/if}
    {#if page === "copy-or-move"}
        <h1>Copy or move files?</h1>
        <div class="large-button-container">
            <button class="large-button">
                <img src="src/img/icons/copy.svg" alt="Copy Icon" />
                <h2>Copy/Backup</h2>
                <span>
                    Copy files from the source to the destination.
                </span>
            </button>

            <button class="large-button">
                <img src="src/img/icons/move.svg" alt="Copy Icon" />
                <h2>Move/Archive</h2>
                <span>
                    Move files to the destination and delete them from the source.
                </span>
            </button>
        </div>
    {/if}
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