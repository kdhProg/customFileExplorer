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
    <p class="description">내 파일 관리하기</p>
    <!-- 폴더 아이콘 밑에 문구 -->
</div>
