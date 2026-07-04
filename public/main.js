window.addEventListener("load", () => {
    fetch("/api/verify-login")
        .then(response => response.json())
        .then(json => {
            if (json.status === "success") {
                const loggedInMsgEl = document.querySelector("#logged-in-msg")
                loggedInMsgEl.style.display = "block"
            } else {
                const loggedOutMsgEl = document.querySelector("#logged-out-msg")
                loggedOutMsgEl.style.display = "block"
            }
        })
})
