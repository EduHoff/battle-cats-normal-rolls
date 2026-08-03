const SEED_STORAGE_KEY = "bcnr_last_seed_url";

document.addEventListener("DOMContentLoaded", () => {
    const currentPath = window.location.pathname;
    const currentSearch = window.location.search;
    const savedUrl = localStorage.getItem(SEED_STORAGE_KEY);

    if (currentPath === "/" || currentPath === "/index.html") {
        if (currentSearch.length > 0) {
            const fullUrl = currentPath + currentSearch;
            localStorage.setItem(SEED_STORAGE_KEY, fullUrl);
        } else if (savedUrl) {
            window.location.href = savedUrl;
        }
    }
});
