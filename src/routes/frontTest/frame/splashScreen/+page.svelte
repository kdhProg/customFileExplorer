<script>
    import "/src/routes/frontTest/style/splashScreen.css";
    import { goto } from "$app/navigation";

    function goToMainPage() {
        goto("/frontTest/frame/mainPage");
    }
</script>

<div class="splash-screen">
    <div class="file-icon" on:click={goToMainPage}></div>
    <p class="description">내 파일 관리하기</p>
    <!-- 폴더 아이콘 밑에 문구 -->
</div>
