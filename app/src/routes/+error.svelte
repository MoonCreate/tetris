<script lang="ts">
    import { page } from "$app/stores";

    const getStatusCode = (s: number) => {
        switch (s) {
            case 403: return "https://raw.githubusercontent.com/SAWARATSUKI/KawaiiLogos/refs/heads/main/ResponseCode/403%20Forbidden.png";
            case 404: return "https://raw.githubusercontent.com/SAWARATSUKI/KawaiiLogos/refs/heads/main/ResponseCode/404%20NotFound.png";
            case 418: return "https://raw.githubusercontent.com/SAWARATSUKI/KawaiiLogos/refs/heads/main/ResponseCode/418%20I'm%20a%20teapot.png";
            case 500: return "https://raw.githubusercontent.com/SAWARATSUKI/KawaiiLogos/refs/heads/main/ResponseCode/500%20InternalServerError.png";
            case 503: return "https://raw.githubusercontent.com/SAWARATSUKI/KawaiiLogos/refs/heads/main/ResponseCode/503%20ServiceUnavailable.png";
            default: return "";
        }
    }

    console.log($page.error?.message);
</script>

<div class="error">
    <img src={getStatusCode($page.status)} alt="status code {$page.status}" />
</div>

<style>
    .error {
        width: 100%;
        height: 100%;
        display: flex;
        flex-direction: column;
        img {
            object-fit: cover;
        }
    }
</style>