<script>
    import "/src/lib/style/splashScreen.css";
    import { goto } from "$app/navigation";

    function goToMainPage() {
        goto("/frontTest/frame/mainPage");
    }
</script>

<div class="splash-screen">
    <div class="splash-icon" on:click={goToMainPage}>
        <img src="/icons/mainLogo.png" alt="mainLogo" class="splash-icon-img">
    </div>
    <p class="description" style="font-size: 30px;">PathFinder</p>
    <p class="description">Simple File explorer</p>
</div>
