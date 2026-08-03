document.addEventListener("DOMContentLoaded", () => {
    const findSelect = document.getElementById("find-next");
    const trackerContainer = document.querySelector(".tracker-container");

    if (!findSelect || !trackerContainer) return;

    function highlightSelectedUnit(unitName) {
        trackerContainer.querySelectorAll(".highlight-find").forEach((cell) => {
            cell.classList.remove("highlight-find");
        });

        if (!unitName || unitName.trim() === "") return;

        const pickCells = trackerContainer.querySelectorAll("td.pick");

        pickCells.forEach((cell) => {
            const mainLink = cell.querySelector("a");

            if (mainLink) {
                const cellText = mainLink.textContent.trim();

                if (cellText === unitName) {
                    cell.classList.add("highlight-find");
                }
            }
        });
    }

    highlightSelectedUnit(findSelect.value);

    findSelect.addEventListener("change", (e) => {
        highlightSelectedUnit(e.target.value);
        updateQueryParamInLinks("find", e.target.value);
    });

    function updateQueryParamInLinks(param, value) {
        const links = trackerContainer.querySelectorAll("a");
        links.forEach((a) => {
            try {
                const url = new URL(a.href);
                if (value) {
                    url.searchParams.set(param, value);
                } else {
                    url.searchParams.delete(param);
                }
                a.href = url.toString();
            } catch (err) {}
        });
    }
});
