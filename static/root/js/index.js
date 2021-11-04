const urlInput = document.getElementById("urlInput");
const submitButton = document.getElementById("submitButton");
const copyButton = document.getElementById("copyButton");
const alertDiv = document.getElementById("alertDiv")

submitButton.addEventListener("click", async() => {
    if (urlInput.readOnly) {
        urlInput.value = "";
        urlInput.readOnly = false;
        submitButton.textContent = "Shorten";

        copyButton.hidden = true;
    } else {
        let url = urlInput.value;
        if (!(/^https?:\/\//i).test(url)) {
            url = "http://" + url
        }

        try {
            new URL(url);
        } catch {
            new bootstrap.Collapse(alertDiv)
            return;
        }

        let res = await fetch("/api/url/", {
            method: "POST",
            body: JSON.stringify({ url: url }),
            headers: {
                "Content-Type": "application/json"
            }
        });
        if (!res.ok) {
            new bootstrap.Collapse(alertDiv)
            return;
        }
        let json = await res.json()
        urlInput.readOnly = true;
        urlInput.value = "https://axo.sh/" + json.id;
        submitButton.textContent = "Another one?";
        copyButton.hidden = false;
    }

}, { passive: true });

copyButton.addEventListener("click", () => {
    navigator.clipboard.writeText(urlInput.value);
})